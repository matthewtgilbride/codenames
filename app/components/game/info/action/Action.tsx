import React, { FC } from 'react';
import {
  currentTeam,
  currentTurn,
  GameState,
  getGuesses,
  isSpyMaster,
  Player,
} from '../../../../model';
import { GuessLog } from '../GuessLog';
import { StartTurn } from './StartTurn';
import { action } from './Action.styles';
import { EndTurn } from './EndTurn';
import { LeaveGame } from './LeaveGame';

export interface ActionProps {
  game: GameState;
  player?: Player;
}

export const Action: FC<ActionProps> = ({ game, game: { board }, player }) => (
  <div className={action}>
    {player &&
      isSpyMaster(player) &&
      currentTeam(game) === player.team &&
      currentTurn(game).type === 'Pending' && (
        <StartTurn game={game} spyMaster={player} />
      )}
    {player && currentTurn(game).type === 'Started' && <EndTurn game={game} />}
    <GuessLog board={board} guesses={getGuesses(game)} />
    {player && <LeaveGame playerName={player.name} gameName={game.name} />}
  </div>
);
