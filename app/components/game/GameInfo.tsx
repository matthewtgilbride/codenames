/* eslint-disable no-alert,no-restricted-globals */
import { lighten } from 'polished';
import { FC } from 'react';
import { useRouter } from 'next/router';
import { css } from '@emotion/css';
import { Palette } from '../../design/color';
import { GameState, Player } from '../../model';
import { voidFetch } from '../../utils/fetch';

export interface PlayProps {
  API_URL: string;
  word?: string;
  player: Player;
  game: GameState;
}

export const GameInfo: FC<PlayProps> = ({ game: { turn, name }, API_URL }) => {
  const router = useRouter();
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

  return (
    <div
      className={css`
        display: flex;
        flex-direction: column;
        align-items: center;
        & button {
          width: 100%;
        }
      `}
    >
      <button
        type="button"
        onClick={onEndTurn}
        className={css`
          background-color: ${Palette.neutral};
          padding: 0.5rem;
          border-radius: 0.25rem;
          margin: 0.5rem;
          width: 5rem;

          :hover {
            background-color: ${lighten(0.1, Palette.neutral)};
          }
        `}
      >
        End Turn
      </button>
    </div>
  );
};
