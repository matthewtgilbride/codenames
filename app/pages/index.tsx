import { FC } from 'react';
import Link from 'next/link';
import { css } from '@emotion/css';
import { NewGameContainer } from '../components/NewGame';
import { Palette } from '../design/color';
import { Breakpoints } from '../design/responsive';

const Home: FC = () => (
  <div className={styleContent}>
    <div>
      <h2>create a new game</h2>
      <NewGameContainer />
    </div>
    <h2>
      or <Link href="/game">join an existing one</Link>
    </h2>
  </div>
);

const { light, blue, contrast } = Palette;

export const styleContent = css`
  background-color: ${light};
  display: flex;
  flex-direction: column;
  margin: auto;
  padding: 1rem;
  border-radius: 1rem;
  box-shadow: 0 0 2px 1px ${blue};
  color: ${contrast};
  max-width: ${Breakpoints.tabletPortrait}px;
  text-align: center;
  * {
    color: ${contrast};
  }
`;

export default Home;
