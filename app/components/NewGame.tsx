import { ChangeEventHandler, FC, useCallback, useState } from 'react';
import styled from 'styled-components';
import { lighten } from 'polished';
import { Breakpoints } from '../design/responsive';
import { Palette } from '../design/color';

const { phone } = Breakpoints;
const { red } = Palette;

const Container = styled.div`
  & input {
    max-width: ${phone / 2}px;
    margin: 0.5rem;
    padding: 0.5rem;
    border-radius: 0.25rem;
  }
  & button {
    min-width: ${phone / 4}px;
    background-color: ${red};
    padding: 0.5rem;
    border-radius: 0.25rem;
    line-height: 1.15;
    :hover {
      background-color: ${lighten(0.1, red)};
    }
  }
`;

interface NewGameProps {
  initialName: string;
}

export const NewGame: FC<NewGameProps> = ({ initialName }) => {
  const [name, setName] = useState(initialName);
  const onChange = useCallback<ChangeEventHandler<HTMLInputElement>>((e) => {
    setName(e.currentTarget.value);
  }, []);

  return (
    <Container>
      <input value={name} onChange={onChange} />
      <button type="button">Start</button>
    </Container>
  );
};
