// color.ts
// Simple RGB / RGBA color system for UI + game use

// ---------- Types ----------
export type RGB = {
  r: number;
  g: number;
  b: number;
};

export type RGBA = RGB & {
  a: number;
};

// ---------- Core Helpers ----------
export function rgb(r: number, g: number, b: number): RGB {
  return { r, g, b };
}

export function rgba(r: number, g: number, b: number, a: number): RGBA {
  return { r, g, b, a };
}

// ---------- Converters ----------
export function toRgbString(c: RGB): string {
  return `rgb(${c.r}, ${c.g}, ${c.b})`;
}

export function toRgbaString(c: RGBA): string {
  return `rgba(${c.r}, ${c.g}, ${c.b}, ${c.a})`;
}

// ---------- Utility ----------
export function clamp(value: number, min = 0, max = 255): number {
  return Math.max(min, Math.min(max, value));
}

export function lerp(a: number, b: number, t: number): number {
  return a + (b - a) * t;
}

// ---------- Color Ops ----------
export function lighten(c: RGB, amount: number): RGB {
  return {
    r: clamp(c.r + 255 * amount),
    g: clamp(c.g + 255 * amount),
    b: clamp(c.b + 255 * amount),
  };
}

export function darken(c: RGB, amount: number): RGB {
  return {
    r: clamp(c.r - 255 * amount),
    g: clamp(c.g - 255 * amount),
    b: clamp(c.b - 255 * amount),
  };
}

// ---------- Predefined Palette (edit this for your style) ----------
export const COLORS = {
  fatigued: rgb(248, 113, 113),
  distracted: rgb(236, 175, 117),
  neutral: rgb(217, 231, 122),
  focus: rgb(52, 211, 153),
  flow: rgb(96, 165, 250),
};