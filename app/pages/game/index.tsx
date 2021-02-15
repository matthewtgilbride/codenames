import { FC } from 'react';
import Link from 'next/link';
import { GetServerSideProps } from 'next';
import { ContentContainer } from '../../components/ContentContainer';

interface GameListProps {
  games: string[];
}

const GameList: FC<GameListProps> = ({ games }) => (
  <ContentContainer>
    <h2>espionage in progress</h2>
    {games.map((g) => (
      <Link key={g} href={`/game/${g}`}>
        {g}
      </Link>
    ))}
  </ContentContainer>
);

export const getServerSideProps: GetServerSideProps<GameListProps> = async () => {
  const result = await fetch(`${process.env.API_URL}/game`);
  const json = await result.json();

  return { props: json as GameListProps };
};

export default GameList;
