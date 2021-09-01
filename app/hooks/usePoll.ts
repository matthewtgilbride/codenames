import { useEffect, useState } from 'react';
import { VoidFetchConfig } from '../utils/fetch';

export type PollConfig<T> = Omit<VoidFetchConfig, 'onSuccess'> & {
  onSuccess: (result: T) => void;
};

export const usePoll = <T>(config: PollConfig<T>) => {
  const [error, setError] = useState(false);
  const [pollCount, setPollCount] = useState(0);
  useEffect(() => {
    const doFetch = async () => {
      try {
        const result = await fetch(config.url, config.init);
        if (result.ok) {
          const json = await result.json();
          config.onSuccess(json as T);
        } else {
          setError(true);
          config.onError();
        }
      } catch (e) {
        setError(true);
        config.onError();
      }
    };
    const i = setInterval(() => {
      if (!error) {
        doFetch();
        setPollCount(pollCount + 1);
      }
    }, 3000);
    return () => clearInterval(i);
  }, [error, pollCount, config]);
};
