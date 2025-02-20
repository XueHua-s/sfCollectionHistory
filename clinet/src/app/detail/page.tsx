'use client';

import { useSearchParams } from 'next/navigation';
import { Suspense, useCallback, useEffect, useMemo, useState } from 'react';
import { DatePicker, Tooltip, Select, Button, Spin, message } from 'antd';
import { getBookDetail, getBookDetailHistory } from '@/client_api/detail';
import { BookInfo } from '@/types/book';
import { alignBookData, bookIsEunuch } from '@/untils';
import DataLineCharts from '@/components/DataLineCharts';
import dayjs from 'dayjs';
import BookSelect from '@/components/BookSelect';
import SuspenseSpin from '@/components/SuspenseSpin';

const { RangePicker } = DatePicker;
const { Option } = Select;

const getChartX = (books: BookInfo[]) => {
  return books.map((book: BookInfo) => book.created_time);
};

const getChartService = (name: string, books: BookInfo[], key: PropertyKey) => {
  return {
    name,
    type: 'line',
    data: books.map((book: any) => book[key]),
  };
};

const BookDetailPage = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const query = useSearchParams();
  const [datePicker, setDatePicker] = useState<any>([
    dayjs().subtract(365, 'day'),
    dayjs(), // Today
  ]);
  const [otherBookId, setOtherBookId] = useState('');
  const [otherBookDetail, setOtherBookDetail] = useState<BookInfo | null>(null);
  const [bookDetail, setBookDetail] = useState<BookInfo | null>(null);
  const [booksHistory, setBooksHistory] = useState<BookInfo[]>([]);
  const [otherBooksHistory, setOtherBooksHistory] = useState<BookInfo[]>([]);
  const [groupType, setGroupType] = useState<number>(1);
  const [loading, setLoading] = useState<boolean>(false);

  const loadBookDetail = async () => {
    const bookId = query.get('bookId');
    if (bookId) {
      setLoading(true);
      try {
        const data = await getBookDetail(bookId);
        if (data.code === 'success') {
          setBookDetail(data.data as BookInfo);
        }
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
      } catch (err: any) {
        messageApi.error('获取作品信息失败, 请前往提交入口, 提交收录');
      } finally {
        setLoading(false);
      }
    } else {
      message.error('Book not found');
    }
  };
  const loadOtherBookDetail = async () => {
    if (otherBookId) {
      setLoading(true);
      try {
        const data = await getBookDetail(otherBookId);
        if (data.code === 'success') {
          setOtherBookDetail(data.data as BookInfo);
        }
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
      } catch (err: any) {
        messageApi.error('获取作品信息失败, 请前往提交入口, 提交收录');
      } finally {
        setLoading(false);
      }
    } else {
      message.error('Book not found');
    }
  };
  const loadHistory = async (bookId: string) => {
    setLoading(true);
    try {
      const data = await getBookDetailHistory({
        b_id: Number(bookId),
        group_type: groupType,
        start_date: dayjs(datePicker[0]).format('YYYY-MM-DD'),
        end_date: dayjs(datePicker[1]).format('YYYY-MM-DD'),
      });
      if (data.code === 'success') {
        setBooksHistory(data.data as BookInfo[]);
      } else {
        message.error('Failed to load book history');
      }
    } finally {
      setLoading(false);
    }
  };
  const loadOtherHistory = async () => {
    if (!otherBookId) {
      return;
    }
    setLoading(true);
    try {
      const data = await getBookDetailHistory({
        b_id: Number(otherBookId),
        group_type: groupType,
        start_date: dayjs(datePicker[0]).format('YYYY-MM-DD'),
        end_date: dayjs(datePicker[1]).format('YYYY-MM-DD'),
      });
      if (data.code === 'success') {
        setOtherBooksHistory(data.data as BookInfo[]);
      } else {
        message.error('Failed to load book history');
      }
    } finally {
      setLoading(false);
    }
  };
  const bookStatus = useMemo(() => {
    if (bookDetail) {
      const oldOver = bookIsEunuch(
        bookDetail.last_update_time,
        bookDetail.finish,
      );
      if (oldOver) {
        return (
          <Tooltip
            title={'已太监, 作品数据将不再维护, 恢复更新后, 请手动提交维护。'}>
            已太监
          </Tooltip>
        );
      }
      return bookDetail.finish === 1 ? (
        <Tooltip
          title={'完结作品数据将不再维护, 状态如有更新, 请手动提交维护。'}>
          已完结
        </Tooltip>
      ) : (
        '连载中'
      );
    }
    return false;
  }, [bookDetail]);
  useEffect(() => {
    loadBookDetail();
    const bookId = query.get('bookId');
    if (bookId) {
      loadHistory(bookId);
    }
  }, []);
  const getBookDataLine = useCallback(
    (key: string, label: string) => {
      if (otherBooksHistory.length > 0 && otherBookId) {
        const { alignedArr1, alignedArr2 } = alignBookData(
          booksHistory,
          otherBooksHistory,
        );
        return (
          <DataLineCharts
            xData={getChartX(alignedArr1)}
            title={label}
            seriesData={[
              getChartService(bookDetail?.book_name ?? '', alignedArr1, key),
              getChartService(
                otherBookDetail?.book_name ?? '',
                alignedArr2,
                key,
              ),
            ]}
          />
        );
      } else {
        return (
          <DataLineCharts
            xData={getChartX(booksHistory)}
            title={label}
            seriesData={[
              getChartService(bookDetail?.book_name ?? '', booksHistory, key),
            ]}
          />
        );
      }
    },
    [booksHistory, bookDetail, otherBooksHistory, otherBookDetail],
  );
  return (
    <div>
      {contextHolder}
      <h1>作品详情/历史数据</h1>
      <p className={'text-primary'}>
        注意: 连载作品超过30天未更新, 状态视为太监,
        数据将不再进行维护。完结作品数据, 也将不再维护。
      </p>
      <p className={'text-primary mt-2'}>
        默认最大查询时间范围为: 1年；按年查询, 范围最大5年。
      </p>
      <div className="query mt-4 gap-4 custom-mobile:flex-col flex items-start">
        <div className="line flex items-center">
          <div className="label w-[100px]">时间范围:</div>
          <RangePicker
            value={datePicker}
            onChange={(dates) => setDatePicker(dates)}
          />
        </div>
        <div className="line flex items-center">
          <div className="label w-[100px]">分组类型:</div>
          <Select value={groupType} onChange={setGroupType}>
            <Option value={1}>天</Option>
            <Option value={2}>月</Option>
            <Option value={3}>年</Option>
          </Select>
        </div>
        <div className="line flex items-center">
          <div className="label w-[100px]">对比作品:</div>
          <div className="flex-1">
            <BookSelect
              className={'w-[200px]'}
              value={otherBookId}
              onChange={setOtherBookId}
            />
          </div>
        </div>
        <div>
          <Button
            type={'primary'}
            onClick={() => {
              loadHistory(query.get('bookId') as string);
              loadOtherHistory();
              loadOtherBookDetail();
            }}>
            查询
          </Button>
        </div>
      </div>
      <Spin spinning={loading}>
        <div
          className={
            'book-detail mt-4 p-2 grid custom-pc:grid-cols-2 gap-4 custom-mobile:grid-cols-1'
          }>
          <div className="detail shadow flex">
            <div className={'flex-1 mr-2'}>
              <img
                className={'w-full'}
                src={bookDetail?.cover_url}
                alt={'cover_url'}
              />
            </div>
            <div className="infos mt-4 flex-1 flex flex-col">
              <h2 className={'text-theme-brand mb-4 text-[24px]'}>
                {bookDetail?.book_name}
              </h2>
              <p className={'flex text-grayLine'}>
                <span className="label w-[40px]">字数:</span>
                {bookDetail?.word_count.toLocaleString()}
              </p>
              <p className={'flex mt-2 text-grayLine'}>
                <span className="label w-[40px]">类型:</span>
                {bookDetail?.book_type}
              </p>
              <p className={'flex text-grayLine mt-2'}>
                <span className="label w-[40px]">状态:</span>
                {bookStatus}
              </p>
            </div>
          </div>
          <div className="date-line shadow p-4">
            {getBookDataLine('tap_num', '点击数据')}
          </div>
          <div className="date-line shadow p-4">
            {getBookDataLine('like_num', '点赞数据')}
          </div>
          <div className="date-line shadow p-4">
            {getBookDataLine('collect_num', '收藏数据')}
          </div>
          <div className="date-line shadow p-4">
            {getBookDataLine('comment_num', '评论数据')}
          </div>
          <div className="date-line shadow p-4">
            {getBookDataLine('comment_long_num', '长评数据')}
          </div>
          <div className="date-line shadow p-4">
            {getBookDataLine('monthly_pass', '月票数据')}
          </div>
          <div className="date-line shadow p-4">
            {getBookDataLine('word_count', '字数数据')}
          </div>
          {/*<div className="date-line shadow p-4">*/}
          {/*  {getBookDataLine('monthly_ticket_ranking', '月票排行数据')}*/}
          {/*</div>*/}
          {/*<div className="date-line shadow p-4">*/}
          {/*  {getBookDataLine('reward_ranking', '打赏排行数据')}*/}
          {/*</div>*/}
        </div>
      </Spin>
    </div>
  );
};
const DetailPage = () => {
  return (
    <Suspense fallback={<SuspenseSpin />}>
      <BookDetailPage />
    </Suspense>
  );
};
export default DetailPage;
