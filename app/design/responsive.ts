export type ResponsiveDeviceTypes =
  | 'phone'
  | 'phoneMd'
  | 'phoneLg'
  | 'tabletPortrait'
  | 'tabletLandscape'
  | 'laptop'
  | 'desktop'
  | '4K';

export type ResponsiveBreakpoints = { [key in ResponsiveDeviceTypes]: number };

export const Breakpoints: ResponsiveBreakpoints = {
  phone: 320,
  phoneMd: 375,
  phoneLg: 425,
  tabletPortrait: 768,
  tabletLandscape: 1024,
  laptop: 1260,
  desktop: 1440,
  '4K': 2560,
};

export const beginAt = (breakpoint: number): string =>
  `@media (min-width: ${breakpoint}px)`;

export const endAt = (breakpoint: number): string =>
  `@media (max-width: ${breakpoint - 1}px`;

export const between = (start: number, end: number): string =>
  `${beginAt(start)} and ${endAt(end)}`;
