import React, { useState, useEffect, useCallback } from 'react';
import { Select, message } from 'antd';
import { MasterBook, queryMasterBook } from '@/client_api/search';
import { debounce } from 'lodash';
import { SearchOutlined } from '@ant-design/icons';

const { Option } = Select;

interface BookSelectProps {
  className: string;
  value: string;
  onChange: (value: string) => void;
}

const MasterBookSelect: React.FC<BookSelectProps> = ({
  value,
  className,
  onChange,
}) => {
  const [options, setOptions] = useState<MasterBook[]>([] as MasterBook[]);
  const [loading, setLoading] = useState(false);

  const fetchBooks = async (bookName = '') => {
    setLoading(true);
    try {
      const response = await queryMasterBook(bookName);

      if (response.code === 'success' && response?.data) {
        setOptions(response?.data);
      } else {
        message.error('Failed to fetch data');
      }
    } catch (error: any) {
      console.log(error);
      message.error('An error occurred');
    } finally {
      setLoading(false);
    }
  };
  const debouncedFetchBooks = useCallback(debounce(fetchBooks, 300), []);
  return (
    <Select
      suffixIcon={<SearchOutlined />}
      className={className}
      showSearch
      placeholder="输入关键词, 搜索作品"
      value={value}
      onChange={onChange}
      onSearch={debouncedFetchBooks}
      loading={loading}
      filterOption={false}>
      {options.map((book: MasterBook) => (
        <Option key={book.b_id} value={book.b_id}>
          <div
            className={'master-book-container'}
            dangerouslySetInnerHTML={{
              __html: book.title,
            }}></div>
        </Option>
      ))}
    </Select>
  );
};

export default MasterBookSelect;
