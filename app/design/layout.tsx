import { FC } from 'react';
import Link from 'next/link';
import { css } from '@emotion/css';
import { Palette } from './color';
import { beginAt, Breakpoints } from './responsive';

const { phoneMd, phoneLg, tabletPortrait } = Breakpoints;

export const Layout: FC = ({ children }) => (
  <div className={container}>
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
  background-color: ${Palette.contrast};
`;

const title = css`
  text-align: center;
  color: ${Palette.neutral};
  font-size: 1rem;
  a {
    color: ${Palette.neutral};
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
  color: ${Palette.neutral};
  font-size: 0.5rem;
  margin: 0.5rem;
  a {
    color: ${Palette.neutral};
  }
`;
