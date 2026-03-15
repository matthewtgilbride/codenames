import { css, Global } from '@emotion/react';
import React, { KeyboardEvent, MouseEvent, PropsWithChildren, useState } from 'react';
import ReactModal from 'react-modal';

// react-modal's class component types are incompatible with React 18's stricter JSX types
const StyledModal = ReactModal as unknown as React.FC<ReactModal.Props>;
import { lighten } from 'polished';
import { overlayColor, Palette } from './color';
import { Breakpoints } from './responsive';

if (typeof window !== 'undefined') {
  ReactModal.setAppElement('#app');
}

export interface ModalProps {
  isOpen: boolean;
  onRequestClose: (event: MouseEvent | KeyboardEvent) => void;
}

export const Modal = ({ isOpen, onRequestClose, children }: PropsWithChildren<ModalProps>) => (
  <>
    <Global styles={modalStyle} />
    <StyledModal
      isOpen={isOpen}
      closeTimeoutMS={500}
      onRequestClose={onRequestClose}
      shouldCloseOnOverlayClick
      shouldCloseOnEsc
    >
      {children}
    </StyledModal>
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
    background-color: ${overlayColor} !important;
  }

  .ReactModal__Content {
    transition: transform 250ms ease-in-out, opacity 250ms ease-in-out;
    opacity: 0;
    transform: translateY(100%);
    inset: 50vh 10vw 10vh 10vw !important;
    padding: 0.5rem !important;
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
