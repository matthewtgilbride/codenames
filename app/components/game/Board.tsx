import React, { FC, useState } from 'react';
import { css } from '@emotion/css';
import { currentTeam, GameState, Player } from '../../model';
import { Card } from './Card';
import { Breakpoints } from '../../design/responsive';
import { voidFetch } from '../../utils/fetch';
import { useApiContext } from '../ApiContext';
import { Modal, useModalControls } from '../../design/Modal';
import { actionButton } from './info/action/Action.styles';

export interface BoardProps {
  player?: Player;
  game: GameState;
}

export const Board: FC<BoardProps> = ({ player, game }) => {
  const team = currentTeam(game);
  const apiContext = useApiContext();
  const { isOpen, open, close } = useModalControls();
  const [word, setWord] = useState<string | undefined>();

  const onGuess = (selectedWord: string) => () => {
    setWord(selectedWord);
    open();
  };

  const confirmGuess = () => {
    const index = game.board.map((c) => c.word).indexOf(word as string);
    voidFetch({
      apiContext,
      path: `/game/${game.name}/${player?.name}/guess/${index}`,
      init: { method: 'PUT' },
      onSuccess: close,
      onError: close,
    });
  };
  return (
    <>
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
      <Modal isOpen={isOpen} onRequestClose={close}>
        Are you sure you want to guess {word}?
        <button className={actionButton} type="button" onClick={confirmGuess}>
          Yes
        </button>
        <button className={actionButton} type="button" onClick={close}>
          No
        </button>
      </Modal>
    </>
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
