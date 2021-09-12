import { useEffect, useState } from 'react';
import {
  initOrDefault,
  onErrorWithContext,
  url,
  VoidFetchConfig,
} from '../utils/fetch';
import { useApiContext } from '../components/ApiContext';

export type PollConfig<T> = Omit<VoidFetchConfig, 'onSuccess'> & {
  onSuccess: (result: T) => void;
};

export const usePoll = <T>(config: PollConfig<T>) => {
  const apiContext = useApiContext();
  const [error, setError] = useState(false);
  const [pollCount, setPollCount] = useState(0);
  useEffect(() => {
    const doFetch = async () => {
      try {
        const result = await fetch(
          url(apiContext, config.path),
          initOrDefault(config.init),
        );
        if (result.ok) {
          const json = await result.json();
          config.onSuccess(json as T);
        } else {
          setError(true);
          onErrorWithContext(result, apiContext, config.onError);
        }
      } catch (e) {
        setError(true);
        onErrorWithContext(e, apiContext, config.onError);
      }
    };
    const i = setInterval(() => {
      if (!error) {
        doFetch();
        setPollCount(pollCount + 1);
      }
    }, 3000);
    return () => clearInterval(i);
  }, [error, pollCount, config, apiContext]);
};
