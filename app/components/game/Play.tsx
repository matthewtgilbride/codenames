/* eslint-disable no-alert */
import { lighten } from 'polished';
import { FC } from 'react';
import { Palette } from '../../design/color';
import { Player } from '../../model';

export interface JoinProps {
  player: Player;
  API_URL: string;
}

export const Play: FC<JoinProps> = ({ player, API_URL }) => (
  <div
    css={`
      display: flex;
      flex-direction: column;
      align-items: center;
    `}
  >
    {!player.is_spy_master && (
      <button
        type="button"
        css={`
          background: transparent;
          border-color: ${Palette.neutral};
          border-radius: 0.25rem;
          padding: 0.5rem;
          margin: 0.5rem;
          width: 5rem;
          color: ${player.team === 'Blue' ? Palette.blue : Palette.red};
          :focus {
            outline: none;
          }
        `}
      >
        Confirm Guess:
      </button>
    )}
    <button
      css={`
        background-color: ${Palette.neutral};
        padding: 0.5rem;
        border-radius: 0.25rem;
        margin: 0.5rem;
        width: 5rem;
        :hover {
          background-color: ${lighten(0.1, Palette.neutral)};
        }
      `}
      type="button"
    >
      End Turn
    </button>
  </div>
);
