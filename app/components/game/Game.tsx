import { FC, useState } from 'react';
import styled from 'styled-components';
import { Breakpoints } from '../../design/responsive';
import { Palette } from '../../design/color';
import { Card } from './Card';
import { Join } from './Join';
import { GameState, Team } from '../../model';
import { Play } from './Play';

const { tabletPortrait } = Breakpoints;

const Container = styled.div<{ first_team: Team }>`
  text-align: center;
  & h2 {
    color: ${(props) =>
      props.first_team === 'Blue' ? Palette.blue : Palette.red};
    margin: 0;
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
  margin: 1rem auto;
  max-width: ${tabletPortrait}px;
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
  first_team,
  name,
  turn,
  players,
}) => {
  const player = players[currentPlayer?.toLowerCase() ?? ''];
  const [selectedWord, setSelectedWord] = useState<string | undefined>();
  const onClick = (word: string) => () => setSelectedWord(word);
  return (
    <Container first_team={first_team}>
      <Grid>
        {board.map((card) => (
          <Card
            key={card.word}
            card={card}
            player={player}
            turn={turn}
            onClick={onClick(card.word)}
          />
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
          <Play
            API_URL={API_URL}
            game={name}
            board={board}
            player={player}
            turn={turn}
            word={selectedWord}
          />
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
