import { ChangeEventHandler, FC, useCallback, useState } from 'react';
import { lighten } from 'polished';
import { css } from '@emotion/css';
import { useNavigate } from 'react-router-dom';
import { Breakpoints } from '../design/responsive';
import { Palette } from '../design/color';
import { voidFetch } from '../utils/fetch';
import { useApiContext } from './ApiContext';
import { useFetchOnce } from '../hooks/useFetch';

const { phone } = Breakpoints;
const { red } = Palette;

interface NewGameProps {
  initialName: string;
}

export const NewGame: FC<NewGameProps> = ({ initialName }) => {
  const [name, setName] = useState(initialName);
  const onChange = useCallback<ChangeEventHandler<HTMLInputElement>>((e) => {
    setName(e.currentTarget.value);
  }, []);

  const apiContext = useApiContext();

  const navigate = useNavigate();
  const onSubmit = useCallback(() => {
    voidFetch({
      apiContext,
      path: '/game',
      init: {
        method: 'POST',
        body: JSON.stringify({ game_name: name }),
      },
      onSuccess: () => navigate(`/game/${name}`),
    });
  }, [name, navigate, apiContext]);

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

export const NewGameContainer = () => {
  const apiContext = useApiContext();
  const [gameName, setGameName] = useState<null | string>(null);
  useFetchOnce(
    {
      apiContext,
      path: '',
      onSuccess: (r) =>
        r.json().then((payload) => setGameName(payload.game_name as string)),
    },
    true,
  );

  return gameName === null ? null : <NewGame initialName={gameName} />;
};
