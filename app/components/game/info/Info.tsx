/* eslint-disable no-alert,no-restricted-globals */
import { FC } from 'react';
import { useRouter } from 'next/router';
import { css } from '@emotion/css';
import { currentTeam, GameState, Player } from '../../../model';
import { voidFetch } from '../../../utils/fetch';
import { PlayerList, PlayerListProps } from './PlayerList';
import { container } from './Info.styles';
import { Palette } from '../../../design/color';
import { Action } from './Action';
import { useApiContext } from '../../ApiContext';

export interface InfoProps {
  word?: string;
  player?: Player;
  game: GameState;
  players: PlayerListProps['players'];
  onJoin: PlayerListProps['onJoin'];
}

export const Info: FC<InfoProps> = ({ player, game, players, onJoin }) => {
  const router = useRouter();
  const apiContext = useApiContext();
  const turn = currentTeam(game);
  const onEndTurn = () => {
    const confirmed = confirm(
      `Are you sure you want to end ${turn} team's turn?`,
    );
    if (confirmed) {
      voidFetch({
        apiContext,
        path: `/game/${game.name}/end-turn`,
        init: { method: 'PUT' },
        onSuccess: () => router.reload(),
      });
    }
  };

  const onLeave = () => {
    const confirmed = confirm(`Are you sure you want to leave the game?`);
    if (confirmed) {
      voidFetch({
        apiContext,
        path: `/game/${game.name}/${player?.name}/leave`,
        init: { method: 'PUT' },
        onSuccess: () => router.push(`/game/${game.name}`),
      });
    }
  };

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
      <Action
        player={player}
        game={game}
        onEndTurn={onEndTurn}
        onLeave={onLeave}
      />
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
    </div>
  );
};
