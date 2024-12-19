import LayoutHeader from '@/components/LayoutHeader';
import SuspenseSpin from '@/components/SuspenseSpin';
import { Suspense } from 'react';

interface DefaultLayoutProps {
  children: React.ReactNode;
}
const DefaultLayout: React.FC<DefaultLayoutProps> = ({ children }) => {
  return (
    <>
      <header>
        <Suspense fallback={<SuspenseSpin />}>
          <LayoutHeader />
        </Suspense>
      </header>
      <main className={'custom-pc:w-[80%] custom-mobile:w-[95%] m-auto'}>
        {children}
      </main>
      <footer></footer>
    </>
  );
};
export default DefaultLayout;
