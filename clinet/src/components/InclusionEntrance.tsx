'use client';
import MasterBookSelect from '@/components/MasterBookSelect';
import { useState } from 'react';
import {Button, message} from "antd";
import {entranceBook} from "@/client_api/search";

const InclusionEntrance = () => {
  const [submitBookId, setSubmitBookId] = useState('');
  const [messageApi, contextHolder] = message.useMessage();
  const submit = async () => {
    try {
      const data = await entranceBook(submitBookId)
      if (data.code === 'success') {
        if (data.message) {
          messageApi.info(data?.message as unknown as string)
          return
        }
        messageApi.success('收录完成')
      }
    } catch (err: any) {
      console.log(err)
      messageApi.error('提交错误')
    }
  }
  return (
    <div className={'flex items-center'}>
      {contextHolder}
      <div className="label mr-2">收录入口:</div>
      <MasterBookSelect
        className={'flex-1 max-w-[430px]'}
        onChange={setSubmitBookId}
        value={submitBookId}
      />
      <div className={'ml-4'}>
        <Button onClick={submit} type={'primary'}>提交</Button>
      </div>
    </div>
  );
};
export default InclusionEntrance;
