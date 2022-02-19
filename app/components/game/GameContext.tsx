import { createContext, FC, useContext, useState } from 'react';
import { GameState } from '../../model';

export interface GameContextType {
  game: GameState;
  setGame: (game: GameState) => void;
}

export const GameContext = createContext<GameContextType | undefined>(
  undefined,
);

export const GameContextProvider: FC = ({ children }) => {
  const [gameState, setGameState] = useState(placeHolderGame);
  return (
    <GameContext.Provider value={{ game: gameState, setGame: setGameState }}>
      {children}
    </GameContext.Provider>
  );
};

export const useGameContext = () => {
  const gameContext = useContext(GameContext);
  if (!gameContext)
    throw new Error('gameContext must be used within a GameContextProvider');
  return gameContext;
};

const placeHolderGame: GameState = {
  name: '',
  players: {},
  turns: [
    {
      type: 'Pending',
      data: 'Blue',
    },
  ],
  board: new Array(25).map((_) => ({
    color: null,
    word: '-',
  })),
};
