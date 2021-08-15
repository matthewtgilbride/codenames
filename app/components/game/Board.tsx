import React, { FC, MouseEventHandler } from 'react';
import { css } from '@emotion/css';
import { CardColor, Player, Team } from '../../model';
import { Card } from './Card';
import { Breakpoints } from '../../design/responsive';

export interface BoardProps {
  player?: Player;
  onGuess: (word: string) => MouseEventHandler;
  turn: Team;
  board: CardColor[];
}

export const Board: FC<BoardProps> = ({ player, onGuess, turn, board }) => (
  <div className={styleBoard()}>
    {board.map((card) => (
      <Card
        key={card.word}
        card={card}
        player={player}
        turn={turn}
        onClick={onGuess(card.word)}
      />
    ))}
  </div>
);

const { tabletPortrait } = Breakpoints;
function styleBoard() {
  return css`
    display: grid;
    padding: 1rem 0;
    margin: auto;
    max-width: ${tabletPortrait}px;
    text-align: center;
    grid-template-columns: 1fr 1fr 1fr 1fr 1fr;
    grid-row-gap: 1rem;
    grid-column-gap: 1rem;
    max-width: ${tabletPortrait}px;
  `;
}
