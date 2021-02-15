import { FC } from 'react';
import { GetServerSideProps } from 'next';
import styled from 'styled-components';
import { lighten } from 'polished';
import { Breakpoints } from '../../design/responsive';
import { getConstants } from '../../constants';
import { Palette } from '../../design/color';
import { Card, CardColor } from '../../components/game/Card';

const { tabletPortrait } = Breakpoints;

const Container = styled.div<{ turn: Team }>`
  text-align: center;
  & h2 {
    color: ${(props) => (props.turn === 'Blue' ? Palette.blue : Palette.red)};
  }
  & p {
    color: ${Palette.neutral};
  }
`;

const Grid = styled.div`
  display: grid;
  padding: 1rem;
  margin: auto;
  max-width: ${tabletPortrait}px;
  text-align: center;
  grid-template-columns: 1fr 1fr 1fr 1fr 1fr;
  grid-row-gap: 1rem;
  grid-column-gap: 1rem;
  max-width: ${tabletPortrait}px;
`;

const ThreeColumnGrid = styled.div`
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  align-items: center;
`;

const PlayerList = styled.div<{ color: Team }>`
  grid-column: ${(props) => (props.color === 'Blue' ? 1 : 3)};
  display: flex;
  flex-direction: column;
  color: ${(props) => (props.color === 'Blue' ? Palette.blue : Palette.red)};
  h2 {
    margin: 0;
  }
  li {
    margin: 1rem 0;
  }
`;

type Team = 'Blue' | 'Red';
interface Player {
  team: Team;
  is_spy_master: boolean;
  name: string;
}

interface GameProps {
  board: CardColor[];
  name: string;
  turn: Team;
  players: { [key: string]: Player };
  guesses: number[];
}

const GameLanding: FC<GameProps> = ({ board, name, turn, players }) => (
  <Container turn={turn}>
    <Grid>
      {board.map(({ color, word }) => (
        <Card key={word} color={color} word={word} />
      ))}
    </Grid>
    <ThreeColumnGrid>
      <p>{turn === 'Blue' && '(•_•)'}</p>
      <h2>{name}</h2>
      <p>{turn === 'Red' && '(•_•)'}</p>
    </ThreeColumnGrid>
    <ThreeColumnGrid>
      <button
        css={`
          grid-column: 2;
          background-color: ${Palette.neutral};
          padding: 0.5rem;
          border-radius: 0.25rem;
          :hover {
            background-color: ${lighten(0.1, Palette.neutral)};
          }
        `}
        type="button"
      >
        Join
      </button>
    </ThreeColumnGrid>
    <ThreeColumnGrid>
      <PlayerList color="Blue">
        <ul>
          {Object.values(players)
            .filter((p) => p.team === 'Blue')
            .map((p) => (
              <li key={p.name}>
                {p.is_spy_master && '⌐■-■  '}
                {p.name}
              </li>
            ))}
        </ul>
      </PlayerList>
      <PlayerList color="Red">
        <ul>
          {Object.values(players)
            .filter((p) => p.team === 'Red')
            .map((p) => (
              <li key={p.name}>
                {p.is_spy_master && '⌐■-■  '}
                {p.name}
              </li>
            ))}
        </ul>
      </PlayerList>
    </ThreeColumnGrid>
  </Container>
);

export const getServerSideProps: GetServerSideProps<GameProps> = async ({
  params,
}) => {
  const { API_BASE_URL } = getConstants();
  const game = params?.name;
  const result = await fetch(`${API_BASE_URL}/game/${game}`);
  const json = await result.json();

  return { props: json as GameProps };
};

export default GameLanding;
