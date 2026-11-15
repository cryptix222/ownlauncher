import React, { useState, useEffect } from 'react';
import { bridge } from '../services/bridge';

/**
 * Titlebar – GitHub Desktop-style custom frame.
 * Contains ONLY: App icon + name (left) · Window controls (right).
 * All status / scan / progress info lives exclusively in FooterBar.
 */
export const Titlebar: React.FC = () => {
  const [isMaximized, setIsMaximized] = useState(false);

  useEffect(() => {
    return bridge.onMaximizeChanged((max) => setIsMaximized(max));
  }, []);

  const handleDrag = (e: React.MouseEvent) => {
    if (e.button === 0) bridge.startDrag();
  };

  return (
    <header
      className="h-[32px] flex items-center justify-between select-none z-50 relative"
      style={{
        background: 'var(--bg-primary)',
        borderBottom: '1px solid var(--border-subtle)',
      }}
    >
      {/* ── Drag region + Brand ── */}
      <div
        onMouseDown={handleDrag}
        onDoubleClick={() => bridge.toggleMaximize()}
        className="flex-1 h-full flex items-center px-3 titlebar-drag cursor-default"
      >
        <div className="flex items-center gap-2 titlebar-no-drag pointer-events-auto">
          <img
            src="/icon.ico"
            alt=""
            aria-hidden="true"
            className="w-4 h-4 object-contain select-none pointer-events-none"
            style={{ imageRendering: 'crisp-edges' }}
          />
          <span
            className="text-[12px] font-semibold tracking-wide"
            style={{ color: 'var(--text-primary)', fontFamily: 'inherit' }}
          >
            OwnLauncher
          </span>
        </div>
      </div>

      {/* ── Window Controls (Windows 11 native-style hitbox) ── */}
      <div className="flex items-center h-full titlebar-no-drag">
        {/* Minimize */}
        <button
          type="button"
          onMouseDown={(e) => e.stopPropagation()}
          onClick={(e) => { e.stopPropagation(); bridge.minimize(); }}
          title="Minimize"
          className="w-[46px] h-full flex items-center justify-center transition-colors duration-100"
          style={{ color: 'var(--text-muted)' }}
          onMouseEnter={(e) => {
            (e.currentTarget as HTMLButtonElement).style.background = 'rgba(255,255,255,0.08)';
            (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-primary)';
          }}
          onMouseLeave={(e) => {
            (e.currentTarget as HTMLButtonElement).style.background = 'transparent';
            (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-muted)';
          }}
        >
          <svg width="10" height="1" viewBox="0 0 10 1" fill="currentColor">
            <rect width="10" height="1" />
          </svg>
        </button>

        {/* Maximize / Restore */}
        <button
          type="button"
          onMouseDown={(e) => e.stopPropagation()}
          onClick={(e) => { e.stopPropagation(); bridge.toggleMaximize(); }}
          title={isMaximized ? 'Restore' : 'Maximize'}
          className="w-[46px] h-full flex items-center justify-center transition-colors duration-100"
          style={{ color: 'var(--text-muted)' }}
          onMouseEnter={(e) => {
            (e.currentTarget as HTMLButtonElement).style.background = 'rgba(255,255,255,0.08)';
            (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-primary)';
          }}
          onMouseLeave={(e) => {
            (e.currentTarget as HTMLButtonElement).style.background = 'transparent';
            (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-muted)';
          }}
        >
          {isMaximized ? (
            <svg width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" strokeWidth="1">
              <path d="M2.5 7.5H1.5V1.5H7.5V2.5" />
              <rect x="2.5" y="2.5" width="6" height="6" />
            </svg>
          ) : (
            <svg width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" strokeWidth="1">
              <rect x="0.5" y="0.5" width="9" height="9" />
            </svg>
          )}
        </button>

        {/* Close */}
        <button
          type="button"
          onMouseDown={(e) => e.stopPropagation()}
          onClick={(e) => { e.stopPropagation(); bridge.close(); }}
          title="Close"
          className="w-[46px] h-full flex items-center justify-center transition-colors duration-100"
          style={{ color: 'var(--text-muted)' }}
          onMouseEnter={(e) => {
            (e.currentTarget as HTMLButtonElement).style.background = '#E81123';
            (e.currentTarget as HTMLButtonElement).style.color = '#ffffff';
          }}
          onMouseLeave={(e) => {
            (e.currentTarget as HTMLButtonElement).style.background = 'transparent';
            (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-muted)';
          }}
        >
          <svg width="10" height="10" viewBox="0 0 10 10" stroke="currentColor" strokeWidth="1.1">
            <line x1="0" y1="0" x2="10" y2="10" />
            <line x1="10" y1="0" x2="0" y2="10" />
          </svg>
        </button>
      </div>
    </header>
  );
};
