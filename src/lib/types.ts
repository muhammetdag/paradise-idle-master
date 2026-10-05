// Type definitions for the Steam Idler app

export interface Game {
  appId: string;
  name: string;
  image?: string;
  isRunning: boolean;
  startTime?: number;
  elapsedTime: number;
}

export interface SteamSearchItem {
  type: string;
  name: string;
  id: number;
  tiny_image: string;
  metascore?: string;
  platforms: {
    windows: boolean;
    mac: boolean;
    linux: boolean;
  };
  streamingvideo: boolean;
  controller_support?: string;
  price?: {
    currency: string;
    initial: number;
    final: number;
  };
}

export interface SteamSearchResponse {
  total: number;
  items: SteamSearchItem[];
}
