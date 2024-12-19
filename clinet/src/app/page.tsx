import RankingFilter from '@/components/RankingFilter';
import SuspenseSpin from '@/components/SuspenseSpin';
import { Suspense } from 'react';
export default function Home() {
  return (
    <div className={'flex flex-col'}>
      <div className="support">
        <h1 className={'text-theme-brand text-[24px]'}>我很可爱请给我钱</h1>
        <p className={'text-grayLine'}>
          开发不易, 服务器运维需要成本, 有能力请赞助。
        </p>
        <div className="qr-code flex">
          <img
            className={'w-[210px] mr-4'}
            src={'/img/mm_facetoface_collect_qrcode_1734620059008.png'}
            alt={'微信赞助'}
          />
          <img
            className={'w-[210px]'}
            src={'/img/1734620052680.jpg'}
            alt={'支付宝赞助'}
          />
        </div>
      </div>
      <Suspense fallback={<SuspenseSpin />}>
        <div>
          <h2 className={'text-theme-brand text-[24px]'}>收藏排名</h2>
          <RankingFilter labelType={''} sortType={'reward_ranking'} />
        </div>
        <div className={'mt-4'}>
          <h2 className={'text-theme-brand text-[24px]'}>月票排名</h2>
          <RankingFilter labelType={''} sortType={'monthly_pass'} />
        </div>
        <div className={'mt-4'}>
          <h2 className={'text-theme-brand text-[24px]'}>作品评论排名</h2>
          <RankingFilter labelType={''} sortType={'comment_num'} />
        </div>
      </Suspense>
    </div>
  );
}
