import { FC } from 'react';
import { css } from '@emotion/css';
import Link from 'next/link';
import { Palette } from '../../design/color';
import { currentTeam, GameState, getFirstTeam, Team } from '../../model';
import { Info } from './info/Info';
import { usePoll } from '../../hooks/usePoll';
import { Board, BoardProps } from './Board';
import { useApiContext } from '../ApiContext';
import { useGameContext } from './GameContext';

export interface GameContainerProps {
  currentPlayer?: {
    name: string;
    secret?: string;
  };
}

export type GameProps = GameContainerProps & Pick<BoardProps, 'player'>;

export const Game: FC<GameProps> = ({ player }) => {
  const {
    game,
    game: { name },
  } = useGameContext();
  const team = getFirstTeam(game);
  const turn = currentTeam(game);
  return (
    <div className={styleContainer(team, turn)}>
      <h2>
        <Link href={`/game/${name}`}>{name}</Link>
      </h2>
      <Board player={player} />
      <Info player={player} />
    </div>
  );
};

export const GameContainer: FC<GameContainerProps> = ({ currentPlayer }) => {
  const apiContext = useApiContext();
  const { game, setGame } = useGameContext();
  usePoll<GameState>({
    apiContext,
    path: `/game/${game.name}${playerSuffix(currentPlayer)}`,
    onSuccess: (newGame: GameState) => setGame(newGame),
  });

  const { players } = game;
  const player = players[currentPlayer?.name.toLowerCase() ?? ''];

  return <Game player={player} />;
};

const playerSuffix = (
  currentPlayer: GameContainerProps['currentPlayer'],
): string => {
  if (!currentPlayer) return '';
  if (currentPlayer.secret !== undefined) {
    return `/${currentPlayer.name}?secret=${currentPlayer.secret}`;
  }
  return `/${currentPlayer.name}`;
};

const styleContainer = (first: Team, current: Team): string => css`
  text-align: center;

  & h2 {
    margin: 0;
    font-size: 1rem;
    > a {
      text-decoration: underline;
      color: ${first === 'Blue' ? Palette.blue : Palette.red};
    }
  }

  & h3 {
    color: ${current === 'Blue' ? Palette.blue : Palette.red};
    font-weight: bold;
  }

  & p {
    color: ${Palette.neutral};
  }
`;
