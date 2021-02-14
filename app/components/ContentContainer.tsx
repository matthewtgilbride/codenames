import styled from 'styled-components';
import { Palette } from '../design/color';

const { neutral, blue, contrast } = Palette;

export const ContentContainer = styled.div`
  background-color: ${neutral};
  display: flex;
  flex-direction: column;
  margin: auto;
  padding: 1rem;
  border-radius: 1rem;
  box-shadow: 0 0 2px 1px ${blue};
  color: ${contrast};
  max-width: 90%;
  text-align: center;
  * {
    color: ${contrast};
  }
`;
