import { FC, useState } from 'react';
import { css } from '@emotion/css';
import { Link } from 'react-router-dom';
import { Palette } from '../../design/color';
import { Breakpoints } from '../../design/responsive';
import { useApiContext } from '../ApiContext';
import { useFetchOnce } from '../../hooks/useFetch';

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
          <Link to={`/game/${g}`}>{g}</Link>
        </li>
      ))}
    </ul>
  </div>
);

const { blue, light, contrast } = Palette;

const container = css`
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

const GameListContainer = () => {
  const apiContext = useApiContext();
  const [games, setGames] = useState<GameListProps['games'] | null>(null);
  useFetchOnce(
    {
      apiContext,
      path: '/game',
      onSuccess: (r) => r.json().then((p) => setGames(p.games)),
    },
    true,
  );
  return games ? <GameList games={games} /> : null;
};

export default GameListContainer;
