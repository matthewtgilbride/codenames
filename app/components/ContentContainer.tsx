import styled from 'styled-components';
import { Palette } from '../design/color';
import { Breakpoints } from '../design/responsive';

const { neutral, blue, contrast } = Palette;

export const ContentContainer = styled.div`
  background-color: ${neutral};
  display: flex;
  flex-direction: column;
  margin: auto;
  border-radius: 1rem;
  box-shadow: 0 0 2px 1px ${blue};
  color: ${contrast};
  max-width: ${Breakpoints.tabletPortrait}px;
  text-align: center;
  * {
    color: ${contrast};
  }
`;
