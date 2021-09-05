import { AppProps } from 'next/app';
import { FC } from 'react';
import { Global } from '@emotion/react';
import { Layout } from '../design/layout';
import { GlobalStyle } from '../design/GlobalStyle';

const App: FC<AppProps> = ({ Component, pageProps }) => (
  <>
    <Global styles={GlobalStyle} />
    <Layout>
      <Component {...pageProps} />
    </Layout>
  </>
);

export default App;
