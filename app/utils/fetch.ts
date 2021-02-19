export interface VoidFetchConfig {
  url: string;
  init?: RequestInit;
  onSuccess: () => void;
  onError: () => void;
}

export const jsonHeaders = {
  'content-type': 'application/json',
};

export const voidFetch = ({
  url,
  init,
  onSuccess,
  onError,
}: VoidFetchConfig): void => {
  fetch(url, init)
    .then((response) => {
      if (response.ok) {
        onSuccess();
      } else {
        onError();
      }
    })
    .catch(() => onError);
};
