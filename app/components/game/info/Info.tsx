/* eslint-disable no-alert,no-restricted-globals */
import { FC } from 'react';
import { css } from '@emotion/css';
import {
  GameState,
  getFirstTeam,
  getGuesses,
  Player,
  Team,
} from '../../../model';
import { PlayerList } from './PlayerList';
import { container } from './Info.styles';
import { Palette } from '../../../design/color';
import { GameLog } from './GameLog';

export interface InfoProps {
  player?: Player;
  game: GameState;
}

export const Info: FC<InfoProps> = ({ player, game }) => (
  <div className={container}>
    <div>
      <PlayerListHeader game={game} team="Blue" />
      <PlayerList spyMaster={false} team="Blue" game={game} player={player} />
      <PlayerList spyMaster team="Blue" game={game} player={player} />
    </div>
    <GameLog board={game.board} turns={game.turns} />
    <div>
      <PlayerListHeader game={game} team="Red" />
      <PlayerList spyMaster={false} team="Red" game={game} player={player} />
      <PlayerList spyMaster team="Red" game={game} player={player} />
    </div>
  </div>
);

const PlayerListHeader: FC<{ game: GameState; team: Team }> = ({
  game,
  team,
}) => {
  const firstTeam = getFirstTeam(game);

  const count = getGuesses(game).filter((i) => game.board[i].color === 'Blue')
    .length;

  return (
    <div
      className={css`
        color: ${team === 'Blue' ? Palette.blue : Palette.red};
      `}
    >
      {count} / {firstTeam === team ? 9 : 8}
    </div>
  );
};
