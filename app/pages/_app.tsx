import { AppProps } from 'next/app';
import { FC } from 'react';
import { Global } from '@emotion/react';
import { GlobalStyle } from '../design/GlobalStyle';
import { Layout } from '../design/layout';
import { ApiContextProvider } from '../components/ApiContext';

const App: FC<AppProps> = ({ Component, pageProps }) => (
  <div suppressHydrationWarning id="app">
    {typeof window === 'undefined' ? null : (
      <>
        <Global styles={GlobalStyle} />
        <Layout>
          <ApiContextProvider>
            <Component {...pageProps} />
          </ApiContextProvider>
        </Layout>
      </>
    )}
  </div>
);

export default App;
