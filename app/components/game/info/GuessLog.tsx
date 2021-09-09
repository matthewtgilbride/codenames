import React, { FC } from 'react';
import { css } from '@emotion/css';
import { GameState } from '../../../model';
import { Palette } from '../../../design/color';

export type GuessLogProps = Pick<GameState, 'board'> & { guesses: number[] };

export const GuessLog: FC<GuessLogProps> = ({ board, guesses }) => (
  <div className={container}>
    <h4>Game Log</h4>
    <ul>
      {guesses
        .slice()
        .reverse()
        .map((g) => (
          <li key={g}>{board[g].word}</li>
        ))}
    </ul>
  </div>
);

const container = css`
  display: flex;
  flex-direction: column;
  color: ${Palette.light};
  & h4 {
    font-weight: bold;
    margin: 0.5rem 0;
  }
  & li {
    margin: 0.25rem 0;
  }
`;
