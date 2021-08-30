import { css } from '@emotion/css';
import { lighten } from 'polished';
import { beginAt, Breakpoints } from '../../design/responsive';
import { Palette } from '../../design/color';

const { tabletPortrait } = Breakpoints;

export const container = css`
  display: grid;
  grid-template-columns: repeat(3, 1fr);

  margin: 1rem auto;
  max-width: ${tabletPortrait}px;
`;

export const action = css`
  display: flex;
  flex-direction: column;
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
  background-color: ${Palette.neutral};
  padding: 0.5rem;
  border-radius: 0.25rem;
  margin: 0.5rem 0;
  width: 100%;
  :hover {
    background-color: ${lighten(0.1, Palette.neutral)};
  }
`;
