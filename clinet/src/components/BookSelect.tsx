import React, { useState, useEffect, useCallback } from 'react';
import { Select, message } from 'antd';
import { getRankRecord } from '@/client_api/rank';
import { BookRank } from '@/types/book';
import { debounce } from 'lodash';

const { Option } = Select;

interface BookSelectProps {
  className: string;
  value: string;
  onChange: (value: string) => void;
}

const BookSelect: React.FC<BookSelectProps> = ({
  value,
  className,
  onChange,
}) => {
  const [options, setOptions] = useState<BookRank[]>([] as BookRank[]);
  const [loading, setLoading] = useState(false);

  const fetchBooks = async (bookName = '') => {
    setLoading(true);
    try {
      const response = await getRankRecord({
        current: 1,
        size: 10,
        book_name: bookName,
        sort_type: 'collect_num',
        label_type: '',
      });

      if (response.code === 'success' && response?.data?.list) {
        setOptions(response?.data?.list);
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

  useEffect(() => {
    fetchBooks();
  }, []);

  return (
    <Select
      className={className}
      showSearch
      placeholder="请选择作品"
      value={value}
      onChange={onChange}
      onSearch={debouncedFetchBooks}
      loading={loading}
      filterOption={false}>
      <Option key={'0000'} value={''}>
        不对比
      </Option>
      {options.map((book: BookRank) => (
        <Option key={book.b_id} value={book.b_id}>
          {book.book_name}
        </Option>
      ))}
    </Select>
  );
};

export default BookSelect;
