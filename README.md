# SFACG Data Analysis Project

本项目是用于爬取并分析 [菠萝包轻小说](https://book.sfacg.com/) 网站数据，并为作者提供实时的数据上涨情况以及与同行作品的对比分析。

## 目录结构

- `client` 文件夹：基于 Next.js 技术 SSG 构建的网页客户端，使用了 TailwindCSS 和 Ant Design 组件库。
- `script` 文件夹：基于 Cheerio 的爬虫，用于爬取菠萝包轻小说小说信息。
- `server` 文件夹：使用 Actix-Web 和 Chrono 技术栈的服务端，采用三层架构设计（服务层、模块层、控制层），并使用 Cron 进行定时任务管理，每天对作品数据进行维护。

## 技术栈

### 客户端

- **Next.js**: 用于构建静态生成的网页客户端。
- **TailwindCSS**: 用于快速开发响应式界面。
- **Ant Design**: 用于提供丰富的 UI 组件。

### 爬虫

- **Cheerio**: 用于解析和操作 HTML，模拟 jQuery 的功能。

### 服务端

- **Actix-Web**: 用于构建高性能的 Web 服务。
- **Chrono**: 用于处理日期和时间。
- **Cron**: 用于管理定时任务，每天定时爬取并维护作品数据。
- **MySQL**: 作为数据库，用于存储爬取的数据。
- **SQLx**: 用于与 MySQL 数据库进行交互的查询工具。

## 安装与使用

### 克隆仓库

```bash
git clone https://github.com/yourusername/sfacg-data-analysis.git
cd sfacg-data-analysis