const cheerio = require('cheerio')
const request = require('./request')
const axios = require("axios");
const analyzeBooks = async (pageNum) => {
  const data = await request.get(`/List/default.aspx?tid=-1&&ud=30&PageIndex=${pageNum}`);
  if (data.status === 200) {
    const $ = cheerio.load(data.data);
    const booksLink = $(".bsubcon .Comic_Pic_List>.Conjunction>a");
    if (booksLink.length === 0) {
      return -1;
    }
    const promise_all = []
    booksLink.each((index, element) => {
      promise_all.push(new Promise((resolve, reject) => {
        const href = $(element).attr('href');
        // 使用正则表达式匹配数字
        const match = href.match(/\/Novel\/(\d+)\//);
        const fetch = async () => {
          try {
            const number = match[1]; // 提取的数字
            const data2 = await axios.post('https://api.sfacg.cloud:18080/api/books/add/' + number)
            console.log(data2.data, '添加结果')
            resolve()
          } catch (err) {
            // 发生错误重新执行
            console.log('错误', err.status, err.data)
            await new Promise((resolve) => {
              setTimeout(() => {
                resolve()
              }, 2000)
            })
            await fetch()
          }
        }
        if (match) {
          fetch()
        } else {
          console.log('No match found');
        }
      }))
    })
    await Promise.all(promise_all)
  }
}
const getStore = async () => {
  try {
    let pageNum = 1
    while (true) {
      const res = await analyzeBooks(pageNum);
      // 没查询到数据,爬到底了,直接中断
      if (res === -1) {
        console.log(`获取完成, 共${pageNum}页`)
        break;
      }
      pageNum++
    }
  } catch (err) {
    // console.log('爬取完成')
    console.log(err)
    throw new Error('爬取首页错误')
  }
}
getStore()
