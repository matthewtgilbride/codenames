import React, { FC, useCallback } from 'react';
import { lighten } from 'polished';
import { css } from '@emotion/css';
import { Palette } from '../../../design/color';
import { GameState, isSpyMaster, Team } from '../../../model';
import { beginAt } from '../../../design/responsive';
import { buttonStyle } from '../../../design/button';

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
  const playerNames = getPlayerNames(players, team, spyMaster);
  const onClick = useCallback(() => {
    // eslint-disable-next-line no-alert
    const name = window.prompt('What is your name?');
    if (name === null) return;
    onJoin(name, team, spyMaster ? '' : null); // TODO: figure out how to pass secret
  }, [team, spyMaster, onJoin]);
  return (
    <div className={styleContainer(team)}>
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
        <button type="button" onClick={onClick}>
          {spyMaster ? 'Join as Spymaster' : 'Join as Operative'}
        </button>
      )}
    </div>
  );
};

function styleContainer(color: Team) {
  return css`
    display: flex;
    border-radius: 0.5rem;
    padding: 0.5rem;
    margin: 1rem;
    font-size: 0.5rem;
    flex-direction: column;
    color: ${Palette.contrast};
    background-color: ${color === 'Blue' ? Palette.blue : Palette.red};

    h2 {
      margin: 0;
    }

    ul {
      display: flex;
      flex-wrap: wrap;
      padding-left: 0.75rem;
    }

    li {
      margin: 0.25rem 0;
      padding-right: 0.5rem;
    }

    button {
      ${buttonStyle};
      font-size: 0.5rem;
      background-color: ${Palette.neutral};
      cursor: pointer;
      :hover {
        background-color: ${lighten(0.1, Palette.neutral)};
      }
      ${beginAt(375)} {
        font-size: 0.75rem;
      }

      ${beginAt(768)} {
        font-size: 1rem;
      }
    }

    ${beginAt(375)} {
      font-size: 0.75rem;
    }

    ${beginAt(768)} {
      font-size: 1rem;
    }
  `;
}

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
