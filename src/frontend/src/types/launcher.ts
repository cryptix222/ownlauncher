export type LaunchMethod =
  | { type: 'Exe'; data: { path: string } }
  | { type: 'Uri'; data: { uri: string } }
  | { type: 'Client'; data: { client_path: string; args: string[] } }
  | { type: 'Bnet'; data: { client_path: string; code: string } };

export interface Game {
  id: string;
  name: string;
  platform: 'Steam' | 'Epic Games' | 'Battle.net' | 'Riot Games' | 'Manual' | string;
  launch_method: LaunchMethod;
  banner_url?: string;
  description?: string;
  installed_path?: string;
  last_launched?: number;
}

export interface Config {
  theme: string;
  minimize_on_launch: boolean;
  close_on_launch: boolean;
  show_platform_badges: boolean;
  compact_sidebar: boolean;
  auto_scan_on_startup: boolean;
  games: Game[];
}

export interface GameMetadata {
  description: string;
  banner_url?: string;
}

export interface LogEntry {
  level: 'info' | 'success' | 'warn' | 'error';
  message: string;
  timestamp: number;
}

export interface StatusState {
  text: string;
  is_busy: boolean;
}
