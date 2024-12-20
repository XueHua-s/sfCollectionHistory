'use client';
import {Button, Input, Pagination, Select, Table} from 'antd';
import {useEffect, useMemo, useRef, useState} from 'react';
import { sortTypes } from '@/types/enums';
import LabelTypeSearchSelecter from '@/components/LabelTypeSearchSelecter';
import { getRankRecord } from '@/client_api/rank';
import {BookRank} from "@/types/book";
const Ranks = () => {
  const [ loading, setLoading ] = useState<boolean>(false);
  const tableRef = useRef<HTMLDivElement>(null);
  // 创建一个状态来存储高度
  const [tableHeight, setTableHeight] = useState(600);
  const [bookName, setBookName] = useState('');
  const [sortType, setSortType] = useState('collect_num');
  const [labelType, setLabelType] = useState('');
  const [tableData, setTableData] = useState<BookRank[]>([]);
  const [current, setCurrent] = useState(1);
  const [size, setSize] = useState(20);
  const [ total, setTotal ] = useState(0);
  // 使用useEffect来监听高度变化
  useEffect(() => {
    // 定义一个函数来更新高度
    const updateHeight = () => {
      if (tableRef.current) {
        setTableHeight(tableRef.current.clientHeight);
      }
    };

    // 监听窗口大小变化，以便在窗口大小改变时更新高度
    window.addEventListener('resize', updateHeight);

    // 初始调用更新高度
    updateHeight();

    // 清理函数，在组件卸载时移除事件监听器
    return () => {
      window.removeEventListener('resize', updateHeight);
    };
  }, []);
  const columns = useMemo(() => {
    return [
      {
        width: 80,
        title: '排名',
        dataIndex: 'rank',
        fixed: true,
        key: 'rank',
      },
      {
        width: 150,
        title: '书名',
        dataIndex: 'book_name',
        key: 'book_name',
      },
      {
        width: 150,
        title: '征文类型',
        dataIndex: 'label_type',
        key: 'label_type',
      },
      {
        width: 150,
        title: '封面',
        dataIndex: 'cover_url',
        key: 'cover_url',
        render: (coverUrl: string) => <img src={coverUrl} alt="cover" style={{ width: '120px', height: 'auto' }} />,
      },
      {
        width: 150,
        title: '类型',
        dataIndex: 'book_type',
        key: 'book_type',
      },
      {
        width: 150,
        title: '点击数',
        dataIndex: 'tap_num',
        key: 'tap_num',
      },
      {
        width: 150,
        title: '标签',
        dataIndex: 'tags',
        key: 'tags',
      },
      {
        width: 150,
        title: '点赞数',
        dataIndex: 'like_num',
        key: 'like_num',
      },
      {
        width: 150,
        title: '收藏数',
        dataIndex: 'collect_num',
        key: 'collect_num',
      },
      {
        width: 150,
        title: '评论数',
        dataIndex: 'comment_num',
        key: 'comment_num',
      },
      {
        width: 150,
        title: '长评数',
        dataIndex: 'comment_long_num',
        key: 'comment_long_num',
      },
      {
        width: 150,
        title: '月票',
        dataIndex: 'monthly_pass',
        key: 'monthly_pass',
      },
      {
        width: 150,
        title: '主站月票排名',
        dataIndex: 'monthly_ticket_ranking',
        key: 'monthly_ticket_ranking',
      },
      {
        width: 150,
        title: '主站打赏排名',
        dataIndex: 'reward_ranking',
        key: 'reward_ranking',
      },
      {
        width: 150,
        title: '创建时间',
        dataIndex: 'created_time',
        key: 'created_time',
      },
      {
        width: 150,
        title: '最后更新时间',
        dataIndex: 'last_update_time',
        key: 'last_update_time',
      },
    ];
  }, [])
  const loadTableData = async (newPage?: number, newSize?: number) => {
    setLoading(true)
    try {
      const data = await getRankRecord({
        current: newPage || current,
        size: newSize || current,
        sort_type: sortType,
        label_type: labelType,
        book_name: bookName,
      });
      if (data.code === 'success' && data?.data) {
        setTableData(data.data?.list)
        setTotal(data.data.total_num)
        if (newPage && newSize) {
          setCurrent(newPage)
          setSize(newSize)
        }
        setLoading(false)
      }
      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
      // @ts-expect-error
    } catch (err: never) {
      console.log(err);
      setLoading(false)
    }
  };
  useEffect(() => {
    loadTableData(1, 10)
  }, []);
  return (
    <div className={'p-2 w-full h-full flex flex-col books-rank'}>
      <div className="query">
        <div className="flex gap-4">
          <div className="item flex items-center">
            <div className="label w-[80px]">书名:</div>
            <div className="content">
              <Input
                value={bookName}
                onChange={(e) => setBookName(e.target.value)}
              />
            </div>
          </div>
          <div className="item flex items-center">
            <div className="label w-[80px]">排序方式:</div>
            <div className="content">
              <Select
                className={'w-[90px]'}
                options={sortTypes}
                onChange={(value) => setSortType(value)}
                value={sortType}
              />
            </div>
          </div>
          <div className="item flex items-center">
            <div className="label w-[80px]">征文类型:</div>
            <div className="content">
              <LabelTypeSearchSelecter
                placeholder={'请选择征文类型'}
                className={'w-[210px]'}
                onChange={(value) => setLabelType(value)}
                value={labelType}
              />
            </div>
          </div>
          <div className={'item flex items-center'}>
            <Button onClick={() => {
              loadTableData(1, size)
            }}>查询</Button>
          </div>
        </div>
      </div>
      <div ref={tableRef} className="table h-[80vh] relative mt-4 w-full overflow-hidden">
        <div className={'absolute w-full h-full'}>
          <Table
            // key={tableHeight}
            loading={loading}
            tableLayout={'fixed'}
            pagination={false}
            scroll={{ x: columns.reduce((count, item) => {
                count += item?.width as number;
                return count;
              }, 0), y: tableHeight - 100 }}// 确保x值足够宽，y值足够高
            columns={columns}
            className={'w-full'}
            bordered
            // virtual
            dataSource={tableData}
            rowKey="id" // 确保每行数据有一个唯一的key
          />
        </div>
      </div>
      <div className="page flex justify-end">
        <Pagination onChange={(page, size) => {
          loadTableData(page, size)
        }} defaultCurrent={current} total={total} pageSize={size} />
      </div>
    </div>
  );
};
export default Ranks;
