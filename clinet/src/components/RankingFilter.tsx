'use client';
import React, { useEffect, useState } from 'react';
import { getRankRecord } from '@/client_api/rank';
import { BookRank } from '@/types/book';
import { useRouter } from 'next/navigation';
import { Spin } from 'antd';
export type SortType =
  | 'like_num'
  | 'collect_num'
  | 'comment_num'
  | 'comment_long_num'
  | 'tap_num'
  | 'monthly_pass'
  | 'monthly_ticket_ranking'
  | 'reward_ranking';
interface RankingFilterProps {
  sortType: SortType;
  labelType: string;
}
// const searchParams = useSearchParams();
const RankingFilter: React.FC<RankingFilterProps> = ({
  sortType,
  labelType,
}) => {
  const router = useRouter();
  const [loading, setLoading] = useState(false);
  const [ranks, setRanks] = useState<BookRank[]>([]);
  const loadRankRecord = async () => {
    try {
      setLoading(true);
      const rankRes = await getRankRecord({
        current: 1,
        size: 10,
        sort_type: sortType,
        label_type: labelType,
        book_name: '',
      });
      if (rankRes.code === 'success') {
        setRanks(rankRes?.data?.list ?? []);
      }
    } catch (err) {
      console.log(err);
    } finally {
      setLoading(false);
    }
  };
  useEffect(() => {
    loadRankRecord();
  }, []);
  return (
    <Spin spinning={loading}>
      <div
        className={
          'book-ranks min-h-[120px] grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-6 gap-4'
        }>
        {ranks.map((item) => (
          <div
            onClick={() => router.push('/detail?bookId=' + item.b_id)}
            className="book cursor-pointer flex flex-col items-center"
            key={item.id}>
            {/* eslint-disable-next-line @next/next/no-img-element */}
            <img
              className={'w-full'}
              alt={item.book_name}
              src={item.cover_url}
            />
            <span className={'text-14px mt-2'}>
              <span>No.{item.rank}</span>
              {item.book_name}
            </span>
          </div>
        ))}
      </div>
    </Spin>
  );
};
export default RankingFilter;
