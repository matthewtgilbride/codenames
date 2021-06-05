import { FC, MouseEventHandler } from 'react';
import { css } from '@emotion/css';
import { darken } from 'polished';
import { Palette } from '../../design/color';
import { beginAt, Breakpoints } from '../../design/responsive';
import { CardColor, CardType, Player, Team } from '../../model';

const { neutral, death, blue, red, contrast } = Palette;
const { phoneLg, tabletPortrait } = Breakpoints;

const CardColorMap: { [key in CardType]: string } = {
  Neutral: neutral,
  Death: death,
  Blue: blue,
  Red: red,
};

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
      className={css`
        background-color: ${color ? CardColorMap[color] : Palette.light};
        box-shadow: 0 0 2px 1px ${Palette.blue};
        color: ${color === 'Death' ? neutral : contrast};
        border-radius: 0.25rem;
        padding: 0.75rem 0.1rem;
        word-break: break-all;
        font-size: ${size}px;
        cursor: ${isDisabled(turn, color, player) ? undefined : 'pointer'};
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
      `}
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
  if (player.is_spy_master) return true;
  return player.team !== turn;
}
