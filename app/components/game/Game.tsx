import { FC, useState } from 'react';
import { css } from '@emotion/css';
import { Palette } from '../../design/color';
import { currentTeam, getFirstTeam, GameState, Team } from '../../model';
import { Info } from './info/Info';
import { usePoll } from '../../hooks/usePoll';
import { Board, BoardProps } from './Board';
import { useApiContext } from '../ApiContext';

export interface GameContainerProps {
  currentPlayer?: string;
  game: GameState;
}

export type GameProps = GameContainerProps & Pick<BoardProps, 'player'>;

export const Game: FC<GameProps> = ({ player, game, game: { name } }) => {
  const team = getFirstTeam(game);
  const turn = currentTeam(game);
  return (
    <div className={styleContainer(team, turn)}>
      <h2>{name}</h2>
      <Board game={game} player={player} />
      <Info game={game} player={player} />
    </div>
  );
};

export const GameContainer: FC<GameContainerProps> = ({
  currentPlayer,
  game,
}) => {
  const apiContext = useApiContext();
  const [gameState, setGameState] = useState(game);
  usePoll<GameState>({
    apiContext,
    path: `/game/${gameState.name}${currentPlayer ? `/${currentPlayer}` : ''}`,
    onSuccess: (newGame: GameState) => setGameState(newGame),
  });

  const { players } = gameState;
  const player = players[currentPlayer?.toLowerCase() ?? ''];

  return <Game game={gameState} player={player} />;
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
