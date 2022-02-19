import { transparentize } from 'polished';

export const Palette = {
  light: '#f2f5f3',
  contrast: '#000000',
  neutral: '#fefcbd',
  death: '#3d3d3b',
  red: '#db8590',
  blue: '#95c8e5',
  gray: '#707677',
};

export const overlayColor = transparentize(0.5, Palette.contrast);
