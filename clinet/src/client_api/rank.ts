import request from '@/request';
import { QueryList, ResponseData } from '@/types/response';
import { BookRank } from '@/types/book';
interface rankRecordProps {
  current: number;
  size: number;
  book_name: string;
  sort_type: string;
  label_type: string;
}
// 查询队列排名
export const getRankRecord = (
  data: rankRecordProps,
): Promise<ResponseData<QueryList<BookRank>>> =>
  request({
    method: 'POST',
    url: '/api/books/rank/records',
    data,
  });
// 查询征文类型
export const getLabelsRecord = (
  keyword: string,
): Promise<ResponseData<string[]>> =>
  request({
    method: 'POST',
    url: '/api/books/label/query',
    data: {
      keyword,
    },
  });
