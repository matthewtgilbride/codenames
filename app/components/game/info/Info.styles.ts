import { css } from '@emotion/css';
import { Breakpoints } from '../../../design/responsive';

const { tabletPortrait } = Breakpoints;

export const container = css`
  display: grid;
  grid-template-columns: 1fr 2fr 1fr;

  margin: 1rem auto;
  max-width: ${tabletPortrait}px;
`;
