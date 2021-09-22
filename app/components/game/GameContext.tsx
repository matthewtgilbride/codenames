import { createContext, FC, useContext, useState } from 'react';
import { GameState } from '../../model';

export interface GameContextType {
  game: GameState;
  setGame: (game: GameState) => void;
}

export const GameContext = createContext<GameContextType | undefined>(
  undefined,
);

export const GameContextProvider: FC<{
  game: GameState;
}> = ({ game, children }) => {
  const [gameState, setGameState] = useState(game);
  return (
    <GameContext.Provider value={{ game: gameState, setGame: setGameState }}>
      {children}
    </GameContext.Provider>
  );
};

export const useGameContext = () => {
  const gameContext = useContext(GameContext);
  if (!gameContext)
    throw new Error('useApiContext must be used within a GameContextProvider');
  return gameContext;
};
