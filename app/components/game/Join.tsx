/* eslint-disable no-alert */
import { lighten } from 'polished';
import {
  ChangeEventHandler,
  FC,
  MouseEventHandler,
  useCallback,
  useState,
} from 'react';
import { useRouter } from 'next/router';
import { Palette } from '../../design/color';
import { Player } from '../../model';
import { jsonHeaders, voidFetch } from '../../utils/fetch';

export interface JoinProps {
  game: string;
  API_URL: string;
}

export const Join: FC<JoinProps> = ({ game, API_URL }) => {
  const [player, setPlayer] = useState<Player>({
    team: 'Blue',
    is_spy_master: false,
    name: '',
  });
  const onChange = useCallback<ChangeEventHandler<HTMLInputElement>>(
    (e) => {
      setPlayer({ ...player, name: e.currentTarget.value });
    },
    [player],
  );

  const togglePlayerType = useCallback<
    MouseEventHandler<HTMLButtonElement>
  >(() => {
    setPlayer(nextPlayerType(player));
  }, [player]);

  const router = useRouter();
  const onJoin = useCallback(() => {
    if (!player.name) {
      alert('name is required');
      return;
    }
    voidFetch({
      url: `${API_URL}/game/${game}/join`,
      init: {
        method: 'PUT',
        body: JSON.stringify(player),
        headers: jsonHeaders,
      },
      onSuccess: () => router.push(`/game/${game}/${player.name}`),
      onError: () => alert('error joining game'),
    });
  }, [player, game, API_URL, router]);

  return (
    <div
      css={`
        display: flex;
        flex-direction: column;
        align-items: center;
        & button,
        input {
          width: 100%;
          padding: 0.5rem;
          margin: 0 0.5rem 1rem;
        }
      `}
    >
      <p>choose a team and role</p>
      <button
        type="button"
        onClick={togglePlayerType}
        css={`
          background: transparent;
          border-color: ${Palette.neutral};
          border-radius: 0.25rem;
          width: 5rem;
          color: ${player.team === 'Blue' ? Palette.blue : Palette.red};
          :focus {
            outline: none;
          }
        `}
      >
        {player.is_spy_master ? '⌐■-■' : '(•_•)'}
      </button>
      <p>your name</p>
      <input
        value={player.name}
        onChange={onChange}
        css={`
          border-radius: 0.25rem;
        `}
      />
      <button
        css={`
          background-color: ${Palette.neutral};
          padding: 0.5rem;
          border-radius: 0.25rem;
          margin: 0.5rem;
          width: 5rem;
          :hover {
            background-color: ${lighten(0.1, Palette.neutral)};
          }
        `}
        type="button"
        onClick={onJoin}
      >
        Join
      </button>
    </div>
  );
};

function nextPlayerType(player: Player): Player {
  const { team, is_spy_master } = player;
  if (is_spy_master) {
    return {
      ...player,
      is_spy_master: false,
      team: team === 'Blue' ? 'Red' : 'Blue',
    };
  }
  return {
    ...player,
    is_spy_master: true,
  };
}
