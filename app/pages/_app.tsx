import { AppProps } from 'next/app';
import { FC } from 'react';
import { GlobalStyle, Layout } from '../design/layout';

const App: FC<AppProps> = ({ Component, pageProps }) => (
  <>
    <GlobalStyle />
    <Layout>
      <Component {...pageProps} />
    </Layout>
  </>
);

export default App;
