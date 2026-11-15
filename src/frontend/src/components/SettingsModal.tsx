import React from 'react';
import { X, Palette, Monitor, RotateCw, Check } from 'lucide-react';
import type { Config } from '../types/launcher';

interface SettingsModalProps {
  isOpen: boolean;
  onClose: () => void;
  config: Config;
  onUpdateConfig: (newCfg: Config) => void;
  onRescan: () => void;
  isScanning: boolean;
}

const THEMES = [
  { id: 'dark',     name: 'GitHub Dark',       accent: '#2f81f7' },
  { id: 'midnight', name: 'Midnight Blue',      accent: '#0284c7' },
  { id: 'zinc',     name: 'Graphite Zinc',      accent: '#10b981' },
  { id: 'nord',     name: 'Nordic Frost',       accent: '#88c0d0' },
  { id: 'amber',    name: 'Industrial Amber',   accent: '#d97706' },
];

export const SettingsModal: React.FC<SettingsModalProps> = ({
  isOpen,
  onClose,
  config,
  onUpdateConfig,
  onRescan,
  isScanning,
}) => {
  if (!isOpen) return null;

  const handleToggle = (key: keyof Config) => {
    onUpdateConfig({ ...config, [key]: !config[key] });
  };

  const handleThemeChange = (themeId: string) => {
    document.documentElement.setAttribute('data-theme', themeId);
    try { localStorage.setItem('ownlauncher_theme', themeId); } catch (_) {}
    onUpdateConfig({ ...config, theme: themeId });
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center p-4"
      style={{ background: 'rgba(1, 4, 9, 0.7)' }}
    >
      {/* Backdrop */}
      <div className="absolute inset-0" onClick={onClose} />

      {/* Modal */}
      <div
        className="relative w-full max-w-sm overflow-hidden view-fade"
        style={{
          background: 'var(--bg-surface)',
          border: '1px solid var(--border-subtle)',
          borderRadius: '6px',
        }}
      >
        {/* Header */}
        <div
          className="px-4 py-2.5 flex items-center justify-between"
          style={{
            background: 'var(--bg-primary)',
            borderBottom: '1px solid var(--border-subtle)',
          }}
        >
          <span
            className="text-[12px] font-semibold uppercase tracking-wider"
            style={{ color: 'var(--text-primary)' }}
          >
            Settings
          </span>
          <button
            onClick={onClose}
            className="p-1 rounded-[3px] transition-colors duration-100"
            style={{ color: 'var(--text-muted)' }}
            onMouseEnter={e => {
              (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-hover)';
              (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-primary)';
            }}
            onMouseLeave={e => {
              (e.currentTarget as HTMLButtonElement).style.background = 'transparent';
              (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-muted)';
            }}
          >
            <X size={14} />
          </button>
        </div>

        {/* Body */}
        <div className="p-4 space-y-5 max-h-[72vh] overflow-y-auto">

          {/* Theme */}
          <section className="space-y-2">
            <label
              className="flex items-center gap-1.5 text-[11px] font-semibold uppercase tracking-wider"
              style={{ color: 'var(--text-secondary)' }}
            >
              <Palette size={12} style={{ color: 'var(--accent)' }} />
              Theme
            </label>
            <div className="grid grid-cols-2 gap-1.5">
              {THEMES.map(t => {
                const isSelected = config.theme === t.id;
                return (
                  <button
                    key={t.id}
                    onClick={() => handleThemeChange(t.id)}
                    className="p-2 rounded-[4px] flex items-center justify-between text-[12px] transition-colors duration-100"
                    style={{
                      background: isSelected ? 'var(--bg-elevated)' : 'var(--bg-primary)',
                      color: isSelected ? 'var(--text-primary)' : 'var(--text-secondary)',
                      fontWeight: isSelected ? 600 : 400,
                    }}
                    onMouseEnter={e => {
                      if (!isSelected) {
                        (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-hover)';
                        (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-primary)';
                      }
                    }}
                    onMouseLeave={e => {
                      if (!isSelected) {
                        (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-primary)';
                        (e.currentTarget as HTMLButtonElement).style.color = 'var(--text-secondary)';
                      }
                    }}
                  >
                    <div className="flex items-center gap-2">
                      <span
                        className="w-2.5 h-2.5 rounded-full shrink-0"
                        style={{ background: t.accent }}
                      />
                      <span>{t.name}</span>
                    </div>
                    {isSelected && <Check size={12} style={{ color: 'var(--accent)' }} />}
                  </button>
                );
              })}
            </div>
          </section>

          {/* Launch Behavior */}
          <section className="space-y-2">
            <label
              className="flex items-center gap-1.5 text-[11px] font-semibold uppercase tracking-wider"
              style={{ color: 'var(--text-secondary)' }}
            >
              <Monitor size={12} style={{ color: 'var(--accent)' }} />
              Launch Behavior
            </label>
            <div className="space-y-1">
              <ToggleRow
                label="Minimize launcher on game start"
                checked={config.minimize_on_launch}
                onChange={() => handleToggle('minimize_on_launch')}
              />
              <ToggleRow
                label="Close launcher on game start"
                checked={config.close_on_launch}
                onChange={() => handleToggle('close_on_launch')}
              />
            </div>
          </section>

          {/* Interface */}
          <section className="space-y-2">
            <label
              className="text-[11px] font-semibold uppercase tracking-wider block"
              style={{ color: 'var(--text-secondary)' }}
            >
              Interface
            </label>
            <div className="space-y-1">
              <ToggleRow
                label="Show platform badges"
                checked={config.show_platform_badges}
                onChange={() => handleToggle('show_platform_badges')}
              />
              <ToggleRow
                label="Compact sidebar view"
                checked={config.compact_sidebar}
                onChange={() => handleToggle('compact_sidebar')}
              />
              <ToggleRow
                label="Auto-scan on startup"
                checked={config.auto_scan_on_startup}
                onChange={() => handleToggle('auto_scan_on_startup')}
              />
            </div>
          </section>

          {/* Library */}
          <section className="pt-1" style={{ borderTop: '1px solid var(--border-subtle)' }}>
            <button
              onClick={onRescan}
              disabled={isScanning}
              className="w-full py-1.5 rounded-[4px] text-[12px] font-medium flex items-center justify-center gap-2 transition-colors duration-100 disabled:opacity-40"
              style={{ background: 'var(--bg-primary)', color: 'var(--text-primary)' }}
              onMouseEnter={e => { (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-elevated)'; }}
              onMouseLeave={e => { (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-primary)'; }}
            >
              <RotateCw size={12} className={isScanning ? 'animate-spin' : ''} style={{ color: isScanning ? 'var(--accent)' : undefined }} />
              {isScanning ? 'Scanning library…' : 'Rescan Game Library'}
            </button>
          </section>
        </div>
      </div>
    </div>
  );
};

const ToggleRow: React.FC<{ label: string; checked: boolean; onChange: () => void }> = ({
  label,
  checked,
  onChange,
}) => (
  <label
    className="flex items-center justify-between p-2 rounded-[4px] cursor-pointer transition-colors duration-100"
    style={{ background: 'var(--bg-primary)' }}
    onMouseEnter={e => { (e.currentTarget as HTMLLabelElement).style.background = 'var(--bg-hover)'; }}
    onMouseLeave={e => { (e.currentTarget as HTMLLabelElement).style.background = 'var(--bg-primary)'; }}
  >
    <span className="text-[12px]" style={{ color: 'var(--text-secondary)' }}>{label}</span>
    <input
      type="checkbox"
      checked={checked}
      onChange={onChange}
      className="w-3.5 h-3.5 rounded-[2px] cursor-pointer"
      style={{ accentColor: 'var(--accent)' }}
    />
  </label>
);
