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
  const onSubmit = useCallback(() => {
    if (!player.name) {
      alert('name is required');
      return;
    }
    fetch(`${API_URL}/game/${game}/join`, {
      method: 'PUT',
      body: JSON.stringify(player),
      headers: { 'content-type': 'application/json' },
    })
      .then((response) => {
        if (response.ok) {
          router.push(`/game/${game}/${player.name}`);
        } else {
          alert('error joining game');
        }
      })
      .catch(() => alert('error joining game'));
  }, [player]);

  return (
    <div
      css={`
        display: flex;
        flex-direction: column;
        align-items: center;
      `}
    >
      <button
        type="button"
        onClick={togglePlayerType}
        css={`
          background: transparent;
          border-color: ${Palette.neutral};
          border-radius: 0.25rem;
          padding: 0.5rem;
          margin: 0.5rem;
          width: 5rem;
          color: ${player.team === 'Blue' ? Palette.blue : Palette.red};
          :focus {
            outline: none;
          }
        `}
      >
        {player.is_spy_master ? '⌐■-■' : '(•_•)'}
      </button>
      <input
        value={player.name}
        onChange={onChange}
        css={`
          margin: 0.5rem;
          padding: 0.5rem;
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
        onClick={onSubmit}
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
