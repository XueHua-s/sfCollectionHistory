import dayjs from 'dayjs';
import { BookInfo } from '@/types/book';
// 作品是否太监
export const bookIsEunuch = (last_update_time: string, finish: number) => {
  // 将字符串转换为dayjs对象
  const date = dayjs(last_update_time);

  // 获取当前时间的dayjs对象
  const now = dayjs();
  return now.diff(date, 'day') > 30 && finish === 0;
};
// 对书本对比数据分析进行双指针自动规划补齐
export function alignBookData(
  arr1: BookInfo[],
  arr2: BookInfo[],
): { alignedArr1: BookInfo[]; alignedArr2: BookInfo[] } {
  const toDate = (dateStr: string): Date => new Date(dateStr);

  // Sort both arrays by created_time
  arr1.sort(
    (a, b) =>
      toDate(a.created_time).getTime() - toDate(b.created_time).getTime(),
  );
  arr2.sort(
    (a, b) =>
      toDate(a.created_time).getTime() - toDate(b.created_time).getTime(),
  );

  const alignedArr1: BookInfo[] = [];
  const alignedArr2: BookInfo[] = [];

  let index1 = 0;
  let index2 = 0;
  let lastKnownData1: BookInfo | null = null;
  let lastKnownData2: BookInfo | null = null;

  // Iterate over both arrays to align them
  while (index1 < arr1.length || index2 < arr2.length) {
    const date1 =
      index1 < arr1.length ? toDate(arr1[index1].created_time) : null;
    const date2 =
      index2 < arr2.length ? toDate(arr2[index2].created_time) : null;

    if (date1 && (!date2 || date1 < date2)) {
      lastKnownData1 = arr1[index1];
      alignedArr1.push(lastKnownData1);
      alignedArr2.push(lastKnownData2 || lastKnownData1);
      index1++;
    } else if (date2 && (!date1 || date2 < date1)) {
      lastKnownData2 = arr2[index2];
      alignedArr2.push(lastKnownData2);
      alignedArr1.push(lastKnownData1 || lastKnownData2);
      index2++;
    } else if (date1 && date2 && date1.getTime() === date2.getTime()) {
      // If dates are equal, align both arrays
      lastKnownData1 = arr1[index1];
      lastKnownData2 = arr2[index2];
      alignedArr1.push(lastKnownData1);
      alignedArr2.push(lastKnownData2);
      index1++;
      index2++;
    }
  }

  return { alignedArr1, alignedArr2 };
}
