import React, { useEffect, useState } from 'react';
import { Titlebar } from './components/Titlebar';
import { Sidebar } from './components/Sidebar';
import { HeroDetail } from './components/HeroDetail';
import { FooterBar } from './components/FooterBar';
import { LogDrawer } from './components/LogDrawer';
import { SettingsModal } from './components/SettingsModal';
import { AddGameModal } from './components/AddGameModal';
import { bridge } from './services/bridge';
import type { Config, Game, LogEntry, StatusState } from './types/launcher';

export const App: React.FC = () => {
  const [config, setConfig] = useState<Config>({
    theme: 'dark',
    minimize_on_launch: false,
    close_on_launch: false,
    show_platform_badges: true,
    compact_sidebar: false,
    auto_scan_on_startup: true,
    games: [],
  });

  const [selectedGameId, setSelectedGameId] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedPlatform, setSelectedPlatform] = useState('All');
  const [status, setStatus] = useState<StatusState>({ text: 'Initializing...', is_busy: false });
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [isLogOpen, setIsLogOpen] = useState(false);
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [isAddModalOpen, setIsAddModalOpen] = useState(false);
  const [isLaunching, setIsLaunching] = useState(false);
  const [isFetchingMeta, setIsFetchingMeta] = useState(false);
  const [isScanning, setIsScanning] = useState(false);

  // Apply theme to DOM
  const applyTheme = (themeName: string) => {
    const validTheme = themeName || 'dark';
    document.documentElement.setAttribute('data-theme', validTheme);
    try {
      localStorage.setItem('ownlauncher_theme', validTheme);
    } catch (_) {}
  };

  // Initialize and register event listeners
  useEffect(() => {
    // 0. Initial theme from localStorage fallback
    try {
      const savedTheme = localStorage.getItem('ownlauncher_theme');
      if (savedTheme) {
        applyTheme(savedTheme);
      }
    } catch (_) {}

    // 1. Fetch initial configuration from Rust backend
    bridge.invoke<Config>('get_config').then(cfg => {
      if (cfg) {
        setConfig(cfg);
        applyTheme(cfg.theme);

        if (cfg.games && cfg.games.length > 0) {
          setSelectedGameId(cfg.games[0].id);
        }
        setStatus({ text: 'Ready', is_busy: false });

        // Auto-scan on startup if enabled
        if (cfg.auto_scan_on_startup) {
          handleRescan();
        }
      }
    }).catch(err => {
      console.error('Failed to load config:', err);
      setStatus({ text: 'Configuration error', is_busy: false });
    });

    // 2. Register typed backend event listeners
    const unlistenConfig = bridge.on('ConfigLoaded', newCfg => {
      setConfig(newCfg);
      applyTheme(newCfg.theme);
    });

    const unlistenGames = bridge.on('GamesDiscovered', games => {
      setConfig(prev => ({ ...prev, games }));
      setSelectedGameId(prevId => {
        if (!prevId && games.length > 0) {
          return games[0].id;
        }
        return prevId;
      });
    });

    const unlistenMeta = bridge.on('MetadataLoaded', ({ game_id, metadata }) => {
      setConfig(prev => ({
        ...prev,
        games: prev.games.map(g =>
          g.id === game_id
            ? { ...g, banner_url: metadata.banner_url || g.banner_url, description: metadata.description }
            : g
        ),
      }));
      setIsFetchingMeta(false);
    });

    const unlistenStatus = bridge.on('Status', s => {
      setStatus(s);
      if (!s.is_busy) {
        setIsScanning(false);
      }
    });

    // 3. Batched Live-Log Stream
    const unlistenLogs = bridge.onLogs(batch => {
      setLogs(prev => [...prev.slice(-300), ...batch]);
    });

    // 4. Global keyboard shortcuts (Ctrl+K for search)
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault();
        const searchInput = document.querySelector('input[placeholder*="Search"]') as HTMLInputElement;
        if (searchInput) {
          searchInput.focus();
        }
      }
    };
    window.addEventListener('keydown', handleKeyDown);

    return () => {
      unlistenConfig();
      unlistenGames();
      unlistenMeta();
      unlistenStatus();
      unlistenLogs();
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, []);

  // Automatically fetch metadata/description for the selected game if missing
  useEffect(() => {
    if (selectedGameId) {
      const currentGame = config.games.find(g => g.id === selectedGameId);
      if (currentGame && (!currentGame.description || currentGame.description.trim() === '')) {
        setIsFetchingMeta(true);
        bridge.invoke('fetch_metadata', { game_id: selectedGameId }).catch(err => {
          console.error('Auto metadata fetch error:', err);
        }).finally(() => {
          setIsFetchingMeta(false);
        });
      }
    }
  }, [selectedGameId]);

  const selectedGame = config.games.find(g => g.id === selectedGameId) || null;

  const handleLaunch = async (id: string) => {
    try {
      setIsLaunching(true);
      await bridge.invoke('launch_game', { id });
    } catch (err: any) {
      alert(`Launch error: ${err?.message || err}`);
    } finally {
      setIsLaunching(false);
    }
  };

  const handleOpenFolder = async (path: string) => {
    try {
      await bridge.openFolder(path);
    } catch (err: any) {
      console.error('Error opening folder:', err);
    }
  };

  const handleRefreshMeta = async (id: string) => {
    try {
      setIsFetchingMeta(true);
      await bridge.invoke('fetch_metadata', { game_id: id });
    } catch (err: any) {
      console.error('Error fetching metadata:', err);
      setIsFetchingMeta(false);
    }
  };

  const handleRemoveGame = async (id: string) => {
    if (confirm('Are you sure you want to remove this game from your library?')) {
      await bridge.invoke('remove_game', { id });
      setConfig(prev => {
        const nextGames = prev.games.filter(g => g.id !== id);
        if (selectedGameId === id) {
          setSelectedGameId(nextGames.length > 0 ? nextGames[0].id : null);
        }
        return { ...prev, games: nextGames };
      });
    }
  };

  const handleRescan = async () => {
    setIsScanning(true);
    try {
      await bridge.invoke('scan_games');
    } catch (err) {
      console.error('Scan error:', err);
      setIsScanning(false);
    }
  };

  const handleAddGame = async (name: string, path: string) => {
    const newGame = await bridge.invoke<Game>('add_game', { name, path });
    if (newGame) {
      setSelectedGameId(newGame.id);
    }
  };

  const handleUpdateConfig = async (newCfg: Config) => {
    setConfig(newCfg);
    applyTheme(newCfg.theme);
    await bridge.invoke('save_config', newCfg);
  };

  return (
    <div
      className="flex flex-col h-screen w-screen overflow-hidden font-sans"
      style={{ background: 'var(--bg-canvas)', color: 'var(--text-primary)' }}
    >
      {/* 1. Titlebar (brand + window controls only — no status here) */}
      <Titlebar />

      {/* 2. Main Content Viewport */}
      <div className="flex flex-1 overflow-hidden relative">
        <Sidebar
          games={config.games}
          selectedGameId={selectedGameId}
          onSelectGame={setSelectedGameId}
          searchQuery={searchQuery}
          onSearchChange={setSearchQuery}
          selectedPlatform={selectedPlatform}
          onSelectPlatform={setSelectedPlatform}
          compact={config.compact_sidebar}
          showBadges={config.show_platform_badges}
          onOpenAddModal={() => setIsAddModalOpen(true)}
          onOpenSettings={() => setIsSettingsOpen(true)}
          onRescan={handleRescan}
          isScanning={isScanning}
        />

        {selectedGame ? (
          <HeroDetail
            game={selectedGame}
            onLaunch={handleLaunch}
            onOpenFolder={handleOpenFolder}
            onRefreshMeta={handleRefreshMeta}
            onRemove={handleRemoveGame}
            isLaunching={isLaunching}
            isFetchingMeta={isFetchingMeta}
          />
        ) : (
          <main
            className="flex-1 flex flex-col items-center justify-center text-center p-8 select-none view-fade"
            style={{ background: 'var(--bg-surface)' }}
          >
            <div
              className="w-12 h-12 rounded-[4px] flex items-center justify-center mb-3"
              style={{ background: 'var(--bg-primary)' }}
            >
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="var(--text-muted)" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" opacity={0.4}>
                <rect x="2" y="6" width="20" height="12" rx="2"/>
                <path d="M8 12h.01M12 12h.01M16 12h.01"/>
              </svg>
            </div>
            <h2 className="text-[13px] font-semibold mb-1" style={{ color: 'var(--text-primary)' }}>
              No game selected
            </h2>
            <p className="text-[12px] max-w-xs mb-4 leading-normal" style={{ color: 'var(--text-muted)' }}>
              Choose a title from the library or add an executable.
            </p>
            <button
              onClick={handleRescan}
              disabled={isScanning}
              className="px-3 py-1.5 rounded-[4px] text-[12px] font-medium transition-colors duration-100 disabled:opacity-40"
              style={{ background: 'var(--bg-elevated)', color: 'var(--text-primary)' }}
              onMouseEnter={e => { (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-hover)'; }}
              onMouseLeave={e => { (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-elevated)'; }}
            >
              {isScanning ? 'Scanning…' : 'Scan Library'}
            </button>
          </main>
        )}

        {/* Log drawer (anchored above footer) */}
        <LogDrawer
          isOpen={isLogOpen}
          onClose={() => setIsLogOpen(false)}
          logs={logs}
          onClear={() => setLogs([])}
        />
      </div>

      {/* 3. Status bar footer — the ONLY place for status/progress/scan info */}
      <FooterBar
        status={status}
        logCount={logs.length}
        isLogOpen={isLogOpen}
        onToggleLog={() => setIsLogOpen(prev => !prev)}
      />

      {/* Modals */}
      <SettingsModal
        isOpen={isSettingsOpen}
        onClose={() => setIsSettingsOpen(false)}
        config={config}
        onUpdateConfig={handleUpdateConfig}
        onRescan={handleRescan}
        isScanning={isScanning}
      />

      <AddGameModal
        isOpen={isAddModalOpen}
        onClose={() => setIsAddModalOpen(false)}
        onAdd={handleAddGame}
      />
    </div>
  );
};

export default App;
