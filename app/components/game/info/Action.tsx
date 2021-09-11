import React, { FC } from 'react';
import { useRouter } from 'next/router';
import {
  currentTeam,
  currentTurn,
  GameState,
  getGuesses,
  isSpyMaster,
  Player,
} from '../../../model';
import { GuessLog } from './GuessLog';
import { Clue } from './Clue';
import { voidFetch } from '../../../utils/fetch';
import { useApiContext } from '../../ApiContext';
import { action, actionButton } from './Action.styles';

export interface ActionProps {
  game: GameState;
  player?: Player;
}

export const Action: FC<ActionProps> = ({ game, game: { board }, player }) => {
  const apiContext = useApiContext();
  const router = useRouter();
  const turn = currentTeam(game);
  const onEndTurn = () => {
    // eslint-disable-next-line no-restricted-globals,no-alert
    const confirmed = confirm(
      `Are you sure you want to end ${turn} team's turn?`,
    );
    if (confirmed) {
      voidFetch({
        apiContext,
        path: `/game/${game.name}/end-turn`,
        init: { method: 'PUT' },
        onSuccess: () => router.reload(),
      });
    }
  };

  const onLeave = () => {
    // eslint-disable-next-line no-restricted-globals,no-alert
    const confirmed = confirm(`Are you sure you want to leave the game?`);
    if (confirmed) {
      voidFetch({
        apiContext,
        path: `/game/${game.name}/${player?.name}/leave`,
        init: { method: 'PUT' },
        onSuccess: () => router.push(`/game/${game.name}`),
      });
    }
  };
  return (
    <div className={action}>
      {player &&
        isSpyMaster(player) &&
        currentTurn(game).type === 'Pending' && (
          <Clue game={game} spyMaster={player} />
        )}
      {player && currentTurn(game).type === 'Started' && (
        <button type="button" onClick={onEndTurn} className={actionButton}>
          End Turn
        </button>
      )}
      <GuessLog board={board} guesses={getGuesses(game)} />
      {player && (
        <button type="button" onClick={onLeave} className={actionButton}>
          Leave Game
        </button>
      )}
    </div>
  );
};
