import { FC } from 'react';
import styled from 'styled-components';
import { Palette } from '../../design/color';
import { beginAt, Breakpoints } from '../../design/responsive';

const { neutral, death, blue, red, contrast } = Palette;
const { phoneLg, tabletPortrait } = Breakpoints;

export type CardType = 'Neutral' | 'Death' | 'Blue' | 'Red';

const CardColorMap: { [key in CardType]: string } = {
  Neutral: neutral,
  Death: death,
  Blue: blue,
  Red: red,
};

export interface CardColor {
  color?: CardType;
  word: string;
}

const Container = styled.div<{ color: CardColor['color'] }>`
  background-color: ${(props) =>
    props.color ? CardColorMap[props.color] : 'white'};
  box-shadow: 0 0 2px 1px ${Palette.blue};
  color: ${contrast};
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
}
`;

export const Card: FC<CardColor> = ({ color, word }) => (
  <Container color={color}>{word}</Container>
);
