import { FC, useState } from 'react';
import { useRouter } from 'next/router';
import { GameContainer, GameContainerProps } from '../../components/game/Game';
import { useApiContext } from '../../components/ApiContext';
import { GameContextProvider } from '../../components/game/GameContext';
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
  const {
    query: { name },
  } = useRouter();
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
