export interface ReaderSettings {
  theme: "light" | "dark" | "system";
  fontSize: number;
  lineHeight: number;
  fontFamily: "system" | "serif" | "mono";
  alwaysOnTop: boolean;
}

export const DEFAULT_READER_SETTINGS: ReaderSettings = {
  theme: "system",
  fontSize: 17,
  lineHeight: 1.75,
  fontFamily: "system",
  alwaysOnTop: false,
};