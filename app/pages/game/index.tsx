import { FC } from 'react';
import Link from 'next/link';
import { GetServerSideProps } from 'next';
import styled from 'styled-components';
import { Breakpoints } from '../../design/responsive';
import { getConstants } from '../../constants';
import { Palette } from '../../design/color';

const { tabletPortrait } = Breakpoints;
const { neutral, blue, contrast } = Palette;

const Container = styled.div`
  background-color: ${neutral};
  margin: auto;
  padding: 4rem;
  border-radius: 1rem;
  display: flex;
  flex-direction: column;
  max-width: ${tabletPortrait}px;
  box-shadow: 0 0 2px 1px ${blue};
  text-align: center;
  color: ${contrast};
  a {
    color: ${contrast};
  }
`;

interface GameListProps {
  games: string[];
}

const GameList: FC<GameListProps> = ({ games }) => (
  <Container>
    <h2>Join a game</h2>
    {games.map((g) => (
      <Link key={g} href={`/game/${g}`}>
        {g}
      </Link>
    ))}
  </Container>
);

export const getServerSideProps: GetServerSideProps<GameListProps> = async () => {
  const { API_BASE_URL } = getConstants();
  const result = await fetch(`${API_BASE_URL}/game`);
  const json = await result.json();

  return { props: json as GameListProps };
};

export default GameList;
