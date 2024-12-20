import type { AxiosError, AxiosResponse } from 'axios';
import { message } from 'antd';
export const responseEach = (res: AxiosResponse) => {
  if (res.data instanceof Blob) {
    return res;
  }
  if (res.data.code === 'success' && res.data.data) {
    return res.data;
  } else {
    // 展示消息
    message.info(res.data.message);
    return res.data;
  }
};
export const responseEachErr = async (err: AxiosError) => {
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-expect-error
  if (err?.response?.data.code === 'error') {
    message.error((err.response.data as any).error as string);
  }
  throw err;
};
