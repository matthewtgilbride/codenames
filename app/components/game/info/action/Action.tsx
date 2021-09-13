import React, { FC } from 'react';
import { GameState } from '../../../../model';
import { GameLog } from '../GameLog';
import { action } from './Action.styles';

export interface ActionProps {
  game: GameState;
}

export const Action: FC<ActionProps> = ({ game, game: { board } }) => (
  <div className={action}>
    <GameLog board={board} turns={game.turns} />
  </div>
);
