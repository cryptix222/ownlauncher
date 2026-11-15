import React from 'react';
import { Search, Plus, Settings, RotateCw, X } from 'lucide-react';
import type { Game } from '../types/launcher';

interface SidebarProps {
  games: Game[];
  selectedGameId: string | null;
  onSelectGame: (id: string) => void;
  searchQuery: string;
  onSearchChange: (q: string) => void;
  selectedPlatform: string;
  onSelectPlatform: (platform: string) => void;
  compact: boolean;
  showBadges: boolean;
  onOpenAddModal: () => void;
  onOpenSettings: () => void;
  onRescan: () => void;
  isScanning: boolean;
}

const PLATFORMS = ['All', 'Steam', 'Epic Games', 'Battle.net', 'Riot Games', 'Manual'];

export const Sidebar: React.FC<SidebarProps> = ({
  games,
  selectedGameId,
  onSelectGame,
  searchQuery,
  onSearchChange,
  selectedPlatform,
  onSelectPlatform,
  compact,
  showBadges,
  onOpenAddModal,
  onOpenSettings,
  onRescan,
  isScanning,
}) => {
  const filteredGames = games.filter(game => {
    const matchesSearch =
      searchQuery.trim() === '' ||
      game.name.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesPlatform =
      selectedPlatform === 'All' ||
      game.platform.toLowerCase() === selectedPlatform.toLowerCase();
    return matchesSearch && matchesPlatform;
  });

  return (
    <aside
      className="w-60 flex flex-col h-full select-none shrink-0"
      style={{
        background: 'var(--bg-primary)',
        borderRight: '1px solid var(--border-subtle)',
      }}
    >
      {/* ── Search ── */}
      <div className="p-2.5 space-y-2" style={{ borderBottom: '1px solid var(--border-subtle)' }}>
        <div className="relative flex items-center">
          <Search size={12} className="absolute left-2.5 pointer-events-none" style={{ color: 'var(--text-muted)' }} />
          <input
            type="text"
            value={searchQuery}
            onChange={e => onSearchChange(e.target.value)}
            placeholder="Search… (Ctrl+K)"
            className="w-full rounded-[6px] pl-8 pr-7 py-[5px] text-[12px] focus:outline-none transition-colors duration-100"
            style={{
              background: 'var(--bg-surface)',
              color: 'var(--text-primary)',
              border: '1px solid var(--border-subtle)',
              caretColor: 'var(--accent)',
              fontFamily: 'inherit',
            }}
            onFocus={e => { (e.currentTarget as HTMLInputElement).style.borderColor = 'var(--accent)'; }}
            onBlur={e =>  { (e.currentTarget as HTMLInputElement).style.borderColor = 'var(--border-subtle)'; }}
          />
          {searchQuery && (
            <button
              onClick={() => onSearchChange('')}
              className="absolute right-2 p-0.5 transition-colors duration-100"
              style={{ color: 'var(--text-muted)' }}
            >
              <X size={11} />
            </button>
          )}
        </div>

        {/* Platform filter chips */}
        <div className="flex items-center gap-0.5 overflow-x-auto pb-0.5" style={{ scrollbarWidth: 'none' }}>
          {PLATFORMS.map(p => {
            const isActive = selectedPlatform === p;
            const count =
              p === 'All'
                ? games.length
                : games.filter(g => g.platform.toLowerCase() === p.toLowerCase()).length;
            return (
              <button
                key={p}
                onClick={() => onSelectPlatform(p)}
                className="px-1.5 py-0.5 rounded-[4px] text-[11px] font-medium whitespace-nowrap transition-colors duration-100 flex items-center gap-1 shrink-0"
                style={{
                  background: isActive ? 'var(--bg-elevated)' : 'transparent',
                  color: isActive ? 'var(--text-primary)' : 'var(--text-muted)',
                }}
                onMouseEnter={e => {
                  if (!isActive) {
                    (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-hover)';
                    (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-secondary)';
                  }
                }}
                onMouseLeave={e => {
                  if (!isActive) {
                    (e.currentTarget as HTMLButtonElement).style.background = 'transparent';
                    (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-muted)';
                  }
                }}
              >
                <span>{p}</span>
                <span style={{ opacity: 0.45, fontSize: '10px' }}>({count})</span>
              </button>
            );
          })}
        </div>
      </div>

      {/* ── Game list ── */}
      <div className="flex-1 overflow-y-auto p-1.5 flex flex-col" style={{ gap: '1px' }}>
        {filteredGames.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-44 text-center px-4 gap-1.5">
            <p className="text-[12px]" style={{ color: 'var(--text-secondary)' }}>No games found</p>
            <p className="text-[11px] leading-snug" style={{ color: 'var(--text-muted)' }}>
              Adjust your filter or rescan.
            </p>
          </div>
        ) : (
          filteredGames.map(game => {
            const isSelected = game.id === selectedGameId;
            return (
              <button
                key={game.id}
                onClick={() => onSelectGame(game.id)}
                className="w-full text-left rounded-[4px] transition-colors duration-100 flex items-center gap-2"
                style={{
                  padding: compact ? '5px 8px' : '6px 8px',
                  background: isSelected ? 'var(--bg-elevated)' : 'transparent',
                }}
                onMouseEnter={e => {
                  if (!isSelected) (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-hover)';
                }}
                onMouseLeave={e => {
                  if (!isSelected) (e.currentTarget as HTMLButtonElement).style.background = 'transparent';
                }}
              >
                {/* Selection indicator bar */}
                <span
                  className="shrink-0 rounded-full"
                  style={{
                    width: '2px',
                    height: isSelected ? '14px' : '6px',
                    background: isSelected ? 'var(--accent)' : 'transparent',
                    transition: 'height 120ms cubic-bezier(0.2,0,0,1), background 100ms ease-out',
                  }}
                />

                {/* Text content */}
                <div className="flex-1 min-w-0 flex items-center justify-between gap-2">
                  <span
                    className="truncate text-[12px]"
                    style={{
                      color: isSelected ? 'var(--text-primary)' : 'var(--text-secondary)',
                      fontWeight: isSelected ? 600 : 400,
                    }}
                  >
                    {game.name}
                  </span>

                  {showBadges && (
                    <span
                      className="shrink-0 text-[9px] font-mono tracking-wider uppercase"
                      style={{ color: 'var(--text-muted)' }}
                    >
                      {game.platform === 'Epic Games' ? 'Epic' : game.platform}
                    </span>
                  )}
                </div>
              </button>
            );
          })
        )}
      </div>

      {/* ── Bottom toolbar ── */}
      <div
        className="p-2 flex items-center justify-between gap-1"
        style={{ borderTop: '1px solid var(--border-subtle)', background: 'var(--bg-primary)' }}
      >
        <button
          onClick={onOpenAddModal}
          title="Add Game Manually"
          className="flex-1 py-1 px-2 rounded-[4px] text-[12px] font-medium flex items-center justify-center gap-1.5 transition-colors duration-100"
          style={{ background: 'var(--bg-surface)', color: 'var(--text-secondary)' }}
          onMouseEnter={e => {
            (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-elevated)';
            (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-primary)';
          }}
          onMouseLeave={e => {
            (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-surface)';
            (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-secondary)';
          }}
        >
          <Plus size={12} style={{ color: 'var(--accent)' }} />
          <span>Add Game</span>
        </button>

        <button
          onClick={onRescan}
          disabled={isScanning}
          title="Rescan Library"
          className="p-1.5 rounded-[4px] transition-colors duration-100 disabled:opacity-40"
          style={{ background: 'var(--bg-surface)', color: 'var(--text-muted)' }}
          onMouseEnter={e => {
            (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-elevated)';
            (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-primary)';
          }}
          onMouseLeave={e => {
            (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-surface)';
            (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-muted)';
          }}
        >
          <RotateCw
            size={13}
            className={isScanning ? 'animate-spin' : ''}
            style={{ color: isScanning ? 'var(--accent)' : undefined }}
          />
        </button>

        <button
          onClick={onOpenSettings}
          title="Settings"
          className="p-1.5 rounded-[4px] transition-colors duration-100"
          style={{ background: 'var(--bg-surface)', color: 'var(--text-muted)' }}
          onMouseEnter={e => {
            (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-elevated)';
            (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-primary)';
          }}
          onMouseLeave={e => {
            (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-surface)';
            (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-muted)';
          }}
        >
          <Settings size={13} />
        </button>
      </div>
    </aside>
  );
};
