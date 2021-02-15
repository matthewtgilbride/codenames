import { NextApiHandler } from 'next';

const handler: NextApiHandler = async (req, res) => {
  const response = await fetch(`${process.env.API_URL}/game`, {
    method: 'POST',
    body: req.body,
    headers: {
      'content-type': 'application/json',
    },
  });
  if (response.ok) {
    res.status(200).end();
  } else {
    res.status(response.status).send(response.body);
  }
};

export default handler;
