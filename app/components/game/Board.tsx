import React, { FC } from 'react';
import { css } from '@emotion/css';
import { currentTeam, GameState, Player } from '../../model';
import { Card } from './Card';
import { Breakpoints } from '../../design/responsive';
import { voidFetch } from '../../utils/fetch';
import { useApiContext } from '../ApiContext';

export interface BoardProps {
  player?: Player;
  game: GameState;
}

export const Board: FC<BoardProps> = ({ player, game }) => {
  const team = currentTeam(game);
  const apiContext = useApiContext();
  const onGuess = (word: string) => () => {
    // eslint-disable-next-line no-restricted-globals,no-alert
    const confirmed = confirm(`Are you sure you want to guess ${word}?`);
    if (confirmed) {
      const index = game.board.map((c) => c.word).indexOf(word);
      voidFetch({
        apiContext,
        path: `/game/${game.name}/${player?.name}/guess/${index}`,
        init: { method: 'PUT' },
      });
    }
  };
  return (
    <div className={container}>
      {game.board.map((card) => (
        <Card
          key={card.word}
          card={card}
          player={player}
          team={team}
          onClick={onGuess(card.word)}
        />
      ))}
    </div>
  );
};

const { tabletPortrait } = Breakpoints;
const container = css`
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
