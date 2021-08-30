/* eslint-disable no-alert,no-restricted-globals */
import { FC, useCallback, useState } from 'react';
import { useRouter } from 'next/router';
import { css } from '@emotion/css';
import { Palette } from '../../design/color';
import { currentTeam, firstTeam, GameState, Player, Team } from '../../model';
import { Info } from './Info';
import { usePoll } from '../../hooks/usePoll';
import { PlayerListProps } from './PlayerList';
import { jsonHeaders, voidFetch } from '../../utils/fetch';
import { Board, BoardProps } from './Board';

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
    <div className={styleContainer(first_team, turn)}>
      <h2>{name}</h2>
      <Board board={board} onGuess={onGuess} turn={turn} player={player} />
      <Info
        API_URL={API_URL}
        player={player}
        game={game}
        onJoin={onJoin}
        players={players}
      />
    </div>
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
        onSuccess: () => {},
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

const styleContainer = (first: Team, current: Team): string => css`
  text-align: center;

  & h2 {
    color: ${first === 'Blue' ? Palette.blue : Palette.red};
    margin-bottom: 0;
  }

  & h3 {
    color: ${current === 'Blue' ? Palette.blue : Palette.red};
    font-weight: bold;
  }

  & p {
    color: ${Palette.neutral};
  }
`;
