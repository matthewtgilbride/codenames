import type {} from 'styled-components/cssprop'; // eslint-disable-line
import { FC } from 'react';
import { GetServerSideProps } from 'next';
import Link from 'next/link';
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
  const result = await fetch(process.env.API_URL as string);
  const json = await result.json();

  return { props: json as HomeProps };
};

export default Home;
