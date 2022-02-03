import { useEffect, useState } from 'react';
import { voidFetch, VoidFetchConfig } from '../utils/fetch';

/**
 * terrible hack because of two things:
 *  1. Next doesn't populate the query parameter on routes the first go-around (see: "rehydration")
 *  2. This "ApiContext" monstrosity I've created changes on every render, so putting it inside a render
 *  function causes the component to thrash
 */
export const useFetchOnce = (config: VoidFetchConfig, ready: boolean) => {
  const [done, setDone] = useState(false);
  useEffect(() => {
    if (ready && !done) {
      setDone(true);
      voidFetch(config);
    }
  }, [config, ready, done]);
};
