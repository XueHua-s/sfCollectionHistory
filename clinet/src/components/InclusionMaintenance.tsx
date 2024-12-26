'use client';
import { useState } from 'react';
import { Button, message, Spin } from 'antd';
import { maintenanceBook } from '@/client_api/search';
import BookSelect from '@/components/BookSelect';

const InclusionMaintenance = () => {
  const [loading, setLoading] = useState(false);
  const [submitBookId, setSubmitBookId] = useState('');
  const [messageApi, contextHolder] = message.useMessage();
  const submit = async () => {
    if (!submitBookId) {
      messageApi.info('请搜索并选择作品后，提交维护。');
      return;
    }
    try {
      setLoading(true);
      const data = await maintenanceBook(submitBookId);
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
        <div className="label min-w-[80px] mr-2">恢复维护:</div>
        <BookSelect
          showNone={false}
          className={'flex-1 max-w-[430px] overflow-hidden'}
          onChange={setSubmitBookId}
          isStartLoading={false}
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
export default InclusionMaintenance;
