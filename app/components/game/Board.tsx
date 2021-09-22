import React, { FC, useState } from 'react';
import { css } from '@emotion/css';
import { currentTurn, getGuesses, Player } from '../../model';
import { Card } from './Card';
import { Breakpoints } from '../../design/responsive';
import { voidFetch } from '../../utils/fetch';
import { useApiContext } from '../ApiContext';
import { Modal, useModalControls } from '../../design/Modal';
import { actionButton, actionModal } from './info/action/Action.styles';
import { useGameContext } from './GameContext';

export interface BoardProps {
  player?: Player;
}

export const Board: FC<BoardProps> = ({ player }) => {
  const { game, setGame } = useGameContext();
  const apiContext = useApiContext();
  const turn = currentTurn(game);
  const { isOpen, open, close } = useModalControls();
  const [word, setWord] = useState<string | undefined>();

  const onGuess = (selectedWord: string) => () => {
    setWord(selectedWord);
    open();
  };

  const confirmGuess = () => {
    close();
    const index = game.board.map((c) => c.word).indexOf(word as string);
    voidFetch({
      apiContext,
      path: `/game/${game.name}/${player?.name}/guess/${index}`,
      init: { method: 'PUT' },
      onSuccess: (r) => {
        r.json().then((g) => setGame(g));
      },
    });
  };

  const guesses = getGuesses(game);
  return (
    <>
      <div className={container}>
        {game.board.map((card, index) => (
          <Card
            key={card.word}
            card={card}
            player={player}
            turn={turn}
            onClick={onGuess(card.word)}
            guessIndex={guesses.indexOf(index)}
          />
        ))}
      </div>
      <Modal isOpen={isOpen} onRequestClose={close}>
        <div className={actionModal}>
          <span>Guess {word}?</span>
          <button className={actionButton} type="button" onClick={confirmGuess}>
            Yes
          </button>
          <button className={actionButton} type="button" onClick={close}>
            No
          </button>
        </div>
      </Modal>
    </>
  );
};

const { tabletPortrait } = Breakpoints;
const container = css`
  display: grid;
  padding: 0.5rem 0;
  margin: auto;
  max-width: ${tabletPortrait}px;
  text-align: center;
  grid-template-columns: 1fr 1fr 1fr 1fr 1fr;
  grid-row-gap: 0.5rem;
  grid-column-gap: 0.5rem;
  max-width: ${tabletPortrait}px;
`;
