import React, { FC } from 'react';
import { css } from '@emotion/css';
import {
  CardColor,
  CardType,
  GameState,
  StartedTurn,
  Turn,
} from '../../../model';
import { Palette } from '../../../design/color';
import { CardColorMap } from '../Card';
import { beginAt } from '../../../design/responsive';

export type GameLogProps = Pick<GameState, 'board' | 'turns'>;

export const GameLog: FC<GameLogProps> = ({ board, turns }) => (
  <div className={container}>
    <h4>Game Log</h4>
    {turns.map((turn, index) => (
      // eslint-disable-next-line react/no-array-index-key
      <div key={index}>
        <TurnContent turn={turn} board={board} />
      </div>
    ))}
  </div>
);

const TurnContent: FC<{ turn: Turn; board: CardColor[] }> = ({
  turn,
  board,
}) => {
  if (turn.type === 'Started') {
    const {
      data: { guesses, spymaster, clue },
    } = turn;
    return (
      <>
        <ul>
          {guesses.map((guess) => (
            <Guess key={guess[1]} guess={guess} board={board} />
          ))}
        </ul>
        <Clue spymaster={spymaster} clue={clue} />
      </>
    );
  }

  const color = turn.data === 'Blue' ? Palette.blue : Palette.red;
  return (
    <>
      <span>Waiting for </span>
      <span style={{ color }}>{turn.data}</span>
      <span> team Spymaster</span>
    </>
  );
};

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
    <div>
      <span style={{ color }}>{`${spymaster.name}`}</span>
      <span>{` said "${clue[0]}" for ${clue[1]}`}</span>
    </div>
  );
};

const container = css`
  display: flex;
  flex-direction: column;
  font-size: 0.5rem;
  ${beginAt(375)} {
    font-size: 0.75rem;
  }
  ${beginAt(768)} {
    font-size: 1rem;
  }
  color: ${Palette.light};
  & h4 {
    margin: 0;
    font-size: 1rem;
  }

  > div {
    border-radius: 0.5rem;
    border-bottom: 1px solid ${Palette.light};
    border-top: 1px solid ${Palette.light};
    padding: 0.5rem 0;
    & li {
      margin-bottom: 0.25rem;
      :last-child {
        margin-bottom: 0.5rem;
      }
    }
  }
`;
