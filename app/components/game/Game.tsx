/* eslint-disable no-alert,no-restricted-globals */
import { FC, useCallback, useState } from 'react';
import { useRouter } from 'next/router';
import { css } from '@emotion/css';
import { Palette } from '../../design/color';
import { currentTeam, firstTeam, GameState, Player, Team } from '../../model';
import { Info } from './info/Info';
import { usePoll } from '../../hooks/usePoll';
import { PlayerListProps } from './info/PlayerList';
import { jsonHeaders, voidFetch } from '../../utils/fetch';
import { Board, BoardProps } from './Board';
import { useApiContext } from '../ApiContext';

export interface GameContainerProps {
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
  game,
  game: { board, name, players },
}) => {
  const first_team = firstTeam(game);
  const turn = currentTeam(game);
  return (
    <div className={styleContainer(first_team, turn)}>
      <h2>{name}</h2>
      <Board board={board} onGuess={onGuess} turn={turn} player={player} />
      <Info player={player} game={game} onJoin={onJoin} players={players} />
    </div>
  );
};

export const GameContainer: FC<GameContainerProps> = ({
  currentPlayer,
  game,
}) => {
  const router = useRouter();
  const { baseUrl, setError } = useApiContext();

  const [gameState, setGameState] = useState(game);
  usePoll<GameState>({
    url: `${baseUrl}/game/${gameState.name}${
      currentPlayer ? `/${currentPlayer}` : ''
    }`,
    onError: () => setError(true),
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
        url: `${baseUrl}/game/${gameState.name}/join`,
        init: {
          method: 'PUT',
          body: JSON.stringify(newPlayer),
          headers: jsonHeaders,
        },
        onSuccess: () => router.push(`/game/${gameState.name}/${name}`),
        onError: () => setError(true),
      });
    },
    [baseUrl, setError, gameState, router],
  );

  const onGuess = (word: string) => () => {
    const confirmed = confirm(`Are you sure you want to guess ${word}?`);
    if (confirmed) {
      const index = gameState.board.map((c) => c.word).indexOf(word);
      voidFetch({
        url: `${baseUrl}/game/${gameState.name}/${player.name}/guess/${index}`,
        init: { method: 'PUT' },
        onSuccess: () => {},
        onError: () => setError(true),
      });
    }
  };

  return (
    <Game game={gameState} onGuess={onGuess} onJoin={onJoin} player={player} />
  );
};

const styleContainer = (first: Team, current: Team): string => css`
  text-align: center;

  & h2 {
    color: ${first === 'Blue' ? Palette.blue : Palette.red};
    margin: 0.5rem 0 0 0;
    font-size: 1rem;
  }

  & h3 {
    color: ${current === 'Blue' ? Palette.blue : Palette.red};
    font-weight: bold;
  }

  & p {
    color: ${Palette.neutral};
  }
`;
