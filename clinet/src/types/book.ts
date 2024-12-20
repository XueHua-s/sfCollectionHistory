export interface BookInfo {
  id: string;
  b_id: number;
  book_name: string;
  cover_url: string;
  book_type: string;
  tap_num: number;
  tags: string;
  like_num: number;
  collect_num: number;
  comment_num: number;
  comment_long_num: number;
  monthly_pass: number;
  monthly_ticket_ranking: number;
  reward_ranking: number;
  created_time: string;
  last_update_time: string;
  label_type: string;
}
export interface BookRank extends BookInfo {
  rank: number;
}
