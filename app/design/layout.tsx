import { normalize } from 'styled-normalize';
import styled, { createGlobalStyle } from 'styled-components';
import { FC } from 'react';
import Link from 'next/link';
import { Palette } from './color';
import { beginAt, Breakpoints } from './responsive';

const { phoneMd, phoneLg, tabletPortrait } = Breakpoints;

export const GlobalStyle = createGlobalStyle`
  ${normalize};
  * {
    font-family: Montserrat, 'Arial CE', Arial, sans-serif;
    box-sizing: border-box;
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
`;

const Title = styled.h1`
  text-align: center;
  color: ${Palette.blue};
  font-size: 1rem;
  a {
    color: ${Palette.blue};
  }
  ${beginAt(phoneMd)} {
    font-size: 1.25rem;
  }
  ${beginAt(phoneLg)} {
    font-size: 1.5rem;
  }
  ${beginAt(tabletPortrait)} {
    font-size: 2rem;
  }
`;

export const Layout: FC = ({ children }) => (
  <Container>
    <Title>
      (•_•) ( •_•) <Link href="/">Codenames</Link> ⌐■-■ (⌐■_■)
    </Title>
    {children}
  </Container>
);
