import { ApiContextType } from '../components/ApiContext';

export interface VoidFetchConfig {
  apiContext: ApiContextType;
  path: string;
  init?: RequestInit;
  onSuccess?: (r: Response) => void;
  remainLoadingOnSuccess?: boolean;
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
  response: Response | Error,
  apiContext: ApiContextType,
  onError?: VoidFetchConfig['onError'],
): void => {
  apiContext.setError(response);
  apiContext.setLoading(false);
  if (onError) onError(response);
};

const onSuccessWithContext = (
  response: Response,
  apiContext: ApiContextType,
  onSuccess?: VoidFetchConfig['onSuccess'],
) => {
  apiContext.setError(null);
  apiContext.setLoading(false);
  if (onSuccess) {
    onSuccess(response);
    // eslint-disable-next-line no-console
  } else console.log(response);
};

export const url = (apiContext: ApiContextType, path?: string) =>
  `${apiContext.baseUrl}${path}`;

export const voidFetch = ({
  apiContext,
  path,
  init,
  onSuccess,
  remainLoadingOnSuccess,
  onError,
}: VoidFetchConfig): void => {
  apiContext.setLoading(true);
  fetch(url(apiContext, path), initOrDefault(init))
    .then((response) => {
      if (response.ok) {
        onSuccessWithContext(response, apiContext, onSuccess);
        if (!remainLoadingOnSuccess) apiContext.setLoading(false);
      } else {
        onErrorWithContext(response, apiContext, onError);
      }
    })
    .catch((e) => onErrorWithContext(e, apiContext, onError));
};
