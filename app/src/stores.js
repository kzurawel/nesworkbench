import { writable } from 'svelte/store';

export const colorItems = writable({
  palettes: [],
  paletteSets: []
});

export const tileItems = writable({
  tileSets: [],
  patternTables: []
});

export const screenItems = writable({
  metatiles: [],
  backgrounds: [],
  scenes: []
});
