import './globals.css';
import { AntdRegistry } from '@ant-design/nextjs-registry';
import DefaultLayout from '@/layout/DefaultLayout';
import SFConfigProvider from '@/components/SFConfigProvider';

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta
          name="viewport"
          content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no"
        />
        <meta
          name="keywords"
          content="小说数据分析, 菠萝包轻小说, sf轻小说, 作者小助手"
        />
        <meta
          name="description"
          content="菠萝包轻小说, 作品分析助手, SF轻小说数据分析网, "
        />
        <title>SF轻小说数据网</title>
      </head>
      <body
        className={`h-[100vh] w-full antialiased flex flex-col thinner-scrollbar`}>
        <AntdRegistry>
          <SFConfigProvider>
            <DefaultLayout>{children}</DefaultLayout>
          </SFConfigProvider>
        </AntdRegistry>
      </body>
    </html>
  );
}
