# Paradise Idle Master

<div align="center">

![Paradise Idle Master](https://img.shields.io/badge/version-1.0.1-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Platform](https://img.shields.io/badge/platform-Windows-lightgrey.svg)

**A modern Steam card idle application built with Tauri and Svelte**

[English](#english) | [Türkçe](#türkçe)

</div>

---

## English

### 🎮 What is Paradise Idle Master?

Paradise Idle Master is a lightweight, modern desktop application that helps you idle Steam games to collect trading cards efficiently. Built with cutting-edge technologies like Tauri and Svelte, it offers a sleek interface and powerful features.

### ✨ Features

- 🎯 **Easy Game Management** - Search and add games by name or App ID
- 🚀 **Multi-Game Idling** - Run multiple games simultaneously
- ⏱️ **Time Tracking** - Monitor idle time for each game
- 💾 **Persistent Storage** - Your game list is automatically saved
- 🌐 **Multi-Language** - Switch between English and Turkish
- 🎨 **Modern UI** - Clean, dark-themed interface
- 📊 **Statistics Panel** - Track total games, active sessions, and total time
- ⭐ **Favorites System** - Save and load your favorite game lists
- 🔍 **Quick Search** - Filter your game list instantly
- 🖥️ **System Tray** - Minimize to tray and run in background

### 📋 Requirements

- Windows 10/11 (64-bit)
- Steam installed and running
- Active Steam account

### 🚀 Installation

1. Download the latest installer from the [Releases](https://github.com/muhammetdag/paradise-idle-master/releases) page
2. Run `Paradise-Idle-Master_1.0.1.msi`
3. Follow the installation wizard
4. Launch Paradise Idle Master from Start Menu or Desktop
5. Start idling!

### 🛠️ Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/muhammetdag/paradise-idle-master.git
   cd paradise-idle-master
   ```
2. Install frontend dependencies:
   ```bash
   npm install
   ```
3. Place `steam_api64.dll` (from official Steamworks SDK or your Steam folder) into `src-tauri/`.
4. Run in development mode:
   ```bash
   npm run tauri dev
   ```
5. Build production installer:
   ```bash
   npm run tauri build
   ```

### 💡 How to Use

#### Adding Games

**Method 1: Search by Name**
- Type a game name in the search bar
- Select from the dropdown results
- Game will be added to your list

**Method 2: Direct App ID**
- Enter a Steam App ID (e.g., 730 for CS:GO)
- Press Enter
- Game will be automatically added

#### Managing Games

- **Start Single Game**: Click the green "Start" button on any game
- **Stop Single Game**: Click the red "Stop" button on running games
- **Start All**: Use the "Start All" button at the bottom
- **Stop All**: Use the "Stop All" button at the bottom
- **Remove Game**: Click the X button on any game card
- **Remove All**: Click "Remove All" in the games section header

#### Favorites System

- **Save Current List**: Click the ⭐ star icon to save your current game list as favorites
- **Load Favorites**: Click the 📥 download icon to restore your saved favorites
- Your favorites are stored locally and persist between sessions

#### Language Settings

- Click the language button (EN/TR) in the top-right corner to switch languages
- Your language preference is saved automatically

#### System Tray

- Close the window to minimize to system tray
- Left-click the tray icon to restore the window
- Right-click the tray icon to access the Exit menu

### 📁 Data Storage

All your data is stored locally in:
```
%APPDATA%\paradise_idle_master\
├── favorites.bin    # Your saved games
└── language.txt     # Language preference
```

### ❓ FAQ

**Q: Is this safe to use?**
A: Yes, Paradise Idle Master uses the official Steam API and doesn't modify any game files.

**Q: Will I get VAC banned?**
A: No, idling games for cards is allowed by Steam's terms of service.

**Q: Can I idle multiple games at once?**
A: Yes, you can idle as many games as you want simultaneously.

**Q: Where can I find a game's App ID?**
A: You can find it in the Steam store URL. For example: `store.steampowered.com/app/730/` - the App ID is 730.

**Q: The application won't start. What should I do?**
A: Make sure Steam is running and you're logged in. Also check that you have the latest Windows updates.

### 🔗 Links

- **Website**: [paradisedev.org](https://paradisedev.org)
- **Discord**: [discord.gg/paradisedev](https://discord.gg/paradisedev)
- **Report Issues**: [GitHub Issues](https://github.com/muhammetdag/paradise-idle-master/issues)

### ⚠️ Disclaimer

This application is not affiliated with or endorsed by Valve Corporation or Steam. Use at your own risk. Paradise Development is not responsible for any issues that may arise from using this software.

### 📝 License

This project is licensed under the MIT License.

---

## Türkçe

### 🎮 Paradise Idle Master Nedir?

Paradise Idle Master, Steam oyunlarını idle ederek kart toplamak için tasarlanmış hafif ve modern bir masaüstü uygulamasıdır. Tauri ve Svelte gibi son teknoloji araçlarla geliştirilmiş olup, şık bir arayüz ve güçlü özellikler sunar.

### ✨ Özellikler

- 🎯 **Kolay Oyun Yönetimi** - İsim veya App ID ile oyun arayın ve ekleyin
- 🚀 **Çoklu Oyun İdle** - Aynı anda birden fazla oyunu çalıştırın
- ⏱️ **Süre Takibi** - Her oyun için idle süresini izleyin
- 💾 **Kalıcı Depolama** - Oyun listeniz otomatik olarak kaydedilir
- 🌐 **Çoklu Dil** - İngilizce ve Türkçe arasında geçiş yapın
- 🎨 **Modern Arayüz** - Temiz, karanlık temalı tasarım
- 📊 **İstatistik Paneli** - Toplam oyun, aktif oturum ve toplam süreyi takip edin
- ⭐ **Favori Sistemi** - Favori oyun listelerinizi kaydedin ve yükleyin
- 🔍 **Hızlı Arama** - Oyun listenizde anında filtreleme yapın
- 🖥️ **Sistem Tepsisi** - Tepsiye küçült ve arka planda çalıştır

### 📋 Gereksinimler

- Windows 10/11 (64-bit)
- Steam yüklü ve çalışır durumda
- Aktif Steam hesabı

### 🚀 Kurulum

1. [Releases](https://github.com/muhammetdag/paradise-idle-master/releases) sayfasından son kurulum dosyasını indirin
2. `Paradise-Idle-Master_1.0.1.msi` dosyasını çalıştırın
3. Kurulum sihirbazını takip edin
4. Başlat Menüsü veya Masaüstü'nden Paradise Idle Master'ı başlatın
5. İdle etmeye başlayın!

### 🛠️ Kaynak Koddan Derleme

1. Depoyu klonlayın:
   ```bash
   git clone https://github.com/muhammetdag/paradise-idle-master.git
   cd paradise-idle-master
   ```
2. Arayüz bağımlılıklarını yükleyin:
   ```bash
   npm install
   ```
3. Steamworks SDK veya Steam dizininizden edineceğiniz `steam_api64.dll` dosyasını `src-tauri/` klasörüne kopyalayın.
4. Geliştirici modunda çalıştırın:
   ```bash
   npm run tauri dev
   ```
5. Üretim sürümünü paketleyin:
   ```bash
   npm run tauri build
   ```

### 💡 Nasıl Kullanılır

#### Oyun Ekleme

**Yöntem 1: İsimle Arama**
- Arama çubuğuna oyun adını yazın
- Açılan listeden seçim yapın
- Oyun listenize eklenecektir

**Yöntem 2: Doğrudan App ID**
- Steam App ID'sini girin (örn: CS:GO için 730)
- Enter'a basın
- Oyun otomatik olarak eklenecektir

#### Oyun Yönetimi

- **Tek Oyun Başlat**: Herhangi bir oyundaki yeşil "Başlat" butonuna tıklayın
- **Tek Oyun Durdur**: Çalışan oyunlardaki kırmızı "Durdur" butonuna tıklayın
- **Tümünü Başlat**: Alttaki "Tümünü Başlat" butonunu kullanın
- **Tümünü Durdur**: Alttaki "Tümünü Durdur" butonunu kullanın
- **Oyun Kaldır**: Herhangi bir oyun kartındaki X butonuna tıklayın
- **Tümünü Kaldır**: Oyunlar bölümü başlığındaki "Tümünü Kaldır" butonuna tıklayın

#### Favori Sistemi

- **Mevcut Listeyi Kaydet**: Mevcut oyun listenizi favori olarak kaydetmek için ⭐ yıldız ikonuna tıklayın
- **Favorileri Yükle**: Kayıtlı favorilerinizi geri yüklemek için 📥 indirme ikonuna tıklayın
- Favorileriniz yerel olarak saklanır ve oturumlar arasında kalıcıdır

#### Dil Ayarları

- Dil değiştirmek için sağ üst köşedeki dil butonuna (EN/TR) tıklayın
- Dil tercihiniz otomatik olarak kaydedilir

#### Sistem Tepsisi

- Pencereyi kapatarak sistem tepsisine küçültün
- Tepsi ikonuna sol tıklayarak pencereyi geri getirin
- Tepsi ikonuna sağ tıklayarak Çıkış menüsüne erişin

### 📁 Veri Depolama

Tüm verileriniz yerel olarak şurada saklanır:
```
%APPDATA%\paradise_idle_master\
├── favorites.bin    # Kayıtlı oyunlarınız
└── language.txt     # Dil tercihiniz
```

### ❓ Sık Sorulan Sorular

**S: Kullanımı güvenli mi?**
C: Evet, Paradise Idle Master resmi Steam API'sini kullanır ve hiçbir oyun dosyasını değiştirmez.

**S: VAC ban yer miyim?**
C: Hayır, kart için oyun idle etmek Steam'in kullanım şartları tarafından izin verilmektedir.

**S: Aynı anda birden fazla oyunu idle edebilir miyim?**
C: Evet, istediğiniz kadar oyunu aynı anda idle edebilirsiniz.

**S: Bir oyunun App ID'sini nereden bulabilirim?**
C: Steam mağaza URL'sinde bulabilirsiniz. Örneğin: `store.steampowered.com/app/730/` - App ID 730'dur.

**S: Uygulama başlamıyor. Ne yapmalıyım?**
C: Steam'in çalıştığından ve giriş yaptığınızdan emin olun. Ayrıca en son Windows güncellemelerine sahip olduğunuzu kontrol edin.

### 🔗 Bağlantılar

- **Website**: [paradisedev.org](https://paradisedev.org)
- **Discord**: [discord.gg/paradisedev](https://discord.gg/paradisedev)
- **Sorun Bildir**: [GitHub Issues](https://github.com/muhammetdag/paradise-idle-master/issues)

### ⚠️ Sorumluluk Reddi

Bu uygulama Valve Corporation veya Steam ile bağlantılı değildir ve onlar tarafından onaylanmamıştır. Kullanım riski size aittir. Paradise Development, bu yazılımın kullanımından kaynaklanabilecek herhangi bir sorundan sorumlu değildir.

### 📝 Lisans

Bu proje MIT Lisansı altında lisanslanmıştır.

---

<div align="center">

**Made with ❤️ by Paradise Development**

Copyright © 2026 Paradise Development

</div>
