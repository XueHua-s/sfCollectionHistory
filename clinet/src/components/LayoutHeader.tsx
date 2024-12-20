'use client';
import { useEffect, useState } from 'react';
import { FieldNumberOutlined, HomeOutlined } from '@ant-design/icons';
import { Menu } from 'antd';
import { usePathname, useRouter } from 'next/navigation';
const HeaderMenus = [
  {
    key: '/',
    label: '首页',
    icon: <HomeOutlined />,
  },
  {
    key: '/ranks',
    label: '排行榜',
    icon: <FieldNumberOutlined />,
  },
];
const LayoutHeader = () => {
  const pathname = usePathname();
  const router = useRouter();
  // 定义一个状态变量来存储页面滚动位置
  const [isFixed, setIsFixed] = useState(false);

  // 使用useEffect来添加和清理滚动事件监听器
  useEffect(() => {
    const handleScroll = () => {
      // 检查滚动位置是否超过页面高度的20%
      if (window.scrollY > window.innerHeight * 0.2) {
        setIsFixed(true);
      } else {
        setIsFixed(false);
      }
    };

    // 添加滚动事件监听器
    window.addEventListener('scroll', handleScroll);

    // 清理函数，在组件卸载时移除事件监听器
    return () => {
      window.removeEventListener('scroll', handleScroll);
    };
  }, []);

  return (
    <div
      className={`w-full transition-all bg-[#faf9f9] ${isFixed ? 'fixed shadow-xl top-0' : ''}`}>
      <div className="header-container text-white custom-pc:w-[80%] custom-pc:m-auto custom-mobile:w-[95%]">
        <Menu
          onClick={(e) => router.push(e.key)}
          className="bg-white"
          selectedKeys={[pathname]}
          mode="horizontal"
          items={HeaderMenus}
        />
      </div>
    </div>
  );
};
export default LayoutHeader;
