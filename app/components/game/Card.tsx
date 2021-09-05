import { FC, MouseEventHandler } from 'react';
import { css } from '@emotion/css';
import { darken } from 'polished';
import { Palette } from '../../design/color';
import { beginAt, Breakpoints } from '../../design/responsive';
import { CardColor, CardType, isSpyMaster, Player, Team } from '../../model';
import { buttonStyle } from '../../design/button';

interface CardProps {
  card: CardColor;
  turn: Team;
  player?: Player;
  onClick: MouseEventHandler<HTMLButtonElement>;
}

export const Card: FC<CardProps> = ({
  turn,
  player,
  card: { color, word },
  onClick,
}) => {
  const size = word.length > 5 ? (1 / word.length) * 72 : 12;
  return (
    <button
      type="button"
      onClick={onClick}
      disabled={isDisabled(turn, color, player)}
      className={styleButton(turn, color, size, player)}
    >
      {word}
    </button>
  );
};

function isDisabled(
  turn: Team,
  color: CardType | null,
  player?: Player,
): boolean {
  if (color) return true;
  if (!player) return true;
  if (isSpyMaster(player)) return true;
  return player.team !== turn;
}

function styleButton(
  turn: Team,
  color: CardType | null,
  size: number,
  player?: Player,
): string {
  return css`
    ${buttonStyle};
    word-break: break-all;
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
  `;
}

const { neutral, death, blue, red, contrast } = Palette;
const { phoneLg, tabletPortrait } = Breakpoints;

const CardColorMap: { [key in CardType]: string } = {
  Neutral: neutral,
  Death: death,
  Blue: blue,
  Red: red,
};
