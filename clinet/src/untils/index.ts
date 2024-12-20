import dayjs from 'dayjs';
// 作品是否太监
export const bookIsEunuch = (last_update_time: string, finish: number) => {
  // 将字符串转换为dayjs对象
  const date = dayjs(last_update_time);

  // 获取当前时间的dayjs对象
  const now = dayjs();
  return now.diff(date, 'day') > 30 && finish === 0;
};
