import type { AxiosError, InternalAxiosRequestConfig } from 'axios';
export const beforEach = async (config: InternalAxiosRequestConfig) => {
  config.baseURL = process.env.NEXT_PUBLIC_API_URL;
  return config;
};

export const beforEachErr = (err: AxiosError) => {
  return err;
};
