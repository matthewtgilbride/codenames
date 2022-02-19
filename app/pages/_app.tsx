import { AppProps } from 'next/app';
import { FC } from 'react';
import { Global } from '@emotion/react';
import { GlobalStyle } from '../design/GlobalStyle';
import { Layout } from '../design/layout';
import { ApiContextProvider } from '../components/ApiContext';
import { GameContextProvider } from '../components/game/GameContext';

const App: FC<AppProps> = ({ Component, pageProps }) => (
  <div suppressHydrationWarning id="app">
    {typeof window === 'undefined' ? null : (
      <>
        <Global styles={GlobalStyle} />
        <Layout>
          <ApiContextProvider>
            <GameContextProvider>
              <Component {...pageProps} />
            </GameContextProvider>
          </ApiContextProvider>
        </Layout>
      </>
    )}
  </div>
);

export default App;
