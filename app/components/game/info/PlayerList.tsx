import React, { FC, useCallback, useState } from 'react';
import { css } from '@emotion/css';
import { useRouter } from 'next/router';
import { GameState, isSpyMaster, Player, Team } from '../../../model';
import { Modal } from '../../../design/Modal';
import { styleButton, styleContainer, styleInput } from './PlayerList.styles';
import { voidFetch } from '../../../utils/fetch';
import { useApiContext } from '../../ApiContext';

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
  const [open, setOpen] = useState(false);
  const [name, setName] = useState('');
  const [secret, setSecret] = useState('');
  const playerNames = getPlayerNames(game.players, team, spyMaster);

  const apiContext = useApiContext();
  const router = useRouter();

  const onJoin = useCallback(() => {
    const newPlayer: Player = {
      name,
      team,
      spymaster_secret: spyMaster ? secret : null,
    };
    voidFetch({
      apiContext,
      path: `/game/${game.name}/join`,
      init: {
        method: 'PUT',
        body: JSON.stringify(newPlayer),
      },
      onSuccess: () => router.push(`/game/${game.name}/${name}`),
      onError: () => setOpen(false),
    });
  }, [spyMaster, team, game.name, router, apiContext, name, secret]);

  return (
    <div className={styleContainer(team)}>
      <Modal isOpen={open} onRequestClose={() => setOpen(false)}>
        <label htmlFor="name" className={styleInput}>
          Name
          <input
            id="name"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </label>
        {spyMaster && (
          <label htmlFor="secret" className={styleInput}>
            Secret
            <input
              id="secret"
              value={secret}
              onChange={(e) => setSecret(e.target.value)}
            />
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
          align-self: flex-start;
          font-weight: bold;
        `}
      >
        {spyMaster ? 'Spymaster' : 'Operative'}(s)
      </div>
      <ul>
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
        <button type="button" onClick={() => setOpen(true)}>
          {spyMaster ? 'Join as Spymaster' : 'Join as Operative'}
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
