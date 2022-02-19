import React, { FC } from 'react';
import { useNavigate } from 'react-router';
import { voidFetch } from '../../../../utils/fetch';
import { actionButton, actionModal } from './Action.styles';
import { useApiContext } from '../../../ApiContext';
import { Modal, useModalControls } from '../../../../design/Modal';
import { useGameContext } from '../../GameContext';

export const LeaveGame: FC<{ playerName: string }> = ({ playerName }) => {
  const {
    game: { name },
  } = useGameContext();
  const apiContext = useApiContext();
  const navigate = useNavigate();
  const { isOpen, open, close } = useModalControls();
  const leave = () => {
    close();
    voidFetch({
      apiContext,
      path: `/game/${name}/${playerName}/leave`,
      init: { method: 'PUT' },
      onSuccess: () => navigate(`/game/${name}`),
      remainLoadingOnSuccess: true,
    });
  };

  return (
    <>
      <button type="button" onClick={open} className={actionButton}>
        Leave Game
      </button>
      <Modal isOpen={isOpen} onRequestClose={close}>
        <div className={actionModal}>
          <span>Leave the game?</span>
          <button className={actionButton} type="button" onClick={leave}>
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
