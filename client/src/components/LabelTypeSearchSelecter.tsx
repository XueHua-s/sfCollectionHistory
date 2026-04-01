'use client';
import React, { useEffect, useState } from 'react';
import { Select } from 'antd';
import { getLabelsRecord } from '@/client_api/rank';

interface LabelTypeSearchSelecterProps {
  value: string;
  className?: string;
  placeholder: string;
  onChange: (value: string) => void;
}
const LabelTypeSearchSelecter: React.FC<LabelTypeSearchSelecterProps> = ({
  value,
  className,
  onChange,
  placeholder,
}) => {
  const [selectList, setSelectList] = useState<
    {
      label: string;
      value: string;
    }[]
  >([]);
  const loadLabels = async () => {
    const data = await getLabelsRecord('');
    if (data.code === 'success' && data?.data) {
      setSelectList([
        {
          label: '全部',
          value: '',
        },
        ...data.data.map((text) => ({
          label: text,
          value: text,
        })),
      ]);
    }
  };
  useEffect(() => {
    loadLabels();
  }, []);
  return (
    <Select
      className={className}
      showSearch
      placeholder={placeholder}
      onChange={(e) => onChange(e)}
      value={value}
      options={selectList}
    />
  );
};
export default LabelTypeSearchSelecter;
