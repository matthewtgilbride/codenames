export interface Constants {
  API_BASE_URL: string;
}

export const getConstants = (): Constants => {
  const { API_HOST, API_PORT } = process.env;
  if (!API_HOST || !API_PORT)
    throw new Error('API_HOST and API_PORT environment variables are required');
  const API_BASE_URL = `http://${API_HOST}:${API_PORT}`;
  return { API_BASE_URL };
};
