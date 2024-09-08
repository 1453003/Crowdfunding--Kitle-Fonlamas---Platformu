

---

# Crowdfunding Akıllı Sözleşmesi

## Genel Bakış

Bu proje, Stellar blok zinciri üzerinde Soroban SDK kullanarak geliştirilmiş bir Crowdfunding (Fonlama) platformunun akıllı sözleşmesini içerir. Akıllı sözleşme, kullanıcıların projeler oluşturmasını, bu projelere fon sağlamasını, başarılı projelerden fon çekmesini ve projeler başarısız olursa geri ödeme talep etmesini sağlar.

## Özellikler

- **Proje Oluşturma:** Kullanıcıların yeni bir fonlama projesi oluşturmasına olanak tanır.
- **Projeyi Fonlama:** Kullanıcılar mevcut projelere fon katkısında bulunabilir.
- **Fonları Çekme:** Proje sahibi, proje hedeflerine ulaşıldığında toplanan fonları çekebilir.
- **Geri Ödeme:** Fonlama süresi sona erdiğinde ve proje hedeflerine ulaşılamadığında geri ödeme yapılabilir.
- **Proje Bilgileri:** Projelerin mevcut durumunu görüntüleme imkanı sağlar.

## Başlarken

Bu akıllı sözleşmeyi kullanmak için aşağıdaki adımları izleyin:

### Gereksinimler

- [Soroban SDK](https://docs.soroban.io/) kurulumu gereklidir.
- [Stellar CLI](https://www.stellar.org/developers/tools/cli/) yapılandırılmalıdır.
- Stellar blok zincirinde akıllı sözleşmelerin nasıl dağıtılacağı ve etkileşimde bulunulacağı hakkında temel bilgi.

### Sözleşme Yapısı

- **Data Enum:** Proje verilerinin saklanması için kullanılan anahtarları tanımlar.
- **Project Yapısı:** Projenin sahibi, fonlama token'ı, fonlama hedefi, toplanan miktar, fonlama süresi ve fonlanma durumu gibi bilgileri içerir.
- **Crowdfunding Sözleşmesi:** Proje oluşturma, fonlama, fon çekme, geri ödeme ve proje bilgilerini alma işlevlerini içerir.

### Fonksiyonlar

1. **Proje Oluşturma (`create_project`):** Yeni bir fonlama projesi oluşturur.
   ```rust
   pub fn create_project(
       e: Env,
       creator: Address,
       funding_token: Address,
       funding_goal: i128,
       duration: u64,
       current_time: u64,
   )
   ```

2. **Projeyi Fonlama (`fund_project`):** Mevcut bir projeye fon katkısında bulunur.
   ```rust
   pub fn fund_project(e: Env, amount: i128, current_time: u64)
   ```

3. **Fonları Çekme (`withdraw_funds`):** Başarılı bir projeden toplanan fonları çeker.
   ```rust
   pub fn withdraw_funds(e: Env)
   ```

4. **Geri Ödeme (`refund`):** Fonlama süresi sonunda ve proje hedeflerine ulaşılamadığında geri ödeme yapar.
   ```rust
   pub fn refund(e: Env, current_time: u64)
   ```

5. **Projeyi Getir (`get_project`):** Projenin mevcut durumunu getirir.
   ```rust
   pub fn get_project(e: Env) -> Project
   ```

---
