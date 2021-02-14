import { ChangeEventHandler, FC, useCallback, useState } from 'react';
import { GetServerSideProps } from 'next';
import styled from 'styled-components';
import Link from 'next/link';
import { lighten } from 'polished';
import { getConstants } from '../constants';
import { Breakpoints } from '../design/responsive';
import { Palette } from '../design/color';

const { tabletPortrait, phone } = Breakpoints;
const { neutral, blue, red, contrast } = Palette;

const Container = styled.div`
  background-color: ${neutral};
  display: flex;
  flex-direction: column;
  margin: auto;
  padding: 4rem;
  border-radius: 1rem;
  max-width: ${tabletPortrait}px;
  text-align: center;
  box-shadow: 0 0 2px 1px ${blue};
  color: ${contrast};
  a,
  input,
  button {
    color: ${contrast};
  }
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

interface HomeProps {
  game_name: string;
}

const Home: FC<HomeProps> = ({ game_name }) => {
  const [name, setName] = useState(game_name);
  const onChange = useCallback<ChangeEventHandler<HTMLInputElement>>((e) => {
    setName(e.currentTarget.value);
  }, []);
  return (
    <Container>
      <div>
        <h2>Create a new game</h2>
        <input value={name} onChange={onChange} />
        <button type="button">Start</button>
      </div>
      <h2>
        Or <Link href="/game">join an existing one</Link>
      </h2>
    </Container>
  );
};

export const getServerSideProps: GetServerSideProps<HomeProps> = async () => {
  const { API_BASE_URL } = getConstants();
  const result = await fetch(API_BASE_URL);
  const json = await result.json();

  return { props: json as HomeProps };
};

export default Home;
