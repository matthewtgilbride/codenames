import type {} from 'styled-components/cssprop'; // eslint-disable-line
import { FC } from 'react';
import { GetServerSideProps } from 'next';
import Link from 'next/link';
import { NewGame } from '../components/NewGame';
import { ContentContainer } from '../components/ContentContainer';

interface HomeProps {
  game_name: string;
  API_URL: string;
}

const Home: FC<HomeProps> = ({ API_URL, game_name }) => (
  <ContentContainer>
    <div>
      <h2>create a new game</h2>
      <NewGame API_URL={API_URL} initialName={game_name} />
    </div>
    <h2>
      or <Link href="/game">join an existing one</Link>
    </h2>
  </ContentContainer>
);

export const getServerSideProps: GetServerSideProps<HomeProps> = async () => {
  const API_URL = process.env.API_URL as string;
  const result = await fetch(API_URL);
  const json = await result.json();

  return { props: { ...json, API_URL } as HomeProps };
};

export default Home;
