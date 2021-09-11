import { ApiContextType } from '../components/ApiContext';

export interface VoidFetchConfig {
  apiContext: ApiContextType;
  path: string;
  init?: RequestInit;
  onSuccess?: (r: Response) => void;
  onError?: (r: Response | Error) => void;
}

export const jsonHeaders = {
  'content-type': 'application/json',
};

export const initOrDefault = (init?: RequestInit): RequestInit => ({
  headers: jsonHeaders,
  ...init,
});

export const onErrorWithContext = (
  apiContext: ApiContextType,
  onError?: VoidFetchConfig['onError'],
): ((r: Response | Error) => void) => (e) => {
  apiContext.setError(e);
  if (onError) onError(e);
};

const onSuccessOrDefault = (onSuccess?: VoidFetchConfig['onSuccess']) =>
  // eslint-disable-next-line no-console
  onSuccess ?? ((r) => console.log(r));

export const url = (apiContext: ApiContextType, path?: string) =>
  `${apiContext.baseUrl}${path}`;

export const voidFetch = ({
  apiContext,
  path,
  init,
  onSuccess,
  onError,
}: VoidFetchConfig): void => {
  fetch(url(apiContext, path), initOrDefault(init))
    .then((response) => {
      if (response.ok) {
        onSuccessOrDefault(onSuccess)(response);
      } else {
        onErrorWithContext(apiContext, onError)(response);
      }
    })
    .catch((e) => onErrorWithContext(apiContext, onError)(e));
};
