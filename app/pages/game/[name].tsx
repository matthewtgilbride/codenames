import { FC } from 'react';
import { GetServerSideProps } from 'next';
import styled from 'styled-components';
import { Breakpoints } from '../../design/responsive';
import { getConstants } from '../../constants';
import { Palette } from '../../design/color';

const { tabletPortrait } = Breakpoints;
const { neutral, blue } = Palette;

const Container = styled.div`
  display: grid;
  // background-color: ${neutral};
  flex-direction: column;
  padding: 0 0.75rem;
  margin: auto;
  border-radius: 1rem;
  max-width: ${tabletPortrait}px;
  text-align: center;
  // box-shadow: 0 0 2px 1px ${blue};
  grid-template-columns: 1fr 1fr 1fr 1fr 1fr;
  grid-row-gap: 1rem;
  grid-column-gap: 0.75rem;
  max-width: ${tabletPortrait}px;
`;

const Card = styled.div`
  background-color: ${neutral};
  box-shadow: 0 0 2px 1px ${blue};
  border-radius: 0.25rem;
  padding: 1rem 0.1rem;
  font-size: 0.5rem;
`;

interface GameLandingProps {
  board: {
    color: string;
    word: string;
  }[];
}

const GameLanding: FC<GameLandingProps> = ({ board }) => (
  <Container>
    {board.map(({ word }) => (
      <Card key={word}>{word}</Card>
    ))}
  </Container>
);

export const getServerSideProps: GetServerSideProps<GameLandingProps> = async ({
  params,
}) => {
  const { API_BASE_URL } = getConstants();
  const game = params?.name;
  const result = await fetch(`${API_BASE_URL}/game/${game}`);
  const json = await result.json();

  return { props: json as GameLandingProps };
};

export default GameLanding;
