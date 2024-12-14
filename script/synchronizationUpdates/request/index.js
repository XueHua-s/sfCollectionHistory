const axios = require('axios')
const request = axios.create()
const { baseUrl } = require('../config')
axios.create({
  timeout: 3600000 // 请求超时时间
})
request.interceptors.request.use((config) => {
  config.baseURL = baseUrl
  return config
}, error => {
  return error
})
request.interceptors.response.use(data => {
  return data
}, (error) => {
  return error
})
module.exports = request
