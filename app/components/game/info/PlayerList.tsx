import React, { FC, useCallback } from 'react';
import { css } from '@emotion/css';
import { useRouter } from 'next/router';
import { GameState, isSpyMaster, Player, Team } from '../../../model';
import { Modal, useModalControls } from '../../../design/Modal';
import { styleButton, styleContainer, styleInput } from './PlayerList.styles';
import { voidFetch } from '../../../utils/fetch';
import { useApiContext } from '../../ApiContext';
import { useInputState } from '../../../hooks/useInputState';

export interface PlayerListProps {
  game: GameState;
  playerName?: string;
  team: Team;
  spyMaster: boolean;
}

export const PlayerList: FC<PlayerListProps> = ({
  game,
  playerName,
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
      <div
        className={css`
          font-weight: bold;
        `}
      >
        {spyMaster ? 'Spymaster' : 'Operative'}(s)
      </div>
      <ul
        className={css`
          align-self: center;
          justify-content: center;
          margin: 0.25rem 0;
        `}
      >
        {playerNames.length > 0 ? (
          playerNames.map((p) => (
            <li key={p} style={p === playerName ? { fontWeight: 'bold' } : {}}>
              {p}
            </li>
          ))
        ) : (
          <li>-</li>
        )}
      </ul>
      {!playerName && (
        <button type="button" onClick={open}>
          Join
        </button>
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
