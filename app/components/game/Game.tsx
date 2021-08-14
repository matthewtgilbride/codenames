/* eslint-disable no-alert,no-restricted-globals */
import { FC, MouseEventHandler, useCallback, useState } from 'react';
import styled from '@emotion/styled';
import { useRouter } from 'next/router';
import { Breakpoints } from '../../design/responsive';
import { Palette } from '../../design/color';
import { Card } from './Card';
import { currentTeam, firstTeam, GameState, Player, Team } from '../../model';
import { GameInfo } from './GameInfo';
import { usePoll } from '../../hooks/usePoll';
import { PlayerList } from './PlayerList';
import { jsonHeaders, voidFetch } from '../../utils/fetch';

const { tabletPortrait } = Breakpoints;

const Container = styled.div<{ first_team: Team; turn: Team }>`
  text-align: center;

  & h2 {
    color: ${(props) =>
      props.first_team === 'Blue' ? Palette.blue : Palette.red};
    margin-bottom: 0;
  }

  & h3 {
    color: ${(props) => (props.turn === 'Blue' ? Palette.blue : Palette.red)};
    font-weight: bold;
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

  margin: 1rem auto;
  max-width: ${tabletPortrait}px;
`;

export interface GameContainerProps {
  API_URL: string;
  currentPlayer?: string;
  game: GameState;
}

export type GameProps = GameContainerProps & {
  player?: Player;
  onGuess: (word: string) => MouseEventHandler;
  onJoin: (name: string, team: Team, spyMasterSecret: string) => void;
};

export const Game: FC<GameProps> = ({
  player,
  onGuess,
  onJoin,
  API_URL,
  game,
  game: { board, name, players },
}) => {
  const first_team = firstTeam(game);
  const turn = currentTeam(game);
  return (
    <Container first_team={first_team} turn={turn}>
      <h2>{name}</h2>
      <Grid>
        {board.map((card) => (
          <Card
            key={card.word}
            card={card}
            player={player}
            turn={turn}
            onClick={onGuess(card.word)}
          />
        ))}
      </Grid>
      <ThreeColumnGrid>
        <div>
          <PlayerList
            isSpyMaster={false}
            team="Blue"
            players={players}
            currentPlayer={player?.name}
            onJoin={onJoin}
          />
          <PlayerList
            isSpyMaster
            team="Blue"
            players={players}
            currentPlayer={player?.name}
            onJoin={onJoin}
          />
        </div>
        <GameInfo API_URL={API_URL} player={player} game={game} />
        <div>
          <PlayerList
            isSpyMaster={false}
            team="Red"
            players={players}
            currentPlayer={player?.name}
            onJoin={onJoin}
          />
          <PlayerList
            isSpyMaster
            team="Red"
            players={players}
            currentPlayer={player?.name}
            onJoin={onJoin}
          />
        </div>
      </ThreeColumnGrid>
    </Container>
  );
};

export const GameContainer: FC<GameContainerProps> = ({
  API_URL,
  currentPlayer,
  game,
}) => {
  const router = useRouter();

  const [gameState, setGameState] = useState(game);
  usePoll<GameState>({
    url: `${API_URL}/game/${gameState.name}${
      currentPlayer ? `/${currentPlayer}` : ''
    }`,
    // eslint-disable-next-line no-alert
    onError: () => alert('error fetching game data'),
    onSuccess: (newGame: GameState) => setGameState(newGame),
  });

  const { players } = gameState;
  const player = players[currentPlayer?.toLowerCase() ?? ''];

  const onJoin = useCallback(
    (name: string, team: Team, spyMasterSecret: string) => {
      const newPlayer: Player = {
        name,
        team,
        spymaster_secret: spyMasterSecret,
      };
      voidFetch({
        url: `${API_URL}/game/${gameState.name}/join`,
        init: {
          method: 'PUT',
          body: JSON.stringify(newPlayer),
          headers: jsonHeaders,
        },
        onSuccess: () => router.push(`/game/${gameState.name}/${name}`),
        onError: () => alert('error joining game'),
      });
    },
    [API_URL, gameState, router],
  );

  const onGuess = (word: string) => () => {
    const confirmed = confirm(`Are you sure you want to guess ${word}?`);
    if (confirmed) {
      const index = gameState.board.map((c) => c.word).indexOf(word);
      voidFetch({
        url: `${API_URL}/game/${gameState.name}/${player.name}/guess/${index}`,
        init: { method: 'PUT' },
        onSuccess: () => router.reload(),
        onError: () => alert('error making guess'),
      });
    }
  };

  return (
    <Game
      API_URL={API_URL}
      game={gameState}
      onGuess={onGuess}
      onJoin={onJoin}
      player={player}
    />
  );
};
