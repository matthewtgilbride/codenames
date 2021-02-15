import { FC } from 'react';
import { GetServerSideProps } from 'next';
import styled from 'styled-components';
import { Breakpoints } from '../../design/responsive';
import { Palette } from '../../design/color';
import { Card } from '../../components/game/Card';
import { Join } from '../../components/game/Join';
import { CardColor, Player, Team } from '../../model';

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
  padding: 1rem 0;
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
      <Join game={name} />
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
  const game = params?.name;
  const url = `${process.env.API_URL}/game/${game}`;
  const result = await fetch(url);
  const json = await result.json();

  return { props: json as GameProps };
};

export default GameLanding;
