import { css } from '@emotion/css';
import { darken, lighten } from 'polished';
import { Team } from '../../../model';
import { Palette } from '../../../design/color';
import { buttonStyle } from '../../../design/button';
import { beginAt } from '../../../design/responsive';

export function styleContainer(color: Team) {
  return css`
    display: flex;
    border-radius: 0.5rem;
    padding: 0.5rem;
    margin: 1rem;
    font-size: 0.5rem;
    flex-direction: column;
    color: ${Palette.contrast};
    background-color: ${color === 'Blue' ? Palette.blue : Palette.red};

    h2 {
      margin: 0;
    }

    ul {
      display: flex;
      flex-wrap: wrap;
      padding-left: 0.75rem;
    }

    li {
      margin: 0.25rem 0;
      padding-right: 0.5rem;
    }

    button {
      ${buttonStyle};
      font-size: 0.5rem;
      background-color: ${Palette.light};
      cursor: pointer;
      :hover {
        background-color: ${darken(0.1, Palette.light)};
      }
      ${beginAt(375)} {
        font-size: 0.75rem;
      }

      ${beginAt(768)} {
        font-size: 1rem;
      }
    }

    ${beginAt(375)} {
      font-size: 0.75rem;
    }

    ${beginAt(768)} {
      font-size: 1rem;
    }
  `;
}

export const styleInput = css`
  display: flex;
  flex-direction: column;
  margin: 0.5rem 0;
`;

export const styleButton = (team: Team) => css`
  ${buttonStyle};
  color: ${Palette.contrast};
  display: block;
  margin: auto;
  background-color: ${team === 'Red' ? Palette.red : Palette.blue};
  :hover {
    background-color: ${lighten(
      0.1,
      team === 'Red' ? Palette.red : Palette.blue,
    )};
  }
`;
