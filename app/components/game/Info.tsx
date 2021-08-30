/* eslint-disable no-alert,no-restricted-globals */
import { FC } from 'react';
import { useRouter } from 'next/router';
import { currentTeam, GameState, getGuesses, Player } from '../../model';
import { voidFetch } from '../../utils/fetch';
import { GuessLog } from './GuessLog';
import { PlayerList, PlayerListProps } from './PlayerList';
import { action, actionButton, container } from './Info.styles';

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
  game: { name, board },
  API_URL,
  players,
  onJoin,
}) => {
  const router = useRouter();
  const turn = currentTeam(game);
  const guesses = getGuesses(game);
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
      <div className={action}>
        <h3>{turn} Team&apos;s Turn</h3>
        {player && (
          <button type="button" onClick={onEndTurn} className={actionButton}>
            End Turn
          </button>
        )}
        <GuessLog board={board} guesses={guesses} />
        {player && (
          <button type="button" onClick={onLeave} className={actionButton}>
            Leave Game
          </button>
        )}
      </div>
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
