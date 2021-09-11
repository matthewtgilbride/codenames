/* eslint-disable no-alert,no-restricted-globals */
import { FC } from 'react';
import { css } from '@emotion/css';
import { currentTeam, GameState, Player } from '../../../model';
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
  const turn = currentTeam(game);

  return (
    <div className={container}>
      <div
        className={css`
          color: ${Palette.blue};
        `}
      >
        0 / 9
      </div>
      <div>
        <h3>{turn} Team&apos;s Turn</h3>
      </div>
      <div
        className={css`
          color: ${Palette.red};
        `}
      >
        0 / 8
      </div>
      <div>
        <PlayerList
          spyMaster={false}
          team="Blue"
          game={game}
          playerName={player?.name}
        />
        <PlayerList
          spyMaster
          team="Blue"
          game={game}
          playerName={player?.name}
        />
      </div>
      <Action game={game} player={player} />
      <div>
        <PlayerList
          spyMaster={false}
          team="Red"
          game={game}
          playerName={player?.name}
        />
        <PlayerList
          spyMaster
          team="Red"
          game={game}
          playerName={player?.name}
        />
      </div>
    </div>
  );
};
