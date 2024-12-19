"use client";
import { useSearchParams } from "next/navigation";
import { useEffect } from "react";

const RankingFilter = ()=> {
  const searchParams = useSearchParams();
  useEffect(() => {
    const search = searchParams.get('search');
    console.log(search);
    
  }, [])
  return <div>排行榜</div>
}
export default RankingFilter