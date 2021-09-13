/* eslint-disable no-alert,no-restricted-globals */
import { FC } from 'react';
import { css } from '@emotion/css';
import { GameState, getFirstTeam, getGuesses, Player } from '../../../model';
import { PlayerList } from './PlayerList';
import { container } from './Info.styles';
import { Palette } from '../../../design/color';
import { Action } from './action/Action';

export interface InfoProps {
  word?: string;
  player?: Player;
  game: GameState;
}

export const Info: FC<InfoProps> = ({ player, game }) => {
  const firstTeam = getFirstTeam(game);

  const blueCount = getGuesses(game).filter(
    (i) => game.board[i].color === 'Blue',
  ).length;
  const redCount = getGuesses(game).filter((i) => game.board[i].color === 'Red')
    .length;

  return (
    <div className={container}>
      <div>
        <div
          className={css`
            color: ${Palette.blue};
            margin: 0 0.25rem;
          `}
        >
          {blueCount} / {firstTeam === 'Blue' ? 9 : 8}
        </div>
        <PlayerList spyMaster={false} team="Blue" game={game} player={player} />
        <PlayerList spyMaster team="Blue" game={game} player={player} />
      </div>
      <Action game={game} />
      <div>
        <div
          className={css`
            color: ${Palette.red};
          `}
        >
          {redCount} / {firstTeam === 'Red' ? 9 : 8}
        </div>
        <PlayerList spyMaster={false} team="Red" game={game} player={player} />
        <PlayerList spyMaster team="Red" game={game} player={player} />
      </div>
    </div>
  );
};
