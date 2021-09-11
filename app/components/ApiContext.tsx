import { createContext, FC, useContext, useEffect, useState } from 'react';
import { Modal } from '../design/Modal';

export interface ApiContextType {
  baseUrl: string;
  error: Error | Response | null;
  setError: (error: Error | Response | null) => void;
}

export const ApiContext = createContext<ApiContextType | undefined>(undefined);

export const ApiContextProvider: FC<{ baseUrl: string }> = ({
  baseUrl,
  children,
}) => {
  const [error, setError] = useState<Error | Response | null>(null);
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
      <Modal isOpen={!!error} onRequestClose={() => setError(null)}>
        {text}
      </Modal>
      <ApiContext.Provider value={{ baseUrl, error, setError }}>
        {children}
      </ApiContext.Provider>
    </>
  );
};

export const useApiContext = () => {
  const apiContext = useContext(ApiContext);
  if (!apiContext)
    throw new Error('useApiContext must be used within an ApiContextProvider');
  return apiContext;
};
