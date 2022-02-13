import { FC, useState } from 'react';
import { useParams } from 'react-router';
import { GameContainer, GameContainerProps } from './Game';
import { useApiContext } from '../ApiContext';
import { GameContextProvider } from './GameContext';
import { GameState } from '../../model';
import { useFetchOnce } from '../../hooks/useFetch';

const GameLanding: FC<GameContainerProps & { game: GameState }> = ({
  game,
  currentPlayer,
}) => (
  <GameContextProvider game={game}>
    <GameContainer currentPlayer={currentPlayer} />
  </GameContextProvider>
);

const GameLandingContainer = () => {
  const apiContext = useApiContext();
  const { name } = useParams();
  const [game, setGame] = useState<GameState | null>(null);
  useFetchOnce(
    {
      apiContext,
      path: `/game/${name}`,
      onSuccess: (r) => r.json().then((json) => setGame(json)),
    },
    !!name,
  );

  return game ? <GameLanding game={game} /> : null;
};

export default GameLandingContainer;
