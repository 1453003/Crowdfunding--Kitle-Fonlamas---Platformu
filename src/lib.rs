#![no_std] 

use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env
};


#[derive(Clone)]
#[contracttype]
pub enum Data {
    Project, 
}


#[derive(Clone)]
#[contracttype]
pub struct Project {
    pub creator: Address,         // Projeyi başlatan kişinin adresi
    pub funding_token: Address,   // Fonlama için kullanılan token'ın adresi
    pub funding_goal: i128,        // Proje için belirlenen hedef fonlama miktarı
    pub raised_amount: i128,       // Şu ana kadar toplanan fon miktarı
    pub end_time: u64,             // Fon toplama süresinin bitiş zamanı
    pub is_funded: bool,           // Projenin fonlanıp fonlanmadığını belirtir
}


#[contract]
pub struct Crowdfunding;

#[contractimpl]
impl Crowdfunding {
    // Yeni bir proje oluşturma fonksiyonu
    pub fn create_project(
        e: Env,
        creator: Address,
        funding_token: Address,
        funding_goal: i128,
        duration: u64,
        current_time: u64,
    ) {
        // Eğer proje zaten varsa, hata fırlatır
        if e.storage().instance().has(&Data::Project) {
            panic!("Project already exists");
        }

        let end_time = current_time + duration; // Projenin bitiş zamanını hesaplar

        // Proje verilerini depolamak için storage'a yazar
        e.storage().instance().set(
            &Data::Project,
            &Project {
                creator,
                funding_token,
                funding_goal,
                raised_amount: 0,
                end_time,
                is_funded: false,
            },
        );
    }

    // Projeyi fonlama fonksiyonu
    pub fn fund_project(e: Env, amount: i128, current_time: u64) {
        let mut project = load_project(&e); // Depolanan projeyi yükler

        // Fonlama süresi geçmişse, hata fırlatır
        if current_time > project.end_time {
            panic!("Project funding period has ended");
        }

        let funding_client = token::Client::new(&e, &project.funding_token); // Token client'ını oluşturur
        let _contract = e.current_contract_address(); // Kontratı tanımlar

        // Fonları projeye transfer eder
        funding_client.transfer(&e.current_contract_address(), &project.creator, &amount);

        project.raised_amount += amount; 
        if project.raised_amount >= project.funding_goal {
            project.is_funded = true; 
        }

        // Güncellenmiş proje verilerini depolar
        e.storage().instance().set(&Data::Project, &project);
    }

    // Toplanan fonları çekme fonksiyonu
    pub fn withdraw_funds(e: Env) {
        let project = load_project(&e); 

        // Proje fonlanmamışsa, hata fırlatır
        if !project.is_funded {
            panic!("Project is not funded yet");
        }

        project.creator.require_auth(); // Projeyi başlatan kişinin yetkisini kontrol eder
        let funding_client = token::Client::new(&e, &project.funding_token); 

        // Fonları proje yaratıcısına transfer eder
        funding_client.transfer(&e.current_contract_address(), &project.creator, &project.raised_amount);
    }

    // Fonlama süresi sonunda geri ödeme fonksiyonu
    pub fn refund(e: Env, current_time: u64) {
        let project = load_project(&e); 

        // Fonlama süresi bitmemişse, hata fırlatır
        if current_time <= project.end_time {
            panic!("Project funding period has not ended yet");
        }

        // Proje fonlanmışsa, geri ödeme yapılmaz
        if project.is_funded {
            panic!("Project is funded, no refunds");
        }

        let refund_amount = project.raised_amount; // Geri ödenecek miktarı belirler
        let funding_client = token::Client::new(&e, &project.funding_token); 
        
        // Fonları geri öder
        funding_client.transfer(&e.current_contract_address(), &project.creator, &refund_amount);

        // Güncellenmiş proje verilerini depolar
        e.storage().instance().set(
            &Data::Project,
            &Project {
                raised_amount: 0,
                ..project
            },
        );
    }

    // Proje verilerini alma fonksiyonu
    pub fn get_project(e: Env) -> Project {
        load_project(&e) 
    }
}

// Projeyi yükleyen yardımcı fonksiyon
fn load_project(e: &Env) -> Project {
    e.storage().instance().get(&Data::Project).unwrap() 
}
