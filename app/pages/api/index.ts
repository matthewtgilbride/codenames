import { NextApiHandler } from 'next';
import { getConstants } from '../../constants';

const handler: NextApiHandler = async (_, res) => {
  const { API_BASE_URL } = getConstants();
  const gameNameResult = await fetch(API_BASE_URL);
  const gameNameJson = await gameNameResult.json();
  const existingGamesResult = await fetch(`${API_BASE_URL}/game`);
  const existingGamesJson = await existingGamesResult.json();

  res.status(200).json({ ...gameNameJson, ...existingGamesJson });
};

export default handler;
