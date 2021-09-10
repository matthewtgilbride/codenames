import { createContext, FC, useContext, useState } from 'react';
import { Modal } from '../design/Modal';

export interface ApiContextType {
  baseUrl: string;
  error: boolean;
  setError: (error: boolean) => void;
}

export const ApiContext = createContext<ApiContextType | undefined>(undefined);

export const ApiContextProvider: FC<{ baseUrl: string }> = ({
  baseUrl,
  children,
}) => {
  const [error, setError] = useState(false);
  return (
    <>
      <Modal isOpen={error} onRequestClose={() => setError(false)}>
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
