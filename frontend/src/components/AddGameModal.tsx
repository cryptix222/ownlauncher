import React, { useState } from 'react';
import { X, Folder, AlertCircle } from 'lucide-react';

interface AddGameModalProps {
  isOpen: boolean;
  onClose: () => void;
  onAdd: (name: string, path: string) => Promise<void>;
}

export const AddGameModal: React.FC<AddGameModalProps> = ({ isOpen, onClose, onAdd }) => {
  const [name, setName] = useState('');
  const [path, setPath] = useState('');
  const [error, setError] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);

  if (!isOpen) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim() || !path.trim()) {
      setError('Both game title and executable path are required.');
      return;
    }
    try {
      setIsSubmitting(true);
      setError('');
      await onAdd(name.trim(), path.trim());
      setName('');
      setPath('');
      onClose();
    } catch (err: any) {
      setError(err?.message || 'Failed to add game.');
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center p-4"
      style={{ background: 'rgba(1, 4, 9, 0.7)' }}
    >
      <div className="absolute inset-0" onClick={onClose} />

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
          <span className="text-[12px] font-semibold uppercase tracking-wider" style={{ color: 'var(--text-primary)' }}>
            Add Game Manually
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

        {/* Form */}
        <form onSubmit={handleSubmit} className="p-4 space-y-4">
          {error && (
            <div
              className="p-2.5 rounded-[4px] flex items-start gap-2 text-[12px]"
              style={{
                background: 'rgba(248,81,73,0.1)',
                border: '1px solid rgba(248,81,73,0.3)',
                color: 'var(--status-error)',
              }}
            >
              <AlertCircle size={14} className="shrink-0 mt-0.5" />
              <span>{error}</span>
            </div>
          )}

          <div>
            <label className="text-[11px] font-semibold mb-1 block" style={{ color: 'var(--text-secondary)' }}>
              Game Title
            </label>
            <input
              type="text"
              value={name}
              onChange={e => setName(e.target.value)}
              placeholder="e.g. Cyberpunk 2077"
              className="w-full rounded-[4px] px-2.5 py-[6px] text-[12px] focus:outline-none transition-colors duration-100"
              style={{
                background: 'var(--bg-primary)',
                border: '1px solid var(--border-subtle)',
                color: 'var(--text-primary)',
                caretColor: 'var(--accent)',
              }}
              onFocus={e => { (e.currentTarget as HTMLInputElement).style.borderColor = 'var(--accent)'; }}
              onBlur={e =>  { (e.currentTarget as HTMLInputElement).style.borderColor = 'var(--border-subtle)'; }}
            />
          </div>

          <div>
            <label className="text-[11px] font-semibold mb-1 block" style={{ color: 'var(--text-secondary)' }}>
              Executable Path (.exe)
            </label>
            <div className="relative flex items-center">
              <input
                type="text"
                value={path}
                onChange={e => setPath(e.target.value)}
                placeholder={'C:\\Games\\MyGame\\game.exe'}
                className="w-full rounded-[4px] pl-2.5 pr-8 py-[6px] text-[12px] focus:outline-none transition-colors duration-100"
                style={{
                  background: 'var(--bg-primary)',
                  border: '1px solid var(--border-subtle)',
                  color: 'var(--text-primary)',
                  caretColor: 'var(--accent)',
                  fontFamily: 'ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace',
                }}
                onFocus={e => { (e.currentTarget as HTMLInputElement).style.borderColor = 'var(--accent)'; }}
                onBlur={e =>  { (e.currentTarget as HTMLInputElement).style.borderColor = 'var(--border-subtle)'; }}
              />
              <Folder size={12} className="absolute right-2.5 pointer-events-none" style={{ color: 'var(--text-muted)' }} />
            </div>
          </div>

          <div
            className="pt-3 flex items-center justify-end gap-2"
            style={{ borderTop: '1px solid var(--border-subtle)' }}
          >
            <button
              type="button"
              onClick={onClose}
              className="px-3 py-1.5 rounded-[4px] text-[12px] font-medium transition-colors duration-100"
              style={{
                background: 'var(--bg-primary)',
                border: '1px solid var(--border-subtle)',
                color: 'var(--text-secondary)',
              }}
              onMouseEnter={e => { (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-elevated)'; }}
              onMouseLeave={e => { (e.currentTarget as HTMLButtonElement).style.background = 'var(--bg-primary)'; }}
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={isSubmitting}
              className="px-4 py-1.5 rounded-[4px] text-[12px] font-semibold transition-colors duration-100 disabled:opacity-50"
              style={{
                background: 'var(--accent)',
                color: 'var(--accent-fg)',
              }}
              onMouseEnter={e => { if (!isSubmitting) (e.currentTarget as HTMLButtonElement).style.background = 'var(--accent-hover)'; }}
              onMouseLeave={e => { (e.currentTarget as HTMLButtonElement).style.background = 'var(--accent)'; }}
            >
              {isSubmitting ? 'Saving…' : 'Add Game'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
