import { FC } from 'react';
import { useParams } from 'react-router';
import { useSearchParams } from 'react-router-dom';
import { GameContainer, GameContainerProps } from './Game';
import { useApiContext } from '../ApiContext';
import { useFetchOnce } from '../../hooks/useFetch';
import { useGameContext } from './GameContext';

const GamePlayer: FC<GameContainerProps> = ({ currentPlayer }) => (
  <GameContainer currentPlayer={currentPlayer} />
);

export const GamePlayerContainer = () => {
  const apiContext = useApiContext();
  const { name, player } = useParams();
  const [query] = useSearchParams();
  const { setGame } = useGameContext();
  useFetchOnce(
    {
      apiContext,
      path: `/game/${name}/${player}?${query}`,
      onSuccess: (r) => r.json().then((json) => setGame(json)),
    },
    !!name,
  );

  const currentPlayer = {
    name: player as string,
    secret: query.get('secret') ?? undefined,
  };

  return <GamePlayer currentPlayer={currentPlayer} />;
};
