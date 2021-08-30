import { FC } from 'react';
import Link from 'next/link';
import { GetServerSideProps } from 'next';
import { css } from '@emotion/css';
import { Palette } from '../../design/color';
import { Breakpoints } from '../../design/responsive';

interface GameListProps {
  games: string[];
}

const GameList: FC<GameListProps> = ({ games }) => (
  <div className={container}>
    <h2>espionage in progress</h2>
    <ul>
      {games.map((g) => (
        <li
          key={g}
          className={css`
            margin: 1rem;
          `}
        >
          <Link href={`/game/${g}`}>{g}</Link>
        </li>
      ))}
    </ul>
  </div>
);

const { blue, neutral, contrast } = Palette;

const container = css`
  background-color: ${neutral};
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

export const getServerSideProps: GetServerSideProps<GameListProps> = async () => {
  const result = await fetch(`${process.env.API_URL}/game`);
  const json = await result.json();

  return { props: json as GameListProps };
};

export default GameList;
