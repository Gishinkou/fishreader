import type { ReaderSettings } from "../types/settings";

interface Props {
  settings: ReaderSettings;
  onChange: (s: ReaderSettings) => void;
  onClose: () => void;
}

const FONT_MIN = 14;
const FONT_MAX = 24;

export default function SettingsPanel({ settings, onChange, onClose }: Props) {
  const update = (patch: Partial<ReaderSettings>) =>
    onChange({ ...settings, ...patch });

  return (
    <div className="settings-panel">
      <div className="toolbar" style={{ marginBottom: 8 }}>
        <button className="icon-btn" onClick={onClose} title="Close">
          ←
        </button>
        <div className="crumb">Settings</div>
      </div>

      <div className="settings-row">
        <label>Theme</label>
        <div className="controls">
          {(["light", "dark", "system"] as const).map((t) => (
            <button
              key={t}
              className={`chip ${settings.theme === t ? "active" : ""}`}
              onClick={() => update({ theme: t })}
            >
              {t}
            </button>
          ))}
        </div>
      </div>

      <div className="settings-row">
        <label>Font size</label>
        <div className="controls">
          <button
            className="chip"
            onClick={() =>
              update({
                fontSize: Math.max(FONT_MIN, settings.fontSize - 1),
              })
            }
          >
            A-
          </button>
          <span style={{ minWidth: 30, textAlign: "center" }}>
            {settings.fontSize}px
          </span>
          <button
            className="chip"
            onClick={() =>
              update({
                fontSize: Math.min(FONT_MAX, settings.fontSize + 1),
              })
            }
          >
            A+
          </button>
        </div>
      </div>

      <div className="settings-row">
        <label>Line height</label>
        <div className="controls">
          {[1.5, 1.75, 2].map((lh) => (
            <button
              key={lh}
              className={`chip ${settings.lineHeight === lh ? "active" : ""}`}
              onClick={() => update({ lineHeight: lh })}
            >
              {lh}
            </button>
          ))}
        </div>
      </div>

      <div className="settings-row">
        <label>Always on top</label>
        <div className="controls">
          <button
            className={`chip ${settings.alwaysOnTop ? "active" : ""}`}
            onClick={() => update({ alwaysOnTop: !settings.alwaysOnTop })}
          >
            {settings.alwaysOnTop ? "On" : "Off"}
          </button>
        </div>
      </div>
    </div>
  );
}