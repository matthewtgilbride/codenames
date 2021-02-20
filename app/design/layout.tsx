import { reset } from 'styled-reset';
import styled, { createGlobalStyle } from 'styled-components';
import { FC } from 'react';
import Link from 'next/link';
import { Palette } from './color';
import { beginAt, Breakpoints } from './responsive';

const { phoneMd, phoneLg, tabletPortrait } = Breakpoints;

export const GlobalStyle = createGlobalStyle`
  ${reset};
  body {
    font-family: Montserrat, 'Arial CE', Arial, sans-serif;
    & * { box-sizing: border-box; }
  }
  button, input {
    border-style: solid;
  }
  h1 {
    font-size: 2rem;
    margin: 2rem 0;
  }
  h2 {
    font-size: 1.5rem;
    margin: 1.5rem;
  }
`;

const Container = styled.div`
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  overflow-y: auto;
  padding: 0 1rem;
  background-color: ${Palette.contrast};
`;

const Title = styled.h1`
  text-align: center;
  color: ${Palette.neutral};
  font-size: 1rem;
  a {
    color: ${Palette.neutral};
    margin: 0 1rem;
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
      (•_•) ( •_•)<Link href="/">Codenames</Link>⌐■-■ (⌐■_■)
    </Title>
    {children}
  </Container>
);
