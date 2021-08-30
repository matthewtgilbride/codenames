/* eslint-disable no-alert */
import { ChangeEventHandler, FC, useCallback, useState } from 'react';
import { lighten } from 'polished';
import { useRouter } from 'next/router';
import { css } from '@emotion/css';
import { Breakpoints } from '../design/responsive';
import { Palette } from '../design/color';
import { jsonHeaders, voidFetch } from '../utils/fetch';

const { phone } = Breakpoints;
const { red } = Palette;

interface NewGameProps {
  initialName: string;
  API_URL: string;
}

export const NewGame: FC<NewGameProps> = ({ initialName, API_URL }) => {
  const [name, setName] = useState(initialName);
  const onChange = useCallback<ChangeEventHandler<HTMLInputElement>>((e) => {
    setName(e.currentTarget.value);
  }, []);

  const router = useRouter();
  const onSubmit = useCallback(() => {
    voidFetch({
      url: `${API_URL}/game`,
      init: {
        method: 'POST',
        body: JSON.stringify({ game_name: name }),
        headers: jsonHeaders,
      },
      onSuccess: () => router.push(`/game/${name}`),
      onError: () => alert('error creating game'),
    });
  }, [name, API_URL, router]);

  return (
    <div className={containerStyle}>
      <input value={name} onChange={onChange} />
      <button type="button" onClick={onSubmit}>
        Start
      </button>
    </div>
  );
};

const containerStyle = css`
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
    :hover {
      background-color: ${lighten(0.1, red)};
    }
  }
`;
