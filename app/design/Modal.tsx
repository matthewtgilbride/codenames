import { css, Global } from '@emotion/react';
import React, { FC, KeyboardEvent, MouseEvent, useState } from 'react';
import ReactModal from 'react-modal';
import { lighten, transparentize } from 'polished';
import { Palette } from './color';
import { Breakpoints } from './responsive';

ReactModal.setAppElement('#app');

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

export const useModalControls = () => {
  const [isOpen, setIsOpen] = useState(false);
  const open = () => setIsOpen(true);
  const close = () => setIsOpen(false);
  return {
    isOpen,
    open,
    close,
  };
};

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
      circle at top right,
      ${Palette.neutral} 0%,
      ${lighten(0.2, Palette.neutral)} 15%,
      ${lighten(0.5, Palette.neutral)} 30%,
      ${Palette.light} 50%,
      ${lighten(0.5, Palette.neutral)} 65%,
      ${lighten(0.2, Palette.neutral)} 90%,
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
