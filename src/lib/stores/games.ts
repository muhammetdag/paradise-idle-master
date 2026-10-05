import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { Game } from '../types';

interface SavedGame {
  app_id: string;
  name: string;
  image?: string;
  elapsed_time: number;
}

function createGamesStore() {
  const { subscribe, set, update } = writable<Game[]>([]);

  // Load favorites on initialization
  invoke<SavedGame[]>('load_favorites')
    .then(savedGames => {
      const games = savedGames.map(sg => ({
        appId: sg.app_id,
        name: sg.name,
        image: sg.image,
        isRunning: false,
        elapsedTime: sg.elapsed_time
      }));
      set(games);
    })
    .catch(err => console.error('Failed to load favorites:', err));

  // Save to disk whenever store changes
  const saveToDisk = (games: Game[]) => {
    const savedGames: SavedGame[] = games.map(g => ({
      app_id: g.appId,
      name: g.name,
      image: g.image,
      elapsed_time: g.elapsedTime
    }));
    invoke('save_favorites', { games: savedGames })
      .catch(err => console.error('Failed to save favorites:', err));
  };

  return {
    subscribe,
    add: (appId: string, name: string, image?: string) => {
      update(games => {
        if (games.find(g => g.appId === appId)) return games;
        const newGames = [...games, { appId, name, image, isRunning: false, elapsedTime: 0 }];
        saveToDisk(newGames);
        return newGames;
      });
    },
    remove: (appId: string) => {
      update(games => {
        const newGames = games.filter(g => g.appId !== appId);
        saveToDisk(newGames);
        return newGames;
      });
    },
    start: (appId: string) => {
      update(games => games.map(g => 
        g.appId === appId ? { ...g, isRunning: true, startTime: Date.now() } : g
      ));
    },
    stop: (appId: string) => {
      update(games => games.map(g => 
        g.appId === appId ? { ...g, isRunning: false, startTime: undefined } : g
      ));
    },
    updateElapsed: (appId: string, elapsed: number) => {
      update(games => {
        const newGames = games.map(g => 
          g.appId === appId ? { ...g, elapsedTime: elapsed } : g
        );
        saveToDisk(newGames);
        return newGames;
      });
    },
    reset: () => {
      set([]);
      saveToDisk([]);
    }
  };
}

export const games = createGamesStore();
