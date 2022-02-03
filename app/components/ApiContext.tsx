import { createContext, FC, useContext, useEffect, useState } from 'react';
import { css } from '@emotion/css';
import { Modal } from '../design/Modal';
import { API_URL } from '../constants';
import { overlayColor } from '../design/color';

export interface ApiContextType {
  baseUrl: string;
  loading: boolean;
  setLoading: (isLoading: boolean) => void;
  error: Error | Response | null;
  setError: (error: Error | Response | null) => void;
}

export const ApiContext = createContext<ApiContextType | undefined>(undefined);

export const ApiContextProvider: FC<{ baseUrl?: string }> = ({
  baseUrl = API_URL,
  children,
}) => {
  const [error, setError] = useState<Error | Response | null>(null);
  const [loading, setLoading] = useState(false);
  const [text, setText] = useState('');
  useEffect(() => {
    if (error instanceof Error) {
      setText(error.message);
    }
    if (error instanceof Response) {
      error.text().then((t) => setText(t));
    }
  }, [error]);
  return (
    <>
      {loading && <LoadingDimmer />}
      <Modal isOpen={!!error} onRequestClose={() => setError(null)}>
        {text}
      </Modal>
      <ApiContext.Provider
        value={{ baseUrl, loading, setLoading, error, setError }}
      >
        {children}
      </ApiContext.Provider>
    </>
  );
};

const LoadingDimmer: FC = () => (
  <div
    className={css`
      position: absolute;
      top: 0;
      right: 0;
      bottom: 0;
      left: 0;
      background-color: ${overlayColor};
      z-index: 1000;
    `}
  />
);

export const useApiContext = () => {
  const apiContext = useContext(ApiContext);
  if (!apiContext)
    throw new Error('useApiContext must be used within an ApiContextProvider');
  return apiContext;
};
