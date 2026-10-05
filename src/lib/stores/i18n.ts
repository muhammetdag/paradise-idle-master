import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

type Language = 'en' | 'tr';

interface Translations {
  en: Record<string, string>;
  tr: Record<string, string>;
}

const translations: Translations = {
  en: {
    // Header
    'app.title': 'Paradise Idle Master',
    'app.copyright': 'Copyright © 2026 Paradise Development',
    
    // Stats Panel
    'stats.total': 'Total',
    'stats.active': 'Active',
    'stats.stopped': 'Stopped',
    'stats.totalTime': 'Total Time',
    'stats.hours': 'h',
    'stats.minutes': 'm',
    
    // Search
    'search.placeholder': 'Search game or enter App ID...',
    
    // Games Section
    'games.title': 'Games',
    'games.searchList': 'Search in list...',
    'games.loadFavorites': 'Load Favorites',
    'games.saveFavorites': 'Save as Favorites',
    'games.removeAll': 'Remove All',
    'games.empty': 'No games',
    'games.noResults': 'No results found',
    
    // Game Item
    'game.remove': 'Remove',
    'game.status.active': 'Active',
    'game.status.ready': 'Ready',
    'game.start': 'Start',
    'game.stop': 'Stop',
    
    // Controls
    'controls.startAll': 'Start All',
    'controls.stopAll': 'Stop All',
    
    // Modal
    'modal.removeAll.title': 'Remove All',
    'modal.removeAll.message': 'Are you sure you want to remove all games and stop all idlers?',
    'modal.savePreset.title': 'Success',
    'modal.savePreset.message': 'Your current game list has been saved as favorite group!',
    'modal.loadPreset.error': 'Error',
    'modal.loadPreset.errorMessage': 'No saved favorite list found.',
    'modal.gameStart.error': 'Error',
    'modal.gameStart.errorMessage': 'Failed to start game',
    'modal.appId.error': 'Error',
    'modal.appId.errorMessage': 'No game found with this App ID or Steam API error occurred.',
    'modal.confirm': 'Confirm',
    'modal.cancel': 'Cancel',
    'modal.ok': 'OK',
    
    // Rust messages
    'rust.search.turkish': 'english',
  },
  tr: {
    // Header
    'app.title': 'Paradise Idle Master',
    'app.copyright': 'Copyright  © 2026 Paradise Development',
    
    // Stats Panel
    'stats.total': 'Toplam',
    'stats.active': 'Aktif',
    'stats.stopped': 'Durmuş',
    'stats.totalTime': 'Toplam Süre',
    'stats.hours': 's',
    'stats.minutes': 'dk',
    
    // Search
    'search.placeholder': 'Oyun ara veya App ID gir...',
    
    // Games Section
    'games.title': 'Oyunlar',
    'games.searchList': 'Listede ara...',
    'games.loadFavorites': 'Favorileri Yükle',
    'games.saveFavorites': 'Favori Liste Olarak Kaydet',
    'games.removeAll': 'Tümünü Kaldır',
    'games.empty': 'Oyun yok',
    'games.noResults': 'Sonuç bulunamadı',
    
    // Game Item
    'game.remove': 'Kaldır',
    'game.status.active': 'Aktif',
    'game.status.ready': 'Hazır',
    'game.start': 'Başlat',
    'game.stop': 'Durdur',
    
    // Controls
    'controls.startAll': 'Tümünü Başlat',
    'controls.stopAll': 'Tümünü Durdur',
    
    // Modal
    'modal.removeAll.title': 'Tümünü Kaldır',
    'modal.removeAll.message': 'Listedeki tüm oyunları kaldırmak ve idler\'ları durdurmak istediğinize emin misiniz?',
    'modal.savePreset.title': 'Başarılı',
    'modal.savePreset.message': 'Şu anki oyun listeniz favori grup olarak kaydedildi!',
    'modal.loadPreset.error': 'Hata',
    'modal.loadPreset.errorMessage': 'Kayıtlı favori liste bulunamadı.',
    'modal.gameStart.error': 'Hata',
    'modal.gameStart.errorMessage': 'Oyun başlatılamadı',
    'modal.appId.error': 'Hata',
    'modal.appId.errorMessage': 'Bu App ID ile bir oyun bulunamadı veya Steam API hatası oluştu.',
    'modal.confirm': 'Onayla',
    'modal.cancel': 'İptal',
    'modal.ok': 'Tamam',
    
    // Rust messages
    'rust.search.turkish': 'turkish',
  }
};

function createI18nStore() {
  const storedLang = (typeof localStorage !== 'undefined' ? localStorage.getItem('paradise_lang') : null) as Language | null;
  const { subscribe, set } = writable<Language>(storedLang || 'en');

  let currentLang: Language = storedLang || 'en';
  
  subscribe(value => {
    currentLang = value;
  });

  return {
    subscribe,
    setLanguage: (lang: Language) => {
      set(lang);
      if (typeof localStorage !== 'undefined') {
        localStorage.setItem('paradise_lang', lang);
      }
      // Save to disk
      invoke('save_language', { lang }).catch(err => console.error('Failed to save language:', err));
    },
    toggle: () => {
      const newLang = currentLang === 'en' ? 'tr' : 'en';
      if (typeof localStorage !== 'undefined') {
        localStorage.setItem('paradise_lang', newLang);
      }
      invoke('save_language', { lang: newLang }).catch(err => console.error('Failed to save language:', err));
      set(newLang);
    }
  };
}

export const currentLanguage = createI18nStore();

export const t = derived(currentLanguage, $lang => {
  return (key: string): string => {
    return translations[$lang][key] || key;
  };
});

// Load language on init
if (typeof window !== 'undefined') {
  invoke<string>('load_language')
    .then(lang => {
      if (lang === 'tr' || lang === 'en') {
        currentLanguage.setLanguage(lang as Language);
      }
    })
    .catch(() => {
      // Use default 'en'
    });
}
