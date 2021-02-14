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
