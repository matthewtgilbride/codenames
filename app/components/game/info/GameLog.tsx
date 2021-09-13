import React, { FC } from 'react';
import { css } from '@emotion/css';
import { CardColor, CardType, GameState, StartedTurn } from '../../../model';
import { Palette } from '../../../design/color';
import { CardColorMap } from '../Card';

export type GameLogProps = Pick<GameState, 'board' | 'turns'>;

export const GameLog: FC<GameLogProps> = ({ board, turns }) => (
  <div className={container}>
    <h4>Game Log</h4>
    {turns.map((t, index) => {
      if (t.type === 'Started') {
        const {
          data: { guesses, spymaster, clue },
        } = t as StartedTurn;
        return (
          <div
            // eslint-disable-next-line react/no-array-index-key
            key={index}
            className={css`
              border-radius: 0.5rem;
              border-bottom: 1px solid ${Palette.light};
              border-top: 1px solid ${Palette.light};
            `}
          >
            <ul>
              {guesses.map((guess) => (
                <Guess key={guess[1]} guess={guess} board={board} />
              ))}
            </ul>
            <Clue spymaster={spymaster} clue={clue} />
          </div>
        );
      }
      const color = t.data === 'Blue' ? Palette.blue : Palette.red;
      return (
        <div
          className={css`
            border-radius: 0.5rem;
            border-bottom: 1px solid ${Palette.light};
            border-top: 1px solid ${Palette.light};
            padding: 1rem 0;
          `}
        >
          <span>Waiting for </span>
          <span style={{ color }}>{t.data}</span>
          <span> team Spymaster</span>
        </div>
      );
    })}
  </div>
);

const Guess: FC<{
  board: CardColor[];
  guess: StartedTurn['data']['guesses'][number];
}> = ({ board, guess: [player, index] }) => {
  const { word, color: cardColor } = board[index];
  const color = CardColorMap[cardColor as CardType];
  return (
    <li>
      <span>{player.name} guessed </span>
      <span style={{ color }}>{word}</span>
    </li>
  );
};

const Clue: FC<Pick<StartedTurn['data'], 'spymaster' | 'clue'>> = ({
  spymaster,
  clue,
}) => {
  const color = spymaster.team === 'Blue' ? Palette.blue : Palette.red;
  return (
    <div
      className={css`
        font-weight: bold;
        padding-bottom: 0.75rem;
        margin: 0 auto;
      `}
    >
      <span style={{ color }}>{`${spymaster.name}`}</span>
      <span>{` said "${clue[0]}" for ${clue[1]}`}</span>
    </div>
  );
};
const container = css`
  display: flex;
  flex-direction: column;
  color: ${Palette.light};
  & h4 {
    margin: 0;
    font-size: 1rem;
  }
  & ul {
    padding: 0.5rem;
  }
  & li {
    margin-bottom: 0.25rem;
  }
`;
