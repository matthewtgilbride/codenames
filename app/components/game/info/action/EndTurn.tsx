import React, { FC } from 'react';
import { voidFetch } from '../../../../utils/fetch';
import { actionButton } from './Action.styles';
import { useApiContext } from '../../../ApiContext';
import { currentTeam, GameState } from '../../../../model';
import { Modal, useModalControls } from '../../../../design/Modal';

export const EndTurn: FC<{ game: GameState }> = ({ game }) => {
  const apiContext = useApiContext();
  const { isOpen, open, close } = useModalControls();
  const endTurn = () => {
    voidFetch({
      apiContext,
      path: `/game/${game.name}/end-turn`,
      init: { method: 'PUT' },
      onSuccess: close,
      onError: close,
    });
  };

  const team = currentTeam(game);

  return (
    <>
      <button type="button" onClick={open} className={actionButton}>
        End Turn
      </button>
      <Modal isOpen={isOpen} onRequestClose={close}>
        {`Are you sure you want to end ${team} team's turn?`}
        <button type="button" onClick={endTurn}>
          Yes
        </button>
        <button type="button" onClick={close}>
          No
        </button>
      </Modal>
    </>
  );
};
