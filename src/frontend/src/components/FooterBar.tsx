import React, { useEffect, useRef, useState } from 'react';
import { Terminal, Circle } from 'lucide-react';
import type { StatusState } from '../types/launcher';

interface FooterBarProps {
  status: StatusState;
  logCount: number;
  isLogOpen: boolean;
  onToggleLog: () => void;
}

/**
 * FooterBar – GitHub Desktop / VS Code-style status bar.
 * This is the ONLY place where status, progress, and background-task
 * info is displayed. Height: 26px.
 */
export const FooterBar: React.FC<FooterBarProps> = ({
  status,
  logCount,
  isLogOpen,
  onToggleLog,
}) => {
  /* Fade the status text when it changes */
  const [displayText, setDisplayText] = useState(status.text || 'Ready');
  const [fadeKey, setFadeKey] = useState(0);
  const prevText = useRef(status.text);

  useEffect(() => {
    if (status.text !== prevText.current) {
      prevText.current = status.text;
      setFadeKey(k => k + 1);
      setDisplayText(status.text || 'Ready');
    }
  }, [status.text]);

  return (
    <footer
      className="relative h-[26px] flex items-center justify-between px-3 select-none z-30 shrink-0"
      style={{
        background: 'var(--bg-primary)',
        borderTop: '1px solid var(--border-subtle)',
      }}
    >
      {/* ── Animated progress bar (top edge, only when busy) ── */}
      {status.is_busy && (
        <div
          className="absolute top-0 left-0 right-0 overflow-hidden"
          style={{ height: '2px', background: 'var(--border-subtle)' }}
        >
          <div
            className="absolute top-0 h-full progress-scan"
            style={{
              width: '33%',
              background: 'var(--accent)',
              borderRadius: '1px',
            }}
          />
        </div>
      )}

      {/* ── Left: status indicator + text ── */}
      <div className="flex items-center gap-2 min-w-0">
        {/* Dot indicator */}
        <Circle
          size={6}
          fill={status.is_busy ? 'var(--accent)' : 'var(--status-success)'}
          stroke="none"
          className="shrink-0 transition-colors duration-200"
          style={{ color: status.is_busy ? 'var(--accent)' : 'var(--status-success)' }}
        />

        {/* Status text with fade-on-change */}
        <span
          key={fadeKey}
          className="text-[11px] truncate max-w-xs view-fade"
          style={{
            color: 'var(--text-muted)',
            fontFamily: 'ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace',
          }}
          title={displayText}
        >
          {displayText}
        </span>
      </div>

      {/* ── Right: Log drawer toggle ── */}
      <button
        onClick={onToggleLog}
        title="Toggle output log"
        className="flex items-center gap-1.5 h-full px-2 transition-colors duration-100"
        style={{
          color: isLogOpen ? 'var(--text-primary)' : 'var(--text-muted)',
          background: isLogOpen ? 'var(--bg-elevated)' : 'transparent',
          fontSize: '11px',
        }}
        onMouseEnter={(e) => {
          if (!isLogOpen) {
            (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-primary)';
            (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-hover)';
          }
        }}
        onMouseLeave={(e) => {
          if (!isLogOpen) {
            (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-muted)';
            (e.currentTarget as HTMLButtonElement).style.background = 'transparent';
          }
        }}
      >
        <Terminal size={10} />
        <span style={{ fontFamily: 'inherit' }}>Logs</span>
        {logCount > 0 && (
          <span
            className="tabular-nums"
            style={{
              fontSize: '10px',
              color: 'var(--text-muted)',
              fontFamily: 'ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace',
              paddingLeft: '2px',
            }}
          >
            {logCount}
          </span>
        )}
      </button>
    </footer>
  );
};
