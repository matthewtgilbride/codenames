import { FC } from 'react';
import { GetServerSideProps } from 'next';
import Link from 'next/link';
import { css } from '@emotion/css';
import { NewGame } from '../components/NewGame';
import { Palette } from '../design/color';
import { Breakpoints } from '../design/responsive';
import { ApiContextProvider } from '../components/ApiContext';

interface HomeProps {
  game_name: string;
  API_URL: string;
}

const Home: FC<HomeProps> = ({ API_URL, game_name }) => (
  <ApiContextProvider baseUrl={API_URL}>
    <div className={styleContent}>
      <div>
        <h2>create a new game</h2>
        <NewGame initialName={game_name} />
      </div>
      <h2>
        or <Link href="/game">join an existing one</Link>
      </h2>
    </div>
  </ApiContextProvider>
);

const { light, blue, contrast } = Palette;

export const styleContent = css`
  background-color: ${light};
  display: flex;
  flex-direction: column;
  margin: auto;
  padding: 1rem;
  border-radius: 1rem;
  box-shadow: 0 0 2px 1px ${blue};
  color: ${contrast};
  max-width: ${Breakpoints.tabletPortrait}px;
  text-align: center;
  * {
    color: ${contrast};
  }
`;

export const getServerSideProps: GetServerSideProps<HomeProps> = async () => {
  const API_URL = process.env.API_URL as string;
  const result = await fetch(API_URL);
  const json = await result.json();

  return { props: { ...json, API_URL } as HomeProps };
};

export default Home;
