import { normalize } from 'styled-normalize';
import styled, { createGlobalStyle } from 'styled-components';
import { FC } from 'react';
import { Palette } from './color';

export const GlobalStyle = createGlobalStyle`
  ${normalize};
  body {
    font-family: Montserrat, 'Arial CE', Arial, sans-serif;
  }
  button {
    border-style: none;
  }
  input {
    border-width: 1px;
  }
`;

const Container = styled.div`
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  background-color: ${Palette.contrast};
  & h1 {
    text-align: center;
    color: ${Palette.blue};
  }
`;

export const Layout: FC = ({ children }) => (
  <Container>
    <h1>Codenames (⌐■_■)</h1>
    {children}
  </Container>
);
