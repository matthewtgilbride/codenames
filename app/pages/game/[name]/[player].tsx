import { FC } from 'react';
import { GetServerSideProps } from 'next';
import { encode } from 'querystring';
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
  query,
}) => {
  const game = params?.name as string;
  const player = params?.player as string;
  const API_URL = process.env.API_URL as string;
  const url = `${API_URL}/game/${game}/${player}?${encode(query)}`;
  const result = await fetch(url);
  const json = await result.json();

  return {
    props: {
      game: json,
      currentPlayer: {
        name: player,
        secret: query.secret === undefined ? null : query.secret,
      },
      API_URL,
    } as GameContainerProps,
  };
};

export default GamePlayer;
