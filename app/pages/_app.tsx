import { useEffect, useState } from 'react';
import type { AppProps } from 'next/app';
import { Global } from '@emotion/react';
import { GlobalStyle } from '../design/GlobalStyle';
import { Layout } from '../design/layout';
import { ApiContextProvider } from '../components/ApiContext';
import { GameContextProvider } from '../components/game/GameContext';

const App = ({ Component, pageProps }: AppProps) => {
  const [mounted, setMounted] = useState(false);
  useEffect(() => {
    setMounted(true);
  }, []);

  if (!mounted) return <div id="app" />;

  return (
    <div id="app">
      <Global styles={GlobalStyle} />
      <Layout>
        <ApiContextProvider>
          <GameContextProvider>
            <Component {...pageProps} />
          </GameContextProvider>
        </ApiContextProvider>
      </Layout>
    </div>
  );
};

export default App;
