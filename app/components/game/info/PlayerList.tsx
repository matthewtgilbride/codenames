import React, { FC, useCallback, useState } from 'react';
import { css } from '@emotion/css';
import { GameState, isSpyMaster, Team } from '../../../model';
import { Modal } from '../../../design/Modal';
import { styleButton, styleContainer, styleInput } from './PlayerList.styles';

export interface PlayerListProps {
  players: GameState['players'];
  currentPlayer?: string;
  team: Team;
  spyMaster: boolean;
  onJoin: (name: string, team: Team, spyMasterSecret: string | null) => void;
}

export const PlayerList: FC<PlayerListProps> = ({
  players,
  currentPlayer,
  team,
  spyMaster,
  onJoin,
}) => {
  const [open, setOpen] = useState(false);
  const onOpen = useCallback(() => {
    setOpen(true);
  }, []);
  const onRequestClose = () => setOpen(false);
  const [name, setName] = useState('');
  const [secret, setSecret] = useState('');
  const playerNames = getPlayerNames(players, team, spyMaster);
  return (
    <div className={styleContainer(team)}>
      <Modal isOpen={open} onRequestClose={onRequestClose}>
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
          onClick={() => onJoin(name, team, spyMaster ? secret : null)}
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
            <li
              key={p}
              style={p === currentPlayer ? { fontWeight: 'bold' } : {}}
            >
              {p}
            </li>
          ))
        ) : (
          <li>-</li>
        )}
      </ul>
      {!currentPlayer && (
        <button type="button" onClick={onOpen}>
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
