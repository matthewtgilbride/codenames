import { FC, useState } from 'react';
import { useParams } from 'react-router';
import { useSearchParams } from 'react-router-dom';
import { GameContainer, GameContainerProps } from '../Game';
import { GameState } from '../../../model';
import { GameContextProvider } from '../GameContext';
import { useApiContext } from '../../ApiContext';
import { useFetchOnce } from '../../../hooks/useFetch';

const GamePlayer: FC<GameContainerProps & { game: GameState }> = ({
  game,
  currentPlayer,
}) => (
  <GameContextProvider game={game}>
    <GameContainer currentPlayer={currentPlayer} />
  </GameContextProvider>
);

export const GamePlayerContainer = () => {
  const apiContext = useApiContext();
  const { name, player } = useParams();
  const [query] = useSearchParams();
  const [game, setGame] = useState<GameState | null>(null);
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

  return game ? <GamePlayer game={game} currentPlayer={currentPlayer} /> : null;
};

export default GamePlayerContainer;
