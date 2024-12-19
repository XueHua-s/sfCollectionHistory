import request from '@/request';
import { QueryList, ResponseData } from '@/types/response';
import { BookInfo, BookRank } from '@/types/book';
interface rankRecordProps {
  current: number;
  size: number;
  book_name: string;
  sort_type: string;
  label_type: string;
}
export const getRankRecord = (
  data: rankRecordProps,
): Promise<ResponseData<QueryList<BookRank>>> =>
  request({
    method: 'POST',
    url: '/api/books/rank/records',
    data,
  });
