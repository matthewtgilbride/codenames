import React, { FC, useState } from 'react';
import { css } from '@emotion/css';
import { darken } from 'polished';
import {
  currentTeam,
  currentTurn,
  GameState,
  getGuesses,
  isSpyMaster,
  Player,
} from '../../../model';
import { GuessLog } from './GuessLog';
import { beginAt } from '../../../design/responsive';
import { Palette } from '../../../design/color';
import { Clue } from './Clue';

export interface ActionProps {
  game: GameState;
  player?: Player;
  onEndTurn: () => void;
  onLeave: () => void;
}

export const Action: FC<ActionProps> = ({
  game,
  game: { board },
  player,
  onEndTurn,
  onLeave,
}) => {
  const [open, setOpen] = useState(false);
  return (
    <div className={action}>
      {player && isSpyMaster(player) && currentTurn(game).type === 'Pending' && (
        <>
          <button
            type="button"
            onClick={() => setOpen(true)}
            className={actionButton}
          >
            Start Turn
          </button>
          <Clue
            game={game}
            spyMaster={player}
            isOpen={open}
            onRequestClose={() => setOpen(false)}
            team={currentTeam(game)}
          />
        </>
      )}
      {player && currentTurn(game).type === 'Started' && (
        <button type="button" onClick={onEndTurn} className={actionButton}>
          End Turn
        </button>
      )}
      <GuessLog board={board} guesses={getGuesses(game)} />
      {player && (
        <button type="button" onClick={onLeave} className={actionButton}>
          Leave Game
        </button>
      )}
    </div>
  );
};

const action = css`
  display: flex;
  flex-direction: column;
  font-size: 0.5rem;
  ${beginAt(375)} {
    font-size: 0.75rem;
  }
  ${beginAt(768)} {
    font-size: 1rem;
  }
  & button {
    font-size: 0.5rem;
    ${beginAt(375)} {
      font-size: 0.75rem;
    }
    ${beginAt(768)} {
      font-size: 1rem;
    }
  }
`;

const actionButton = css`
  background-color: ${Palette.light};
  padding: 0.5rem;
  border-radius: 0.25rem;
  margin: 0.5rem 0;
  width: 100%;
  :hover {
    background-color: ${darken(0.1, Palette.light)};
  }
`;
