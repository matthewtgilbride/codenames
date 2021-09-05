import React, { ChangeEventHandler, FC, useState } from 'react';
import { css } from '@emotion/css';
import { darken } from 'polished';
import { Modal } from '../../../design/Modal';
import { Team } from '../../../model';
import { Palette } from '../../../design/color';
import { buttonStyle } from '../../../design/button';

export interface ClueProps {
  isOpen: boolean;
  onRequestClose: (event: React.MouseEvent | React.KeyboardEvent) => void;
  team: Team;
}

export const Clue: FC<ClueProps> = ({ isOpen, onRequestClose, team }) => {
  const [wordState, setWordState] = useState('');
  const onWordChange: ChangeEventHandler<HTMLInputElement> = (event) =>
    setWordState(event.target.value);
  const [amountState, setAmountState] = useState(1);
  const onAmountChange: ChangeEventHandler<HTMLInputElement> = (event) =>
    setAmountState(parseInt(event.target.value, 10));

  return (
    <Modal isOpen={isOpen} onRequestClose={onRequestClose}>
      <div className={container}>
        <div className={row}>
          <label htmlFor="word">
            <span>Word</span>
            <input id="word" onChange={onWordChange} value={wordState} />
          </label>
          <label htmlFor="for">
            <span>For</span>
            <input
              id="for"
              type="number"
              onChange={onAmountChange}
              value={amountState.toString()}
            />
          </label>
        </div>
        <div className={row}>
          <button type="submit" className={styleButton(team)}>
            Give Clue
          </button>
        </div>
      </div>
    </Modal>
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
