import React from 'react';
import zhCN from 'antd/locale/zh_CN';
import { ConfigProvider } from 'antd';
interface SFConfigProviderProps {
  children: React.ReactNode;
}
const SFConfigProvider: React.FC<SFConfigProviderProps> = ({ children }) => {
  return (
    <ConfigProvider locale={zhCN}>
      <ConfigProvider
        theme={{
          token: {
            // Seed Token，影响范围大
            colorPrimary: '#e29464',
            borderRadius: 2,
            // 派生变量，影响范围小
            colorBgContainer: '#faf9f9',
          },
          components: {},
        }}>
        {children}
      </ConfigProvider>
    </ConfigProvider>
  );
};
export default SFConfigProvider;
