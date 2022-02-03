import { AppProps } from 'next/app';
import { FC } from 'react';
import { Global } from '@emotion/react';
import { Layout } from '../design/layout';
import { GlobalStyle } from '../design/GlobalStyle';
import { ApiContextProvider } from '../components/ApiContext';

const App: FC<AppProps> = ({ Component, pageProps }) => (
  <>
    <Global styles={GlobalStyle} />
    <Layout>
      <ApiContextProvider>
        <Component {...pageProps} />
      </ApiContextProvider>
    </Layout>
  </>
);

export default App;
