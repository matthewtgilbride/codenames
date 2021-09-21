import { css } from '@emotion/css';
import { darken } from 'polished';
import { Palette } from '../../../../design/color';

export const actionButton = css`
  background-color: ${Palette.light};
  padding: 0.5rem;
  border-radius: 0.25rem;
  width: 100%;
  max-width: 12rem;
  margin: 0.5rem auto;
  :hover {
    background-color: ${darken(0.1, Palette.light)};
  }
`;

export const actionModal = css`
  display: flex;
  flex-direction: column;
  align-items: center;
`;
