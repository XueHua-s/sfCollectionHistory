// 查询征文类型
import { BookInfo } from '@/types/book';
import { ResponseData } from '@/types/response';
import request from '@/request';
// 获取书本详情
export const getBookDetail = (
  bookId: string,
): Promise<ResponseData<BookInfo>> =>
  request({
    method: 'GET',
    url: `/api/books/detail/${bookId}`,
  });
interface GetBookDetailHistoryProps {
  start_date: string;
  end_date: string;
  group_type: number;
  b_id: number;
}
// 获取书本历史记录
export const getBookDetailHistory = (
  data: GetBookDetailHistoryProps,
): Promise<ResponseData<BookInfo[]>> =>
  request({
    url: '/api/books/analysis/records',
    method: 'POST',
    data,
  });
