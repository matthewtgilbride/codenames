import React, { FC } from 'react';
import { useRouter } from 'next/router';
import { voidFetch } from '../../../../utils/fetch';
import { actionButton } from './Action.styles';
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
    voidFetch({
      apiContext,
      path: `/game/${gameName}/${playerName}/leave`,
      init: { method: 'PUT' },
      onSuccess: () => router.push(`/game/${gameName}`),
      onError: close,
    });
  };

  return (
    <>
      <button type="button" onClick={open} className={actionButton}>
        Leave Game
      </button>
      <Modal isOpen={isOpen} onRequestClose={close}>
        Are you sure you want to leave the game?
        <button className={actionButton} type="button" onClick={leave}>
          Yes
        </button>
        <button className={actionButton} type="button" onClick={close}>
          No
        </button>
      </Modal>
    </>
  );
};
