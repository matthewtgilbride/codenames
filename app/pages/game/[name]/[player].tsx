import { FC, useState } from 'react';
import { encode } from 'querystring';
import { useRouter } from 'next/router';
import {
  GameContainer,
  GameContainerProps,
} from '../../../components/game/Game';
import { GameState } from '../../../model';
import { GameContextProvider } from '../../../components/game/GameContext';
import { useApiContext } from '../../../components/ApiContext';
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
  const {
    query: { name, player, ...rest },
  } = useRouter();
  const [game, setGame] = useState<GameState | null>(null);
  useFetchOnce(
    {
      apiContext,
      path: `/game/${name}/${player}?${encode(rest)}`,
      onSuccess: (r) => r.json().then((json) => setGame(json)),
    },
    !!name,
  );

  const currentPlayer = {
    name: player as string,
    secret: rest.secret as string,
  };

  return game ? <GamePlayer game={game} currentPlayer={currentPlayer} /> : null;
};

export default GamePlayerContainer;
