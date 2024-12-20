'use client';
import React, { useEffect, useRef } from 'react';
import * as echarts from 'echarts';

interface DataLineChartsProps {
  xData: string[];
  seriesData: any[];
  title: string;
}

const DataLineCharts: React.FC<DataLineChartsProps> = ({
  xData,
  seriesData,
  title,
}) => {
  const chartRef = useRef<HTMLDivElement | null>(null);

  const renderChart = () => {
    if (chartRef.current) {
      const chartInstance = echarts.init(chartRef.current);
      const options = {
        title: {
          text: title || 'Stacked Line',
          top: '0%', // 增加标题的顶部留白
        },
        tooltip: {
          trigger: 'axis',
        },
        legend: {
          data: seriesData.map((item) => item.name),
          top: '6%', // 图例的顶部留白
          left: 'center', // 图例居中
        },
        grid: {
          top: '15%', // 调整顶部留白
          left: '3%',
          right: '4%',
          bottom: '3%',
          containLabel: true,
        },
        toolbox: {
          feature: {
            saveAsImage: {},
          },
        },
        xAxis: {
          type: 'category',
          boundaryGap: false,
          data: xData,
        },
        yAxis: {
          type: 'value',
        },
        series: seriesData,
      };

      chartInstance.setOption(options);

      // Resize chart on window resize
      window.addEventListener('resize', () => {
        chartInstance.resize();
      });

      return () => {
        window.removeEventListener('resize', () => {
          chartInstance.resize();
        });
        chartInstance.dispose();
      };
    }
  };
  useEffect(() => {
    renderChart();
  }, [xData, seriesData, title]);

  return <div ref={chartRef} className={'w-full h-[600px]'} />;
};

export default DataLineCharts;
