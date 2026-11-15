import React, { useEffect, useRef } from 'react';
import { Terminal, Trash2, X } from 'lucide-react';
import type { LogEntry } from '../types/launcher';

interface LogDrawerProps {
  isOpen: boolean;
  onClose: () => void;
  logs: LogEntry[];
  onClear: () => void;
}

const LEVEL_STYLES: Record<string, { color: string }> = {
  error:   { color: 'var(--status-error)' },
  success: { color: 'var(--status-success)' },
  warn:    { color: 'var(--status-warning)' },
  info:    { color: 'var(--text-muted)' },
};

export const LogDrawer: React.FC<LogDrawerProps> = ({ isOpen, onClose, logs, onClear }) => {
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (isOpen && scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [logs, isOpen]);

  const formatTime = (ts: number) => new Date(ts * 1000).toTimeString().split(' ')[0];

  if (!isOpen) return null;

  return (
    /* Bottom-anchored at footer top (26px footer) */
    <div
      className="absolute left-0 right-0 flex flex-col view-fade"
      style={{
        bottom: 0,
        height: '220px',
        background: 'var(--bg-surface)',
        borderTop: '1px solid var(--border-subtle)',
        zIndex: 40,
      }}
    >
      {/* Drawer header */}
      <div
        className="h-8 px-3 flex items-center justify-between shrink-0"
        style={{
          background: 'var(--bg-primary)',
          borderBottom: '1px solid var(--border-subtle)',
        }}
      >
        <div className="flex items-center gap-2 text-[12px] font-medium" style={{ color: 'var(--text-secondary)' }}>
          <Terminal size={12} style={{ color: 'var(--accent)' }} />
          <span>Output</span>
          <span
            className="tabular-nums"
            style={{ fontSize: '10px', color: 'var(--text-muted)', fontFamily: 'ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace' }}
          >
            ({logs.length})
          </span>
        </div>

        <div className="flex items-center gap-1">
          <IconBtn title="Clear logs" onClick={onClear}><Trash2 size={12} /></IconBtn>
          <IconBtn title="Close" onClick={onClose}><X size={13} /></IconBtn>
        </div>
      </div>

      {/* Log entries */}
      <div
        ref={scrollRef}
        className="flex-1 overflow-y-auto p-2.5 space-y-1 select-text"
        style={{
          background: 'rgba(13,17,23,0.4)',
          fontFamily: 'ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace',
          fontSize: '11px',
        }}
      >
        {logs.length === 0 ? (
          <div className="text-center py-8" style={{ color: 'var(--text-muted)' }}>No output recorded.</div>
        ) : (
          logs.map((log, i) => {
            const style = LEVEL_STYLES[log.level.toLowerCase()] ?? LEVEL_STYLES.info;
            return (
              <div key={i} className="flex items-start gap-2 leading-relaxed">
                <span className="shrink-0 tabular-nums" style={{ color: 'var(--text-muted)' }}>
                  [{formatTime(log.timestamp)}]
                </span>
                <span
                  className="shrink-0 uppercase font-bold text-[9px] px-1 py-0.5 rounded"
                  style={{
                    color: style.color,
                    border: `1px solid ${style.color}33`,
                    background: `${style.color}12`,
                    letterSpacing: '0.05em',
                  }}
                >
                  {log.level}
                </span>
                <span style={{ color: 'var(--text-primary)' }} className="break-all">{log.message}</span>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
};

const IconBtn: React.FC<{ title: string; onClick: () => void; children: React.ReactNode }> = ({ title, onClick, children }) => (
  <button
    onClick={onClick}
    title={title}
    className="p-1 rounded transition-colors duration-100"
    style={{ color: 'var(--text-muted)', background: 'transparent' }}
    onMouseEnter={e => {
      (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-elevated)';
      (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-primary)';
    }}
    onMouseLeave={e => {
      (e.currentTarget as HTMLButtonElement).style.background = 'transparent';
      (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-muted)';
    }}
  >
    {children}
  </button>
);
