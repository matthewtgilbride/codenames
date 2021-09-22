import { FC } from 'react';
import { GetServerSideProps } from 'next';
import { GameContainer, GameContainerProps } from '../../components/game/Game';
import { ApiContextProvider } from '../../components/ApiContext';
import { GameContextProvider } from '../../components/game/GameContext';
import { GameState } from '../../model';

const GameLanding: FC<
  GameContainerProps & { API_URL: string; game: GameState }
> = ({ API_URL, game, currentPlayer }) => (
  <ApiContextProvider baseUrl={API_URL}>
    <GameContextProvider game={game}>
      <GameContainer currentPlayer={currentPlayer} />
    </GameContextProvider>
  </ApiContextProvider>
);

export const getServerSideProps: GetServerSideProps<
  GameContainerProps & { game: GameState }
> = async ({ params }) => {
  const game = params?.name;
  const API_URL = process.env.API_URL as string;
  const url = `${API_URL}/game/${game}`;
  const result = await fetch(url);
  const json = await result.json();

  return { props: { game: json, API_URL } };
};

export default GameLanding;
