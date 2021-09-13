import React, { FC } from 'react';
import { voidFetch } from '../../../../utils/fetch';
import { actionButton, actionModal } from './Action.styles';
import { useApiContext } from '../../../ApiContext';
import { currentTeam, GameState } from '../../../../model';
import { Modal, useModalControls } from '../../../../design/Modal';

export const EndTurn: FC<{ game: GameState }> = ({ game }) => {
  const apiContext = useApiContext();
  const { isOpen, open, close } = useModalControls();
  const endTurn = () => {
    close();
    voidFetch({
      apiContext,
      path: `/game/${game.name}/end-turn`,
      init: { method: 'PUT' },
    });
  };

  const team = currentTeam(game);

  return (
    <>
      <button type="button" onClick={open} className={actionButton}>
        End Turn
      </button>
      <Modal isOpen={isOpen} onRequestClose={close}>
        <div className={actionModal}>
          <span>{`End ${team} Team's turn?`}</span>
          <button className={actionButton} type="button" onClick={endTurn}>
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
