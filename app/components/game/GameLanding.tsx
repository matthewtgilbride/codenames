import { FC } from 'react';
import { useParams } from 'react-router';
import { GameContainer, GameContainerProps } from './Game';
import { useApiContext } from '../ApiContext';
import { useGameContext } from './GameContext';
import { useFetchOnce } from '../../hooks/useFetch';

const GameLanding: FC<GameContainerProps> = ({ currentPlayer }) => (
  <GameContainer currentPlayer={currentPlayer} />
);

export const GameLandingContainer = () => {
  const apiContext = useApiContext();
  const { setGame } = useGameContext();
  const { name } = useParams();
  useFetchOnce(
    {
      apiContext,
      path: `/game/${name}`,
      onSuccess: (r) => r.json().then((json) => setGame(json)),
    },
    !!name,
  );

  return <GameLanding />;
};
