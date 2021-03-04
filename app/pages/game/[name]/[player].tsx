import { FC } from 'react';
import { GetServerSideProps } from 'next';
import { Game, GameProps } from '../../../components/game/Game';

const GamePlayer: FC<GameProps> = (props) => <Game {...props} />;

export const getServerSideProps: GetServerSideProps<GameProps> = async ({
  params,
}) => {
  const game = params?.name as string;
  const player = params?.player as string;
  const API_URL = process.env.API_URL as string;
  const url = `${API_URL}/game/${game}/${player}`;
  const result = await fetch(url);
  const json = await result.json();

  return { props: { game: json, currentPlayer: player, API_URL } as GameProps };
};

export default GamePlayer;
