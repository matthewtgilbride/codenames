import React, { ChangeEventHandler, FC, useState } from 'react';
import ReactModal, { Styles } from 'react-modal';
import { css } from '@emotion/css';
import { transparentize } from 'polished';
import { Palette } from '../../../design/color';

export interface ClueProps {
  isOpen: boolean;
}

export const Clue: FC<ClueProps> = ({ isOpen }) => {
  const [wordState, setWordState] = useState('');
  const onWordChange: ChangeEventHandler<HTMLInputElement> = (event) =>
    setWordState(event.target.value);
  const [amountState, setAmountState] = useState(1);
  const onAmountChange: ChangeEventHandler<HTMLInputElement> = (event) =>
    setAmountState(parseInt(event.target.value, 10));

  return (
    <ReactModal isOpen={isOpen} style={style} closeTimeoutMS={500}>
      <div className={contentStyle}>
        <input onChange={onWordChange} value={wordState} />
        <input onChange={onAmountChange} value={amountState.toString()} />
      </div>
    </ReactModal>
  );
};

const contentStyle = css`
  display: flex;
  flex-direction: column;
  height: 100%;
`;

const style: Styles = {
  overlay: {
    backgroundColor: transparentize(0.5, Palette.contrast),
  },
  content: {
    backgroundColor: Palette.neutral,
  },
};
