Crowdfunding Akıllı Sözleşmesi
Genel Bakış
Bu proje, Stellar blok zinciri üzerinde Soroban SDK kullanarak geliştirilmiş bir Crowdfunding (Fonlama) platformunun akıllı sözleşmesini içerir. Akıllı sözleşme, kullanıcıların projeler oluşturmasını, bu projelere fon sağlamasını, başarılı projelerden fon çekmesini ve projeler başarısız olursa geri ödeme talep etmesini sağlar.

Özellikler
Proje Oluşturma: Kullanıcıların yeni bir fonlama projesi oluşturmasına olanak tanır.
Projeyi Fonlama: Kullanıcılar mevcut projelere fon katkısında bulunabilir.
Fonları Çekme: Proje sahibi, proje hedeflerine ulaşıldığında toplanan fonları çekebilir.
Geri Ödeme: Fonlama süresi sona erdiğinde ve proje hedeflerine ulaşılamadığında geri ödeme yapılabilir.
Proje Bilgileri: Projelerin mevcut durumunu görüntüleme imkanı sağlar.
Başlarken
Bu akıllı sözleşmeyi kullanmak için aşağıdaki adımları izleyin:

Gereksinimler
Soroban SDK kurulumu gereklidir.
Stellar CLI yapılandırılmalıdır.
Stellar blok zincirinde akıllı sözleşmelerin nasıl dağıtılacağı ve etkileşimde bulunulacağı hakkında temel bilgi.
Sözleşme Yapısı
Data Enum: Proje verilerinin saklanması için kullanılan anahtarları tanımlar.
Project Yapısı: Projenin sahibi, fonlama token'ı, fonlama hedefi, toplanan miktar, fonlama süresi ve fonlanma durumu gibi bilgileri içerir.
Crowdfunding Sözleşmesi: Proje oluşturma, fonlama, fon çekme, geri ödeme ve proje bilgilerini alma işlevlerini içerir.
Fonksiyonlar
Proje Oluşturma (create_project): Yeni bir fonlama projesi oluşturur.

rust
Kodu kopyala
pub fn create_project(
    e: Env,
    creator: Address,
    funding_token: Address,
    funding_goal: i128,
    duration: u64,
    current_time: u64,
)
Projeyi Fonlama (fund_project): Mevcut bir projeye fon katkısında bulunur.

rust
Kodu kopyala
pub fn fund_project(e: Env, amount: i128, current_time: u64)
Fonları Çekme (withdraw_funds): Başarılı bir projeden toplanan fonları çeker.

rust
Kodu kopyala
pub fn withdraw_funds(e: Env)
Geri Ödeme (refund): Fonlama süresi sonunda ve proje hedeflerine ulaşılamadığında geri ödeme yapar.

rust
Kodu kopyala
pub fn refund(e: Env, current_time: u64)
Projeyi Getir (get_project): Projenin mevcut durumunu getirir.

rust
Kodu kopyala
pub fn get_project(e: Env) -> Project
