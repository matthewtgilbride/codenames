import { FC, MouseEventHandler } from 'react';
import { css } from '@emotion/css';
import { darken } from 'polished';
import { Palette } from '../../design/color';
import { beginAt, Breakpoints } from '../../design/responsive';
import {
  CardColor,
  CardType,
  isSpyMaster,
  Player,
  Turn,
  turnTeam,
} from '../../model';
import { buttonStyle } from '../../design/button';

interface CardProps {
  card: CardColor;
  turn: Turn;
  player?: Player;
  onClick: MouseEventHandler<HTMLButtonElement>;
  guessNumber?: number;
}

export const Card: FC<CardProps> = ({
  turn,
  player,
  card: { color, word },
  onClick,
  guessNumber,
}) => {
  const size = word.length > 5 ? (1 / word.length) * 72 : 12;
  return (
    <button
      type="button"
      onClick={onClick}
      disabled={isDisabled(turn, color, player)}
      className={styleButton(turn, color, size, player)}
    >
      {guessNumber && <p>{guessNumber}</p>}
      {word}
    </button>
  );
};

function isDisabled(
  turn: Turn,
  color: CardType | null,
  player?: Player,
): boolean {
  if (turn.type === 'Pending') return true;
  if (color) return true;
  if (!player) return true;
  if (isSpyMaster(player)) return true;
  return player.team !== turnTeam(turn);
}

function styleButton(
  turn: Turn,
  color: CardType | null,
  size: number,
  player?: Player,
): string {
  return css`
    ${buttonStyle};
    padding: 0.75rem 0.1rem;
    word-break: break-all;
    position: relative;
    background-color: ${color ? CardColorMap[color] : Palette.light};
    color: ${color === 'Death' ? neutral : contrast};
    font-size: ${size}px;
    cursor: ${isDisabled(turn, color, player) ? 'initial' : 'pointer'};

    :hover {
      background-color: ${isDisabled(turn, color, player)
        ? undefined
        : darken(0.1, Palette.light)};
    }

    ${beginAt(phoneLg)} {
      font-size: ${size * 1.5}px;
    }

    ${beginAt(tabletPortrait)} {
      font-size: ${size * 2}px;
      padding: 1rem 0.1rem;
    }

    & p {
      font-size: 0.3rem;
      display: flex;
      justify-content: center;
      align-items: center;
      width: 0.8rem;
      height: 0.8rem;
      border-radius: 50%;
      color: ${Palette.light};
      background-color: ${Palette.gray};
      position: absolute;
      top: 2%;
      left: 2%;
    }
  `;
}

const { neutral, death, blue, red, contrast } = Palette;
const { phoneLg, tabletPortrait } = Breakpoints;

export const CardColorMap: { [key in CardType]: string } = {
  Neutral: neutral,
  Death: death,
  Blue: blue,
  Red: red,
};
