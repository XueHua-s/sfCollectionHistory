export interface ResponseData<T> {
  code: 'success' | 'error';
  data?: T;
  message?: T;
  error?: T;
}
export interface QueryList<T> {
  current: number;
  size: number;
  list: T[];
  total_num: number;
  total_page: number;
}
