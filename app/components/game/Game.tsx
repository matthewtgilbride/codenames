/* eslint-disable no-alert,no-restricted-globals */
import { FC, useCallback, useState } from 'react';
import styled from '@emotion/styled';
import { useRouter } from 'next/router';
import { Breakpoints } from '../../design/responsive';
import { Palette } from '../../design/color';
import { currentTeam, firstTeam, GameState, Player, Team } from '../../model';
import { GameInfo } from './GameInfo';
import { usePoll } from '../../hooks/usePoll';
import { PlayerList, PlayerListProps } from './PlayerList';
import { jsonHeaders, voidFetch } from '../../utils/fetch';
import { Board, BoardProps } from './Board';

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

export type GameProps = GameContainerProps &
  Pick<BoardProps, 'player' | 'onGuess'> &
  Pick<PlayerListProps, 'onJoin'>;

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
      <Board board={board} onGuess={onGuess} turn={turn} />
      <ThreeColumnGrid>
        <div>
          <PlayerList
            spyMaster={false}
            team="Blue"
            players={players}
            currentPlayer={player?.name}
            onJoin={onJoin}
          />
          <PlayerList
            spyMaster
            team="Blue"
            players={players}
            currentPlayer={player?.name}
            onJoin={onJoin}
          />
        </div>
        <GameInfo API_URL={API_URL} player={player} game={game} />
        <div>
          <PlayerList
            spyMaster={false}
            team="Red"
            players={players}
            currentPlayer={player?.name}
            onJoin={onJoin}
          />
          <PlayerList
            spyMaster
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
    (name: string, team: Team, spyMasterSecret: string | null) => {
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
