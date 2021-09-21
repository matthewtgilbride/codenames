import { css } from '@emotion/react';
import reset from 'emotion-reset';

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
`;
