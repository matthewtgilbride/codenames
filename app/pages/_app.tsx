import { AppProps } from 'next/app';
import { FC } from 'react';
import { css, Global } from '@emotion/react';
import reset from 'emotion-reset';
import { Layout } from '../design/layout';

const App: FC<AppProps> = ({ Component, pageProps }) => (
  <>
    <Global styles={GlobalStyle} />
    <Layout>
      <Component {...pageProps} />
    </Layout>
  </>
);

export const GlobalStyle = css`
  ${reset};
  body {
    font-family: Montserrat, 'Arial CE', Arial, sans-serif;
    & * {
      box-sizing: border-box;
    }
  }
  button,
  input {
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
  .ReactModal__Overlay {
    transition: transform 500ms ease-in-out, opacity 250ms ease-in-out;
    opacity: 0;
    transform: scale(0) rotateZ(-180deg);
  }

  .ReactModal__Overlay--after-open {
    transform: scale(1) rotateZ(0deg);
    opacity: 1;
    border-radius: 0;
  }

  .ReactModal__Overlay--before-close {
    opacity: 0;
    transform: scale(0) rotateZ(-180deg);
  }
`;

export default App;
