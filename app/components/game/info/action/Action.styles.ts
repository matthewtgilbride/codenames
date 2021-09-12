import { css } from '@emotion/css';
import { darken } from 'polished';
import { beginAt } from '../../../../design/responsive';
import { Palette } from '../../../../design/color';

export const action = css`
  display: flex;
  flex-direction: column;
  margin: 1rem;
  font-size: 0.5rem;
  ${beginAt(375)} {
    font-size: 0.75rem;
  }
  ${beginAt(768)} {
    font-size: 1rem;
  }
  & button {
    font-size: 0.5rem;
    ${beginAt(375)} {
      font-size: 0.75rem;
    }
    ${beginAt(768)} {
      font-size: 1rem;
    }
  }
`;

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

export const modalContainer = css`
  display: flex;
  flex-direction: column;
  align-items: center;
`;
