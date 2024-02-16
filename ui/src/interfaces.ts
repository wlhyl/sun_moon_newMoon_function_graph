// request
export interface DateRequest {
  // 年，最小值1900
  year: number;
  // 月
  month: number;
  // 日
  day: number;
  // 时
  hour: number;
  // 分
  minute: number;
  // 秒
  second: number;
}

export interface DateRangeRequest {
  start: DateRequest;
  end: DateRequest;
}

// Responser
export interface LongResponser {
  date: HoroDateTime;
  long: number;
}

export interface HoroDateTime {
  // 年
  year: number;
  // 月
  month: number;
  // 日
  day: number;
  // 时
  hour: number;
  // 分
  minute: number;
  // 秒
  second: number;

  // 时区
  // 东为正，西为负
  tz: number;
}
