import { writable, type Writable } from 'svelte/store';
import type { CombatantSetup } from './types';

export const sleep = (): Promise<void> => {
  return new Promise((resolve) => setTimeout(resolve));
};

export const calculateVariations = (cs: CombatantSetup[]): number => {
  let result = 1;
  for (const c of cs) {
    if (c.vary) {
      result *= Math.max(1, c.max_count! - c.min_count! + 1);
    }
  }
  return result;
};

export const localStorageStore = <T>(key: string, def: T): Writable<T> => {
  const val = localStorage.getItem(key);
  if (val) {
    try {
      def = JSON.parse(val);
    } catch {}
  }
  const store = writable(def);
  return {
    set(value) {
      store.set(value);
      localStorage.setItem(key, JSON.stringify(value));
    },
    update(updater) {
      store.update((v) => {
        const r = updater(v);
        localStorage.setItem(key, JSON.stringify(r));
        return r;
      });
    },
    subscribe(run, invalidate?) {
      return store.subscribe(run, invalidate);
    },
  };
};
