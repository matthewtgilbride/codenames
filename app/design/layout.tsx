import { FC, useLayoutEffect } from 'react';
import Link from 'next/link';
import { css } from '@emotion/css';
import ReactModal from 'react-modal';
import { lighten } from 'polished';
import { Palette } from './color';
import { beginAt, Breakpoints } from './responsive';

const { phoneMd, phoneLg, tabletPortrait } = Breakpoints;

export const Layout: FC = ({ children }) => {
  useLayoutEffect(() => ReactModal.setAppElement('#app'), []);
  return (
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
        <a href="https://www.mattgilbride.com">
          friendly neighborhood developer
        </a>
      </p>
      {children}
    </div>
  );
};

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
    ${Palette.light} 0%,
    ${lighten(0.1, Palette.contrast)} 20%,
    ${Palette.contrast} 40%,
    ${Palette.contrast} 60%,
    ${lighten(0.1, Palette.contrast)} 80%,
    ${Palette.light} 100%
  );
`;

const title = css`
  text-align: center;
  color: ${Palette.light};
  font-size: 1rem;
  margin: 1.5rem 0 1rem 0;
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
