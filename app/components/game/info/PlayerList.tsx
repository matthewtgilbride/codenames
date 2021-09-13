import React, { FC, useCallback } from 'react';
import { useRouter } from 'next/router';
import {
  currentTeam,
  currentTurn,
  GameState,
  isSpyMaster,
  Player,
  Team,
} from '../../../model';
import { Modal, useModalControls } from '../../../design/Modal';
import { styleButton, styleContainer, styleInput } from './PlayerList.styles';
import { voidFetch } from '../../../utils/fetch';
import { useApiContext } from '../../ApiContext';
import { useInputState } from '../../../hooks/useInputState';
import { StartTurn } from './action/StartTurn';
import { LeaveGame } from './action/LeaveGame';
import { EndTurn } from './action/EndTurn';

export interface PlayerListProps {
  game: GameState;
  player?: Player;
  team: Team;
  spyMaster: boolean;
}

export const PlayerList: FC<PlayerListProps> = ({
  game,
  player,
  team,
  spyMaster,
}) => {
  const { isOpen, open, close } = useModalControls();
  const [name, onNameChange] = useInputState();
  const [secret, onSecretChange] = useInputState();
  const playerNames = getPlayerNames(game.players, team, spyMaster);

  const apiContext = useApiContext();
  const router = useRouter();

  const onJoin = useCallback(() => {
    const newPlayer: Player = {
      name,
      team,
      spymaster_secret: spyMaster ? secret : null,
    };
    close();
    voidFetch({
      apiContext,
      path: `/game/${game.name}/join`,
      init: {
        method: 'PUT',
        body: JSON.stringify(newPlayer),
      },
      onSuccess: () => router.push(`/game/${game.name}/${name}`),
    });
  }, [spyMaster, team, game.name, router, apiContext, name, secret, close]);

  return (
    <div className={styleContainer(team)}>
      <Modal isOpen={isOpen} onRequestClose={close}>
        <label htmlFor="name" className={styleInput}>
          Name
          <input id="name" value={name} onChange={onNameChange} />
        </label>
        {spyMaster && (
          <label
            htmlFor="secret"
            className={styleInput}
            title="Enter something here to keep others from revealing the game key"
          >
            Personal Game Key Secret
            <input id="secret" value={secret} onChange={onSecretChange} />
          </label>
        )}
        <button
          type="submit"
          disabled={!name}
          onClick={onJoin}
          className={styleButton(team)}
        >
          Join as {team} Team {spyMaster ? 'Spy Master' : 'Operative'}
        </button>
      </Modal>
      <div>{spyMaster ? 'Spymaster' : 'Operative'}(s)</div>
      <ul>
        {playerNames.length > 0 ? (
          playerNames.map((p) => (
            <li
              key={p}
              style={p === player?.name ? { fontWeight: 'bold' } : {}}
            >
              {p}
            </li>
          ))
        ) : (
          <li>-</li>
        )}
      </ul>
      {!player && (
        <button type="button" onClick={open}>
          Join
        </button>
      )}
      {player &&
        player.team === team &&
        spyMaster &&
        isSpyMaster(player) &&
        currentTeam(game) === player.team &&
        currentTurn(game).type === 'Pending' && (
          <StartTurn game={game} spyMaster={player} />
        )}
      {player &&
        player.team === team &&
        spyMaster === isSpyMaster(player) &&
        ((spyMaster && isSpyMaster(player)) ||
          (!spyMaster && !isSpyMaster(player))) &&
        currentTurn(game).type === 'Started' && <EndTurn game={game} />}
      {player && player.team === team && spyMaster === isSpyMaster(player) && (
        <LeaveGame playerName={player.name} gameName={game.name} />
      )}
    </div>
  );
};

function getPlayerNames(
  players: GameState['players'],
  team: Team,
  spyMaster: boolean,
): string[] {
  return Object.values(players)
    .filter((p) => p.team === team && isSpyMaster(p) === spyMaster)
    .map((p) => p.name)
    .sort((a, b) => a.localeCompare(b));
}
