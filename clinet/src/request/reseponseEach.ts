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
  console.log(err);
  if (err?.response?.data.code === 'error') {
    message.error(err.response.data.error);
  }
  throw err;
};
