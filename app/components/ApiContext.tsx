import { createContext, FC, useContext, useState } from 'react';
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
  return (
    <>
      <Modal isOpen={!!error} onRequestClose={() => setError(null)}>
        The API that this site uses has thrown an error. That should not happen.
        If it does, and you are Matt, check the debugger.
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
