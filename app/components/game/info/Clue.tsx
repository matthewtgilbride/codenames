import React, { ChangeEventHandler, FC, useState } from 'react';
import { css } from '@emotion/css';
import { darken } from 'polished';
import { Modal } from '../../../design/Modal';
import { Team } from '../../../model';
import { Palette } from '../../../design/color';
import { buttonStyle } from '../../../design/button';

export interface ClueProps {
  isOpen: boolean;
  team: Team;
}

export const Clue: FC<ClueProps> = ({ isOpen, team }) => {
  const [wordState, setWordState] = useState('');
  const onWordChange: ChangeEventHandler<HTMLInputElement> = (event) =>
    setWordState(event.target.value);
  const [amountState, setAmountState] = useState(1);
  const onAmountChange: ChangeEventHandler<HTMLInputElement> = (event) =>
    setAmountState(parseInt(event.target.value, 10));

  return (
    <Modal isOpen={isOpen}>
      <div className={container}>
        <div className={row}>
          <div>
            <label htmlFor="word">Word</label>
            <input id="word" onChange={onWordChange} value={wordState} />
          </div>
          <div>
            <label>For</label>
            <input
              type="number"
              onChange={onAmountChange}
              value={amountState.toString()}
            />
          </div>
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
  align-items: center;
  margin: auto;
  > div {
    display: flex;
    flex-direction: column;
    justify-content: center;
    margin: 0 0.5rem;
    :first-child {
      flex-basis: 50%;
      flex-grow: 1;
    }
    :last-child {
      max-width: 3rem;
    }
  }
  & input {
    padding: 0.25rem;
    max-width: 100%;
  }
  & label {
    font-size: 0.75rem;
    padding: 0.25rem 0;
  }
`;

function styleButton(team: Team) {
  return css`
    ${buttonStyle};
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
