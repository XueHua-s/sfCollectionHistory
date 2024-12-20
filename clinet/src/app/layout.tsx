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
      <body
        className={`h-[100vh] w-full antialiased flex flex-col thinner-scrollbar`}>
        <SFConfigProvider>
          <AntdRegistry>
            <DefaultLayout>{children}</DefaultLayout>
          </AntdRegistry>
        </SFConfigProvider>
      </body>
    </html>
  );
}
