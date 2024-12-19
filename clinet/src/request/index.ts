import axios, { AxiosInstance } from 'axios';
import { beforEach, beforEachErr } from '@/request/beforEach';
import { responseEach, responseEachErr } from '@/request/reseponseEach';
const request: AxiosInstance = axios.create({
  timeout: 180000,
});
// 请求拦截
request.interceptors.request.use(beforEach, beforEachErr);
// 响应拦截
request.interceptors.response.use(responseEach, responseEachErr);
export default request;
