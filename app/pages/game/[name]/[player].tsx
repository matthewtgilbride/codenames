import { FC } from 'react';
import { GetServerSideProps } from 'next';
import {
  GameContainer,
  GameContainerProps,
} from '../../../components/game/Game';
import { ApiContextProvider } from '../../../components/ApiContext';

const GamePlayer: FC<GameContainerProps & { API_URL: string }> = ({
  API_URL,
  ...rest
}) => (
  <ApiContextProvider baseUrl={API_URL}>
    <GameContainer {...rest} />
  </ApiContextProvider>
);

export const getServerSideProps: GetServerSideProps<GameContainerProps> = async ({
  params,
}) => {
  const game = params?.name as string;
  const player = params?.player as string;
  const API_URL = process.env.API_URL as string;
  const url = `${API_URL}/game/${game}/${player}`;
  const result = await fetch(url);
  const json = await result.json();

  return {
    props: { game: json, currentPlayer: player, API_URL } as GameContainerProps,
  };
};

export default GamePlayer;
