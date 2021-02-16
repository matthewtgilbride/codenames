import { FC } from 'react';
import { GetServerSideProps } from 'next';
import { Game, GameProps } from '../../components/game/Game';

const GameLanding: FC<GameProps> = (props) => <Game {...props} />;

export const getServerSideProps: GetServerSideProps<GameProps> = async ({
  params,
}) => {
  const game = params?.name;
  const API_URL = process.env.API_URL as string;
  const url = `${API_URL}/game/${game}`;
  const result = await fetch(url);
  const json = await result.json();

  return { props: { ...json, API_URL } as GameProps };
};

export default GameLanding;
