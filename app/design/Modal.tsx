import { css, Global } from '@emotion/react';
import React, { FC } from 'react';
import ReactModal from 'react-modal';
import { transparentize } from 'polished';
import { Palette } from './color';

export interface ModalProps {
  isOpen: boolean;
}

export const Modal: FC<ModalProps> = ({ isOpen, children }) => (
  <>
    <Global styles={modalStyle} />
    <ReactModal isOpen={isOpen} closeTimeoutMS={500}>
      {children}
    </ReactModal>
  </>
);

const modalStyle = css`
  .ReactModal__Overlay {
    background-color: ${transparentize(0.5, Palette.contrast)} !important;
  }

  .ReactModal__Content {
    transition: transform 250ms ease-in-out, opacity 250ms ease-in-out;
    opacity: 0;
    transform: translateY(100%);
    inset: 60vh 10vw 5vh 10vw !important;
    background-color: ${Palette.neutral} !important;
  }

  .ReactModal__Content--after-open {
    transform: translateY(0);
    opacity: 1;
    border-radius: 0;
  }

  .ReactModal__Content--before-close {
    opacity: 0;
    transform: translateY(100%);
  }
`;
