/* eslint-disable no-alert,no-restricted-globals */
import { lighten } from 'polished';
import { FC } from 'react';
import { useRouter } from 'next/router';
import { css } from '@emotion/css';
import { Palette } from '../../design/color';
import { currentTeam, GameState, getGuesses, Player } from '../../model';
import { voidFetch } from '../../utils/fetch';
import { GuessLog } from './GuessLog';
import { beginAt } from '../../design/responsive';

export interface PlayProps {
  API_URL: string;
  word?: string;
  player?: Player;
  game: GameState;
}

export const GameInfo: FC<PlayProps> = ({
  player,
  game,
  game: { name, board },
  API_URL,
}) => {
  const router = useRouter();
  const turn = currentTeam(game);
  const guesses = getGuesses(game);
  const onEndTurn = () => {
    const confirmed = confirm(
      `Are you sure you want to end ${turn} team's turn?`,
    );
    if (confirmed) {
      voidFetch({
        url: `${API_URL}/game/${name}/end-turn`,
        init: { method: 'PUT' },
        onSuccess: () => router.reload(),
        onError: () => alert('failed to end turn'),
      });
    }
  };

  const onLeave = () => {
    const confirmed = confirm(`Are you sure you want to leave the game?`);
    if (confirmed) {
      voidFetch({
        url: `${API_URL}/game/${name}/${player?.name}/leave`,
        init: { method: 'PUT' },
        onSuccess: () => router.push(`/game/${name}`),
        onError: () => alert('failed to leave game'),
      });
    }
  };

  return (
    <div
      className={css`
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
      `}
    >
      <h3>{turn} Team&apos;s Turn</h3>
      {player && (
        <button
          type="button"
          onClick={onEndTurn}
          className={css`
            background-color: ${Palette.neutral};
            padding: 0.5rem;
            border-radius: 0.25rem;
            margin: 0.5rem 0;
            width: 100%;
            :hover {
              background-color: ${lighten(0.1, Palette.neutral)};
            }
          `}
        >
          End Turn
        </button>
      )}
      <GuessLog board={board} guesses={guesses} />
      {player && (
        <button
          type="button"
          onClick={onLeave}
          className={css`
            background-color: ${Palette.neutral};
            padding: 0.5rem;
            border-radius: 0.25rem;
            margin: 0.5rem 0;
            width: 100%;
            :hover {
              background-color: ${lighten(0.1, Palette.neutral)};
            }
          `}
        >
          Leave Game
        </button>
      )}
    </div>
  );
};
