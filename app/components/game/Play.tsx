/* eslint-disable no-alert,no-restricted-globals */
import { lighten } from 'polished';
import { FC } from 'react';
import { useRouter } from 'next/router';
import { Palette } from '../../design/color';
import { CardColor, Player, Team } from '../../model';

export interface PlayProps {
  game: string;
  board: CardColor[];
  player: Player;
  turn: Team;
  word?: string;
  API_URL: string;
}

export const Play: FC<PlayProps> = ({
  game,
  board,
  player,
  turn,
  word,
  API_URL,
}) => {
  const router = useRouter();
  const onClick = () => {
    if (!word) {
      alert(
        'Click on a word to select it, and then click here to confirm your guess',
      );
    } else {
      const index = board.map((c) => c.word).indexOf(word);
      const url = `${API_URL}/game/${game}/${player.name}/guess/${index}`;
      fetch(url, {
        method: 'PUT',
      })
        .then((response) => {
          if (response.ok) {
            router.reload();
          } else {
            alert('error making guess');
          }
        })
        .catch(() => alert('error making guess'));
    }
  };
  return (
    <div
      css={`
        display: flex;
        flex-direction: column;
        align-items: center;
      `}
    >
      {!player.is_spy_master && (
        <button
          onClick={onClick}
          type="button"
          css={`
            background: transparent;
            border-color: ${Palette.neutral};
            border-radius: 0.25rem;
            padding: 0.5rem;
            margin: 0.5rem;
            width: 5rem;
            color: ${player.team === 'Blue' ? Palette.blue : Palette.red};

            :focus {
              outline: none;
            }
          `}
        >
          {word ?? '(•_•)'}
        </button>
      )}
      <button
        css={`
          background-color: ${Palette.neutral};
          padding: 0.5rem;
          border-radius: 0.25rem;
          margin: 0.5rem;
          width: 5rem;

          :hover {
            background-color: ${lighten(0.1, Palette.neutral)};
          }
        `}
        type="button"
        onClick={() => {
          const confirmed = confirm(
            `Are you sure you want to end ${turn} team's turn?`,
          );
          if (confirmed) {
            const url = `${API_URL}/game/${game}/end-turn`;
            fetch(url, { method: 'PUT' })
              .then((response) => {
                if (response.ok) {
                  router.reload();
                } else {
                  alert('failed to end turn');
                }
              })
              .catch(() => alert('failed to end turn'));
          }
        }}
      >
        End Turn
      </button>
    </div>
  );
};
