import type { Config, Game, GameMetadata, LogEntry, StatusState } from '../types/launcher';

declare global {
  interface Window {
    ipc?: {
      postMessage: (message: string) => void;
    };
    __LAUNCHER_PENDING__?: Map<string, { resolve: (val: any) => void; reject: (err: Error) => void }>;
    __LAUNCHER_LISTENERS__?: Map<string, Array<(payload: any) => void>>;
    __LAUNCHER_RESPONSE__?: (resp: any) => void;
    __LAUNCHER_EVENT__?: (event: any) => void;
    __ON_WINDOW_MAXIMIZE_CHANGED?: (isMaximized: boolean) => void;
  }
}

type EventMap = {
  ConfigLoaded: Config;
  GamesDiscovered: Game[];
  MetadataLoaded: { game_id: string; metadata: GameMetadata };
  Log: LogEntry;
  Status: StatusState;
};

class BridgeService {
  private hasNativeIpc = typeof window !== 'undefined' && !!window.ipc;
  private pendingRequests = new Map<string, { resolve: (val: any) => void; reject: (err: Error) => void }>();
  private listeners = new Map<string, Set<(payload: any) => void>>();
  private maximizeListeners = new Set<(isMax: boolean) => void>();
  private logBuffer: LogEntry[] = [];
  private logRafScheduled = false;
  private logSubscribers = new Set<(logs: LogEntry[]) => void>();

  constructor() {
    if (typeof window !== 'undefined') {
      window.__LAUNCHER_PENDING__ = this.pendingRequests;

      // Handle window maximize state changes from Rust
      window.__ON_WINDOW_MAXIMIZE_CHANGED = (isMaximized: boolean) => {
        this.maximizeListeners.forEach(cb => {
          try { cb(isMaximized); } catch (e) { console.error(e); }
        });
      };

      // Handle responses from Rust
      window.__LAUNCHER_RESPONSE__ = (resp: { id: string; success: boolean; data?: any; error?: string }) => {
        const entry = this.pendingRequests.get(resp.id);
        if (entry) {
          this.pendingRequests.delete(resp.id);
          if (resp.success) {
            entry.resolve(resp.data);
          } else {
            entry.reject(new Error(resp.error || 'Native bridge error'));
          }
        }
      };

      // Handle pushed events from Rust
      window.__LAUNCHER_EVENT__ = (event: { type: string; payload: any }) => {
        if (event.type === 'Log') {
          this.queueLog(event.payload as LogEntry);
        }

        const callbacks = this.listeners.get(event.type);
        if (callbacks) {
          callbacks.forEach(cb => {
            try {
              cb(event.payload);
            } catch (err) {
              console.error(`Error in Bridge event handler for [${event.type}]:`, err);
            }
          });
        }
      };
    }
  }

  /**
   * Invokes an asynchronous command to the Rust backend
   */
  public async invoke<T = any>(action: string, payload: any = {}): Promise<T> {
    if (!this.hasNativeIpc || !window.ipc) {
      console.warn(`[Bridge-DevMode] Simulating call: ${action}`, payload);
      return this.mockInvoke(action, payload);
    }

    const id = `${Date.now()}-${Math.random().toString(36).substring(2, 9)}`;

    return new Promise<T>((resolve, reject) => {
      this.pendingRequests.set(id, { resolve, reject });

      const requestObj = { id, action, payload };
      try {
        window.ipc!.postMessage(JSON.stringify(requestObj));
      } catch (err) {
        this.pendingRequests.delete(id);
        reject(err instanceof Error ? err : new Error(String(err)));
      }

      // Timeout safety: 30s
      setTimeout(() => {
        if (this.pendingRequests.has(id)) {
          this.pendingRequests.delete(id);
          reject(new Error(`Timeout waiting for backend response to '${action}'`));
        }
      }, 30000);
    });
  }

  /**
   * Registers a typed event listener and returns an unlisten function
   */
  public on<K extends keyof EventMap>(event: K, callback: (payload: EventMap[K]) => void): () => void {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, new Set());
    }

    const set = this.listeners.get(event)!;
    set.add(callback);

    return () => {
      set.delete(callback);
      if (set.size === 0) {
        this.listeners.delete(event);
      }
    };
  }

  /**
   * High performance throttled log stream (batches updates with requestAnimationFrame)
   */
  public onLogs(callback: (newLogs: LogEntry[]) => void): () => void {
    this.logSubscribers.add(callback);
    return () => {
      this.logSubscribers.delete(callback);
    };
  }

  private queueLog(log: LogEntry) {
    this.logBuffer.push(log);
    if (!this.logRafScheduled) {
      this.logRafScheduled = true;
      requestAnimationFrame(() => {
        const batch = [...this.logBuffer];
        this.logBuffer = [];
        this.logRafScheduled = false;
        this.logSubscribers.forEach(cb => cb(batch));
      });
    }
  }

  /**
   * Subscribes to window maximize state changes
   */
  public onMaximizeChanged(callback: (isMax: boolean) => void): () => void {
    this.maximizeListeners.add(callback);
    return () => {
      this.maximizeListeners.delete(callback);
    };
  }

  // --- Window Helper Commands ---
  public minimize() {
    return this.invoke('window_minimize');
  }

  public toggleMaximize() {
    return this.invoke('window_maximize_toggle');
  }

  public close() {
    return this.invoke('window_close');
  }

  public startDrag() {
    return this.invoke('window_drag');
  }

  public openFolder(path: string) {
    return this.invoke('open_folder', { path });
  }

  public openUrl(url: string) {
    return this.invoke('open_url', { url });
  }

  // --- Mock implementation when running in normal browser dev server ---
  private async mockInvoke(action: string, payload: any): Promise<any> {
    switch (action) {
      case 'get_config':
        return {
          theme: 'dark',
          minimize_on_launch: false,
          close_on_launch: false,
          show_platform_badges: true,
          compact_sidebar: false,
          auto_scan_on_startup: true,
          games: [
            {
              id: 'steam:730',
              name: 'Counter-Strike 2',
              platform: 'Steam',
              launch_method: { type: 'Uri', data: { uri: 'steam://rungameid/730' } },
              banner_url: 'https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/730/library_hero.jpg',
              description: 'The successor to CS:GO built on the Source 2 engine, featuring improved smoke, lighting, and tick-rate systems.',
              installed_path: 'C:\\Program Files (x86)\\Steam\\steamapps\\common\\Counter-Strike Global Offensive',
            },
            {
              id: 'riot:valorant',
              name: 'VALORANT',
              platform: 'Riot Games',
              launch_method: { type: 'Client', data: { client_path: 'C:\\RiotClient.exe', args: [] } },
              banner_url: 'https://images.contentstack.io/v3/assets/blt370612130e63e66f/blt028a3825cb10f5e0/5f4ee7be73661635606d3cf6/VALORANT_Jett_Red_Banner.jpg',
              description: 'A tactical 5v5 character-based shooter from Riot Games. Precise gunplay meets unique agent abilities.',
              installed_path: 'C:\\Riot Games\\VALORANT',
            }
          ]
        };
      case 'scan_games':
        return [];
      case 'fetch_metadata':
        return { description: 'Mock description — metadata not available in development mode.', banner_url: undefined };
      case 'launch_game':
        return true;
      case 'save_config':
        return payload;
      default:
        return true;
    }
  }
}

export const bridge = new BridgeService();
