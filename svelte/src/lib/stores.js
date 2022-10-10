import { writable } from "svelte/store";

export const ticksPerFrame = writable(1);
export const gridSize = writable(64);
export const showFps = writable(false);
export const universeTemplate = writable("Empty");
export const hidden = writable(true);
