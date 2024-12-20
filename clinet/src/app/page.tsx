import RankingFilter from '@/components/RankingFilter';
import SuspenseSpin from '@/components/SuspenseSpin';
import { Suspense } from 'react';
import {Tooltip} from "antd";
export default function Home() {
  return (
    <div className={'flex flex-col'}>
      <div className="support">
        <h1 className={'text-theme-brand text-[24px]'}>我很可爱请给我钱</h1>
        <p className={'text-grayLine'}>
          开发不易, 服务器运维需要成本, 有能力请赞助。
        </p>
        <div className="qr-code flex">
          <div className={'p-2 h-[210px]'}>
            <Tooltip title={'微信赞助'}>
              <img
                className={'h-full'}
                src={'/img/IMG_20241220_234004.png'}
                alt={'微信赞助'}
              />
            </Tooltip>
          </div>
          <Tooltip title={'支付宝赞助'}>
            <img
              className={'h-[210px]'}
              src={'/img/IMG_20241220_234038.jpg'}
              alt={'支付宝赞助'}
            />
          </Tooltip>

        </div>
      </div>
      <Suspense fallback={<SuspenseSpin/>}>
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
