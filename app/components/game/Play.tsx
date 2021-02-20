/* eslint-disable no-alert,no-restricted-globals */
import { lighten } from 'polished';
import { FC } from 'react';
import { useRouter } from 'next/router';
import { Palette } from '../../design/color';
import { CardColor, Player, Team } from '../../model';
import { voidFetch } from '../../utils/fetch';

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

  const onGuess = () => {
    if (!word) {
      alert(
        'Tap on a word to select it, and then tap here to confirm your guess.',
      );
    } else {
      const confirmed = confirm(`Are you sure you want to guess ${word}?`);
      if (confirmed) {
        const index = board.map((c) => c.word).indexOf(word);
        voidFetch({
          url: `${API_URL}/game/${game}/${player.name}/guess/${index}`,
          init: { method: 'PUT' },
          onSuccess: () => router.reload(),
          onError: () => alert('error making guess'),
        });
      }
    }
  };

  const onEndTurn = () => {
    const confirmed = confirm(
      `Are you sure you want to end ${turn} team's turn?`,
    );
    if (confirmed) {
      voidFetch({
        url: `${API_URL}/game/${game}/end-turn`,
        init: { method: 'PUT' },
        onSuccess: () => router.reload(),
        onError: () => alert('failed to end turn'),
      });
    }
  };

  return (
    <div
      css={`
        display: flex;
        flex-direction: column;
        align-items: center;
        & button {
          width: 100%;
        }
      `}
    >
      {!player.is_spy_master && player.team === turn && (
        <>
          <p>guess</p>
          <button
            onClick={onGuess}
            type="button"
            css={`
              background: transparent;
              border-color: ${Palette.neutral};
              border-radius: 0.25rem;
              padding: 0.5rem;
              margin: 0 0.5rem 0.5rem;
              width: 5rem;
              color: ${player.team === 'Blue' ? Palette.blue : Palette.red};
              :focus {
                outline: none;
              }
            `}
          >
            {word ?? '?'}
          </button>
        </>
      )}
      <button
        type="button"
        onClick={onEndTurn}
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
      >
        End Turn
      </button>
    </div>
  );
};
