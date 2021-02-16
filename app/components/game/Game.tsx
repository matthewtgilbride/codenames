import { FC } from 'react';
import styled from 'styled-components';
import { Breakpoints } from '../../design/responsive';
import { Palette } from '../../design/color';
import { Card } from './Card';
import { Join } from './Join';
import { GameState, Team } from '../../model';
import { Play } from './Play';

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

export type GameProps = {
  API_URL: string;
  currentPlayer?: string;
} & GameState;

export const Game: FC<GameProps> = ({
  API_URL,
  currentPlayer,
  board,
  name,
  turn,
  players,
}) => {
  const player = players[currentPlayer?.toLowerCase() ?? ''];
  return (
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
        {player ? (
          <Play API_URL={API_URL} player={player} />
        ) : (
          <Join game={name} API_URL={API_URL} />
        )}
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
};
