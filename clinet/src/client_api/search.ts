import { ResponseData } from '@/types/response';
import request from '@/request';
import { BookInfo } from '@/types/book';
export interface MasterBook {
  b_id: number;
  clear_title: string;
  title: string;
  url: string;
}
// 获取书本历史记录
export const queryMasterBook = (
  keyword: string,
): Promise<ResponseData<MasterBook[]>> =>
  request({
    url: '/api/books/query/master_books',
    method: 'POST',
    data: {
      keyword,
    },
  });
// 收录作品
export const entranceBook = (bookId: string): Promise<ResponseData<BookInfo>> =>
  request({
    url: `/api/books/add/${bookId}`,
    method: 'POST',
  });
// 维护作品
export const maintenanceBook = (
  bookId: string,
): Promise<ResponseData<BookInfo>> =>
  request({
    url: `/api/books/maintenance/${bookId}`,
    method: 'POST',
  });
