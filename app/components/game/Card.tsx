import { FC, MouseEventHandler } from 'react';
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
}) => (
  <button
    type="button"
    onClick={onClick}
    disabled={isDisabled(turn, color, player)}
    css={`
      background-color: ${color ? CardColorMap[color] : 'white'};
      box-shadow: 0 0 2px 1px ${Palette.blue};
      color: ${color === 'Death' ? neutral : contrast};
      border-radius: 0.25rem;
      padding: 1rem 0.1rem;
      word-break: break-all;
      font-size: 0.5rem;
      ${beginAt(phoneLg)} {
        font-size: 0.75rem;
      }
      ${beginAt(tabletPortrait)} {
        font-size: unset;
        padding: 2rem 0.1rem;
      }
    `}
  >
    {word}
  </button>
);

function isDisabled(turn: Team, color?: CardType, player?: Player): boolean {
  if (color) return true;
  if (!player) return true;
  if (player.is_spy_master) return true;
  return player.team !== turn;
}
