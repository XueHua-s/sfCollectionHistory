import RankingFilter from '@/components/RankingFilter';
import SuspenseSpin from '@/components/SuspenseSpin';
import { Suspense } from 'react';
import { Tooltip } from 'antd';
import InclusionEntrance from '@/components/InclusionEntrance';
import InclusionMaintenance from '@/components/InclusionMaintenance';
export default function Home() {
  return (
    <div className={'flex flex-col'}>
      <div className="support">
        <h1 className={'text-theme-brand text-[24px]'}>我很可爱请给我钱</h1>
        <p className={'text-grayLine'}>
          开发不易, 服务器运维需要成本, 有能力请赞助。
        </p>
        <div className="qr-code flex">
          <div className={'p-2'}>
            <Tooltip title={'微信赞助'}>
              <img
                className={'w-[120px]'}
                src={'/img/IMG_20241220_234004.png'}
                alt={'微信赞助'}
              />
            </Tooltip>
          </div>
          <div className={'mt-[1px]'}>
            <Tooltip title={'支付宝赞助'}>
              <img
                className={'w-[135px]'}
                src={'/img/IMG_20241220_234038.jpg'}
                alt={'支付宝赞助'}
              />
            </Tooltip>
          </div>
          <div className={'flex-1'}>
          </div>
        </div>
      </div>
      <div className="group-link">
        <h1 className={'text-theme-brand text-[24px]'}>官方群</h1>
        <div>
          <a className={'text-grayLine'}
             href="http://qm.qq.com/cgi-bin/qm/qr?_wv=1027&k=_NZERm3fGETb7y1GM4gPsLxLPLrDDZe5&authKey=TvhtgSEdswRDdeMspCpD6O8ubPecckM5Sk0znzlViXzi%2FUg7IgXPIjTKq93uuy6X&noverify=0&group_code=996997193"
             target={'_blank'}>加入QQ群</a>
        </div>
        <p className={'text-grayLine mt-4'}>
          如果有问题和建议反馈, 请加入群聊交流。暂无其它反馈渠道。
        </p>
      </div>
      <div className="submitEntry mb-4">
        {/*提交入口*/}
        <h1 className={'text-theme-brand text-[24px]'}>提交入口</h1>
        <div className={'submit-in'}>
          <Suspense fallback={<SuspenseSpin />}>
            <InclusionEntrance />
          </Suspense>
          <Suspense fallback={<SuspenseSpin />}>
            <div className={'mt-4'}>
              <InclusionMaintenance />
            </div>
          </Suspense>
        </div>
      </div>
      <Suspense fallback={<SuspenseSpin />}>
        <div>
          <h2 className={'text-theme-brand text-[24px]'}>打赏排名</h2>
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
