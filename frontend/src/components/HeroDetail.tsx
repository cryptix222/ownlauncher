import React from 'react';
import { Play, FolderOpen, RefreshCw, Trash2, Terminal, HardDrive, Clock } from 'lucide-react';
import type { Game } from '../types/launcher';

interface HeroDetailProps {
  game: Game;
  onLaunch: (id: string) => void;
  onOpenFolder: (path: string) => void;
  onRefreshMeta: (id: string) => void;
  onRemove: (id: string) => void;
  isLaunching: boolean;
  isFetchingMeta: boolean;
}

export const HeroDetail: React.FC<HeroDetailProps> = ({
  game,
  onLaunch,
  onOpenFolder,
  onRefreshMeta,
  onRemove,
  isLaunching,
  isFetchingMeta,
}) => {
  const formatLastPlayed = (ts?: number) => {
    if (!ts) return 'Never';
    return new Date(ts * 1000).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  const getLaunchMethodDetails = () => {
    switch (game.launch_method.type) {
      case 'Uri':
        return { type: 'URI Protocol', target: game.launch_method.data.uri };
      case 'Exe':
        return { type: 'Direct Executable', target: game.launch_method.data.path };
      case 'Client':
        return {
          type: 'Client Service',
          target: `${game.launch_method.data.client_path} ${game.launch_method.data.args.join(' ')}`,
        };
      case 'Bnet':
        return {
          type: 'Battle.net Client',
          target: `${game.launch_method.data.client_path} --exec="launch ${game.launch_method.data.code}"`,
        };
      default:
        return { type: 'Standard', target: 'N/A' };
    }
  };

  const methodDetails = getLaunchMethodDetails();

  return (
    <main
      className="flex-1 h-full overflow-y-auto flex flex-col relative select-text view-fade"
      style={{ background: 'var(--bg-surface)' }}
    >
      {/* ── Hero Banner ── */}
      <div
        className="relative w-full shrink-0 overflow-hidden"
        style={{ height: '220px', background: 'var(--bg-primary)' }}
      >
        {game.banner_url ? (
          <img
            src={game.banner_url}
            alt={game.name}
            className="w-full h-full object-cover object-top"
            style={{ filter: 'brightness(0.75) contrast(1.05)' }}
          />
        ) : (
          /* Clean typographic fallback — no icons */
          <div
            className="w-full h-full flex items-end px-5 pb-5"
            style={{ background: 'var(--bg-primary)' }}
          />
        )}

        {/* Gradient fade to surface */}
        <div
          className="absolute inset-0 pointer-events-none"
          style={{
            background:
              'linear-gradient(to top, var(--bg-surface) 0%, rgba(22,27,34,0.55) 55%, transparent 100%)',
          }}
        />

        {/* Floating info + action bar */}
        <div className="absolute bottom-4 left-5 right-5 flex items-end justify-between gap-4 pointer-events-auto">
          {/* Title block */}
          <div className="space-y-1 max-w-xl">
            {/* Platform label + installed tag */}
            <div className="flex items-center gap-2">
              <span
                className="text-[11px] font-medium tracking-wide"
                style={{ color: 'var(--text-secondary)' }}
              >
                {game.platform}
              </span>
              <span
                className="text-[10px] px-1.5 py-0.5 rounded-[3px] font-mono"
                style={{
                  background: 'rgba(63,185,80,0.12)',
                  color: 'var(--status-success)',
                  letterSpacing: '0.03em',
                }}
              >
                Installed
              </span>
            </div>

            {/* Game name — large, tight, GitHub-style */}
            <h1
              className="text-[22px] font-bold tracking-tight leading-none"
              style={{ color: 'var(--text-primary)', fontFamily: 'inherit' }}
            >
              {game.name}
            </h1>
          </div>

          {/* Action buttons */}
          <div className="flex items-center gap-1.5 shrink-0">
            {/* Launch — GitHub green */}
            <button
              disabled={isLaunching}
              onClick={() => onLaunch(game.id)}
              className="btn-launch flex items-center gap-1.5 rounded-[6px] px-5 py-[7px] text-[13px] font-semibold transition-colors duration-100 disabled:opacity-50"
              style={{ background: 'var(--launch-btn)', color: 'var(--launch-btn-fg)' }}
              onMouseEnter={e => {
                if (!isLaunching)
                  (e.currentTarget as HTMLButtonElement).style.background = 'var(--launch-btn-hover)';
              }}
              onMouseLeave={e => {
                (e.currentTarget as HTMLButtonElement).style.background = 'var(--launch-btn)';
              }}
            >
              <Play size={13} fill="currentColor" stroke="none" />
              <span>{isLaunching ? 'Launching…' : 'Launch'}</span>
            </button>

            {game.installed_path && (
              <IconBtn title="Open install directory" onClick={() => onOpenFolder(game.installed_path!)}>
                <FolderOpen size={15} />
              </IconBtn>
            )}

            <IconBtn
              title="Refresh metadata"
              onClick={() => onRefreshMeta(game.id)}
              disabled={isFetchingMeta}
            >
              <RefreshCw
                size={14}
                className={isFetchingMeta ? 'animate-spin' : ''}
                style={{ color: isFetchingMeta ? 'var(--accent)' : undefined }}
              />
            </IconBtn>

            <IconBtn title="Remove from library" onClick={() => onRemove(game.id)} danger>
              <Trash2 size={14} />
            </IconBtn>
          </div>
        </div>
      </div>

      {/* ── Detail grid ── */}
      <div className="p-5 grid grid-cols-1 lg:grid-cols-3 gap-6 flex-1">
        {/* Description */}
        <div className="lg:col-span-2 space-y-2">
          <SectionLabel>About</SectionLabel>
          <div
            className="text-[13px] leading-relaxed whitespace-pre-line"
            style={{ color: 'var(--text-primary)' }}
          >
            {isFetchingMeta && (!game.description || game.description.trim() === '') ? (
              <p style={{ color: 'var(--text-muted)', fontStyle: 'italic' }}>Loading description…</p>
            ) : game.description && game.description.trim() !== '' ? (
              game.description
            ) : (
              <p style={{ color: 'var(--text-muted)' }}>No description available.</p>
            )}
          </div>
        </div>

        {/* Technical details */}
        <div className="space-y-3">
          <SectionLabel>Launch Details</SectionLabel>

          <DetailRow
            label="Type"
            value={methodDetails.type}
            mono
          />

          <DetailRow
            label={<><Terminal size={10} style={{ display: 'inline', marginRight: 3 }} />Command / URI</>}
            value={methodDetails.target}
            mono
            small
            selectable
          />

          {game.installed_path && (
            <DetailRow
              label={<><HardDrive size={10} style={{ display: 'inline', marginRight: 3 }} />Install Directory</>}
              value={game.installed_path}
              mono
              small
              selectable
            />
          )}

          <DetailRow
            label={<><Clock size={10} style={{ display: 'inline', marginRight: 3 }} />Last Played</>}
            value={formatLastPlayed(game.last_launched)}
          />
        </div>
      </div>
    </main>
  );
};

/* ── Internal helpers ── */

const SectionLabel: React.FC<{ children: React.ReactNode }> = ({ children }) => (
  <h2
    className="text-[11px] font-semibold uppercase tracking-wider"
    style={{ color: 'var(--text-secondary)' }}
  >
    {children}
  </h2>
);

interface DetailRowProps {
  label: React.ReactNode;
  value: string;
  mono?: boolean;
  small?: boolean;
  selectable?: boolean;
}

const DetailRow: React.FC<DetailRowProps> = ({ label, value, mono, small, selectable }) => (
  <div>
    <span
      className="flex items-center text-[11px] mb-0.5"
      style={{ color: 'var(--text-muted)' }}
    >
      {label}
    </span>
    <span
      title={value}
      style={{
        display: 'block',
        fontFamily: mono
          ? 'ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace'
          : 'inherit',
        fontSize: small ? '10px' : '12px',
        fontWeight: !mono ? 500 : undefined,
        color: small ? 'var(--text-secondary)' : 'var(--text-primary)',
        wordBreak: 'break-all',
        userSelect: selectable ? 'text' : undefined,
      }}
    >
      {value}
    </span>
  </div>
);

const IconBtn: React.FC<{
  title: string;
  onClick: () => void;
  disabled?: boolean;
  danger?: boolean;
  children: React.ReactNode;
}> = ({ title, onClick, disabled, danger, children }) => (
  <button
    onClick={onClick}
    disabled={disabled}
    title={title}
    className="p-2 rounded-[4px] transition-colors duration-100 disabled:opacity-40"
    style={{ color: 'var(--text-muted)', background: 'transparent' }}
    onMouseEnter={e => {
      (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-hover)';
      (e.currentTarget as HTMLButtonElement).style.color = danger
        ? 'var(--status-error)'
        : 'var(--text-primary)';
    }}
    onMouseLeave={e => {
      (e.currentTarget as HTMLButtonElement).style.background = 'transparent';
      (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-muted)';
    }}
  >
    {children}
  </button>
);
