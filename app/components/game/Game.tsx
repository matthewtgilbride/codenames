import { FC, useState } from 'react';
import styled from 'styled-components';
import { Breakpoints } from '../../design/responsive';
import { Palette } from '../../design/color';
import { Card } from './Card';
import { Join } from './Join';
import { GameState, Team } from '../../model';
import { Play } from './Play';
import { usePoll } from '../../hooks/usePoll';

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
  game: GameState;
};

export const Game: FC<GameProps> = ({ API_URL, currentPlayer, game }) => {
  const [selectedWord, setSelectedWord] = useState<string | undefined>();
  const onClick = (word: string) => () => setSelectedWord(word);

  const [gameState, setGameState] = useState(game);
  usePoll<GameState>({
    url: `${API_URL}/game/${gameState.name}${
      currentPlayer ? `/${currentPlayer}` : ''
    }`,
    // eslint-disable-next-line no-alert
    onError: () => alert('error fetching game data'),
    onSuccess: (newGame: GameState) => setGameState(newGame),
  });

  const { players, first_team, board, turn, name } = gameState;
  const player = players[currentPlayer?.toLowerCase() ?? ''];

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
        <p>Blue Team {turn === 'Blue' && '(•_•)'}</p>
        <h2>{name}</h2>
        <p>Red Team {turn === 'Red' && '(•_•)'}</p>
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
            player={player}
            word={selectedWord}
            game={gameState}
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
