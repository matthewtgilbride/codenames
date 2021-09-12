/* eslint-disable no-alert,no-restricted-globals */
import { FC } from 'react';
import { css } from '@emotion/css';
import {
  currentTurn,
  GameState,
  getFirstTeam,
  getGuesses,
  Player,
  Turn,
} from '../../../model';
import { PlayerList } from './PlayerList';
import { container } from './Info.styles';
import { Palette } from '../../../design/color';
import { Action } from './action/Action';

export interface InfoProps {
  word?: string;
  player?: Player;
  game: GameState;
}

export const Info: FC<InfoProps> = ({ player, game }) => {
  const turn = currentTurn(game);
  const firstTeam = getFirstTeam(game);

  const blueCount = getGuesses(game).filter(
    (i) => game.board[i].color === 'Blue',
  ).length;
  const redCount = getGuesses(game).filter((i) => game.board[i].color === 'Red')
    .length;

  return (
    <div className={container}>
      <div
        className={css`
          color: ${Palette.blue};
        `}
      >
        {blueCount} / {firstTeam === 'Blue' ? 9 : 8}
      </div>
      <div>
        <h3>{heading(turn)}</h3>
      </div>
      <div
        className={css`
          color: ${Palette.red};
        `}
      >
        {redCount} / {firstTeam === 'Red' ? 9 : 8}
      </div>
      <div>
        <PlayerList
          spyMaster={false}
          team="Blue"
          game={game}
          playerName={player?.name}
        />
        <PlayerList
          spyMaster
          team="Blue"
          game={game}
          playerName={player?.name}
        />
      </div>
      <Action game={game} player={player} />
      <div>
        <PlayerList
          spyMaster={false}
          team="Red"
          game={game}
          playerName={player?.name}
        />
        <PlayerList
          spyMaster
          team="Red"
          game={game}
          playerName={player?.name}
        />
      </div>
    </div>
  );
};

function heading(turn: Turn): string {
  if (turn.type === 'Pending') {
    return `Waiting for ${turn.data} Spymaster`;
  }
  return `${turn.data.clue[0]} (${turn.data.clue[1]})`;
}
