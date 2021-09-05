import { css, Global } from '@emotion/react';
import React, { FC, MouseEvent, KeyboardEvent } from 'react';
import ReactModal from 'react-modal';
import { transparentize } from 'polished';
import { Palette } from './color';
import { Breakpoints } from './responsive';

export interface ModalProps {
  isOpen: boolean;
  onRequestClose: (event: MouseEvent | KeyboardEvent) => void;
}

export const Modal: FC<ModalProps> = ({ isOpen, onRequestClose, children }) => (
  <>
    <Global styles={modalStyle} />
    <ReactModal
      isOpen={isOpen}
      closeTimeoutMS={500}
      onRequestClose={onRequestClose}
      shouldCloseOnOverlayClick
      shouldCloseOnEsc
    >
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
    inset: 75vh 10vw 1rem 10vw !important;
    background: radial-gradient(
      circle at bottom left,
      ${Palette.neutral} 0%,
      ${Palette.light} 30%,
      ${Palette.light} 70%,
      ${Palette.neutral} 100%
    ) !important;
    max-width: ${Breakpoints.phone}px !important;
    margin: auto;
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
