import React, { FC } from 'react';
import { css } from '@emotion/css';
import { darken } from 'polished';
import { Modal, useModalControls } from '../../../../design/Modal';
import { currentTeam, GameState, Player, Team } from '../../../../model';
import { Palette } from '../../../../design/color';
import { buttonStyle } from '../../../../design/button';
import { voidFetch } from '../../../../utils/fetch';
import { useApiContext } from '../../../ApiContext';
import { actionButton } from './Action.styles';
import { useInputState } from '../../../../hooks/useInputState';

export interface ClueProps {
  game: GameState;
  spyMaster: Player;
}

export const StartTurn: FC<ClueProps> = ({ game, spyMaster }) => {
  const { isOpen, open, close } = useModalControls();
  const [word, onWordChange] = useInputState();
  const [amount, onAmountChange] = useInputState();

  const team = currentTeam(game);

  const apiContext = useApiContext();
  const onSubmit = () => {
    close();
    voidFetch({
      apiContext,
      path: `/game/${game.name}/${spyMaster.name}/start-turn`,
      init: {
        method: 'PUT',
        body: JSON.stringify({ word, amount: parseInt(amount, 10) }),
      },
    });
  };

  return (
    <>
      <button type="button" onClick={open} className={actionButton}>
        Start Turn
      </button>
      <Modal isOpen={isOpen} onRequestClose={close}>
        <div className={container}>
          <div className={row}>
            <label htmlFor="word">
              <span>Word</span>
              <input id="word" value={word} onChange={onWordChange} />
            </label>
            <label htmlFor="for">
              <span>For</span>
              <input
                id="for"
                type="number"
                value={amount}
                onChange={onAmountChange}
              />
            </label>
          </div>
          <div className={row}>
            <button
              type="submit"
              className={styleButton(team)}
              onClick={onSubmit}
            >
              Give Clue
            </button>
          </div>
        </div>
      </Modal>
    </>
  );
};

const container = css`
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  height: 100%;
`;

const row = css`
  display: flex;
  flex-wrap: wrap;
  height: 100%;
  width: 100%;
  align-items: center;
  margin: auto;
  & label {
    display: flex;
    flex-direction: column;
    justify-content: center;
    margin: 0 0.5rem;
    font-size: 0.75rem;
    padding: 0.25rem 0;
  }
  & input {
    padding: 0.25rem;
    max-width: 100%;
  }
  & label:first-child {
    flex-basis: 50%;
    flex-grow: 1;
  }
  & label:last-child {
    max-width: 3rem;
  }
`;

function styleButton(team: Team) {
  return css`
    ${buttonStyle};
    margin: auto;
    padding: 0.5rem;
    background-color: ${team === 'Red' ? Palette.red : Palette.blue};
    :hover {
      background-color: ${darken(
        0.1,
        team === 'Red' ? Palette.red : Palette.blue,
      )};
    }
  `;
}
