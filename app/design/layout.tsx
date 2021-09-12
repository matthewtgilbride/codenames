import { FC } from 'react';
import Link from 'next/link';
import { css } from '@emotion/css';
import { lighten } from 'polished';
import { Palette } from './color';
import { beginAt, Breakpoints } from './responsive';

const { phoneMd, phoneLg, tabletPortrait } = Breakpoints;

export const Layout: FC = ({ children }) => (
  <div className={container} id="app">
    <h1 className={title}>
      (•_•) ( •_•)
      <Link prefetch={!process.env.STORYBOOK} href="/">
        Codenames
      </Link>
      ⌐■-■ (⌐■_■)
    </h1>
    <p className={subtitle}>
      brought to you by your{' '}
      <a href="https://www.mattgilbride.com">friendly neighborhood developer</a>
    </p>
    {children}
  </div>
);

const container = css`
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  overflow-y: auto;
  padding: 0 1rem;
  background: radial-gradient(
    circle at top left,
    ${lighten(0.5, Palette.contrast)} 0%,
    ${lighten(0.4, Palette.contrast)} 10%,
    ${lighten(0.2, Palette.contrast)} 20%,
    ${Palette.contrast} 50%,
    ${lighten(0.2, Palette.contrast)} 80%,
    ${lighten(0.4, Palette.contrast)} 90%,
    ${lighten(0.5, Palette.contrast)} 100%
  );
`;

const title = css`
  text-align: center;
  color: ${Palette.light};
  font-size: 1rem;
  margin: 1.5rem 0 0 0;
  a {
    color: ${Palette.light};
    margin: 0 1rem;
  }
  ${beginAt(phoneMd)} {
    font-size: 1.25rem;
  }
  ${beginAt(phoneLg)} {
    font-size: 1.5rem;
  }
  ${beginAt(tabletPortrait)} {
    font-size: 2rem;
  }
`;

const subtitle = css`
  text-align: center;
  color: ${Palette.light};
  font-size: 0.5rem;
  margin: 0.5rem;
  a {
    color: ${Palette.light};
  }
`;
