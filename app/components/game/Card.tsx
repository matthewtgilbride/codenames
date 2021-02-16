import { FC } from 'react';
import styled from 'styled-components';
import { Palette } from '../../design/color';
import { beginAt, Breakpoints } from '../../design/responsive';
import { CardColor, CardType } from '../../model';

const { neutral, death, blue, red, contrast } = Palette;
const { phoneLg, tabletPortrait } = Breakpoints;

const CardColorMap: { [key in CardType]: string } = {
  Neutral: neutral,
  Death: death,
  Blue: blue,
  Red: red,
};

const Container = styled.div<{ color: CardColor['color'] }>`
  background-color: ${(props) =>
    props.color ? CardColorMap[props.color] : 'white'};
  box-shadow: 0 0 2px 1px ${Palette.blue};
  color: ${(props) => (props.color === 'Death' ? neutral : contrast)};
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
