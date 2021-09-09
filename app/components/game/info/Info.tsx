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

export interface InfoProps {
  API_URL: string;
  word?: string;
  player?: Player;
  game: GameState;
  players: PlayerListProps['players'];
  onJoin: PlayerListProps['onJoin'];
}

export const Info: FC<InfoProps> = ({
  player,
  game,
  API_URL,
  players,
  onJoin,
}) => {
  const router = useRouter();
  const turn = currentTeam(game);
  const onEndTurn = () => {
    const confirmed = confirm(
      `Are you sure you want to end ${turn} team's turn?`,
    );
    if (confirmed) {
      voidFetch({
        url: `${API_URL}/game/${name}/end-turn`,
        init: { method: 'PUT' },
        onSuccess: () => router.reload(),
        onError: () => alert('failed to end turn'),
      });
    }
  };

  const onLeave = () => {
    const confirmed = confirm(`Are you sure you want to leave the game?`);
    if (confirmed) {
      voidFetch({
        url: `${API_URL}/game/${name}/${player?.name}/leave`,
        init: { method: 'PUT' },
        onSuccess: () => router.push(`/game/${name}`),
        onError: () => alert('failed to leave game'),
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
        API_URL={API_URL}
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
