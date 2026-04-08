import { defaultConfig } from './default-config.js';
import { normalizeConfig } from './config.js';

export const STORAGE_KEY = 'hiccup90:nas-dashboard';

export function loadConfig() {
  try {
    const raw = window.localStorage.getItem(STORAGE_KEY);

    if (!raw) {
      return normalizeConfig(defaultConfig);
    }

    return normalizeConfig(JSON.parse(raw));
  } catch {
    return normalizeConfig(defaultConfig);
  }
}

export function saveConfig(config) {
  window.localStorage.setItem(STORAGE_KEY, JSON.stringify(normalizeConfig(config)));
}
