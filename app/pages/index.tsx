import type {} from 'styled-components/cssprop'; // eslint-disable-line
import { FC } from 'react';
import { GetServerSideProps } from 'next';
import Link from 'next/link';
import { getConstants } from '../constants';
import { NewGame } from '../components/NewGame';
import { ContentContainer } from '../components/ContentContainer';

interface HomeProps {
  game_name: string;
}

const Home: FC<HomeProps> = ({ game_name }) => (
  <ContentContainer>
    <div>
      <h2>create a new game</h2>
      <NewGame initialName={game_name} />
    </div>
    <h2>
      or <Link href="/game">join an existing one</Link>
    </h2>
  </ContentContainer>
);

export const getServerSideProps: GetServerSideProps<HomeProps> = async () => {
  const { API_BASE_URL } = getConstants();
  const result = await fetch(API_BASE_URL);
  const json = await result.json();

  return { props: json as HomeProps };
};

export default Home;
