'use client';
import MasterBookSelect from '@/components/MasterBookSelect';
import { useState } from 'react';
import { Button, message, Spin } from 'antd';
import { entranceBook } from '@/client_api/search';

const InclusionEntrance = () => {
  const [loading, setLoading] = useState(false);
  const [submitBookId, setSubmitBookId] = useState('');
  const [messageApi, contextHolder] = message.useMessage();
  const submit = async () => {
    if (!submitBookId) {
      messageApi.info('请搜索并选择作品后，提交收录。');
      return;
    }
    try {
      setLoading(true);
      const data = await entranceBook(submitBookId);
      if (data.code === 'success') {
        if (data.message) {
          messageApi.info(data?.message as unknown as string);
          return;
        }
        messageApi.success('收录完成');
      }
    } catch (err: any) {
      console.log(err);
      messageApi.error('提交错误');
    } finally {
      setLoading(false);
    }
  };
  return (
    <Spin spinning={loading}>
      <div className={'flex items-center'}>
        {contextHolder}
        <div className="label min-w-[80px] mr-2">收录入口:</div>
        <MasterBookSelect
          className={'flex-1 max-w-[430px] overflow-hidden'}
          onChange={setSubmitBookId}
          value={submitBookId}
        />
        <div className={'ml-4'}>
          <Button onClick={submit} type={'primary'}>
            提交
          </Button>
        </div>
      </div>
    </Spin>
  );
};
export default InclusionEntrance;
