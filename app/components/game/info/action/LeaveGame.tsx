import React, { FC } from 'react';
import { useRouter } from 'next/router';
import { voidFetch } from '../../../../utils/fetch';
import { actionButton, actionModal } from './Action.styles';
import { useApiContext } from '../../../ApiContext';
import { Modal, useModalControls } from '../../../../design/Modal';

export const LeaveGame: FC<{ gameName: string; playerName: string }> = ({
  gameName,
  playerName,
}) => {
  const apiContext = useApiContext();
  const router = useRouter();
  const { isOpen, open, close } = useModalControls();
  const leave = () => {
    close();
    voidFetch({
      apiContext,
      path: `/game/${gameName}/${playerName}/leave`,
      init: { method: 'PUT' },
      onSuccess: () => router.push(`/game/${gameName}`),
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
