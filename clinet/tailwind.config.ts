import type { Config } from "tailwindcss";
import Animate from 'tailwindcss-animate'
export default {
  darkMode: ["class"],
  content: ["app/**/*.{ts,tsx}", "components/**/*.{ts,tsx}", './**/*.tsx', './index.html'],
  theme: {
    container: {
      center: true,
      padding: "2rem",
    },
    extend: {
      screens: {
        'custom-pc': '760px',
        'custom-mobile': {'max': '759px'},
        'max-lg': {
          max: '1566px'
        },
        'max-mobile-nav': {
          max: '1156px'
        },
        '2xl': '1400px', // 已定义
        '3xl': '1600px', // 新增
        '4xl': '1920px', // 新增
      },
      zIndex: {
        'max': '9999',
        'middle': '3000',
        'antd-tips-flow': '2000'
      },
      colors: {
        primary: '#e29464',
        secondary: '#c2734d',
        third: '#f6c3a1',
        customCyan: '#f7d3bf',
        theme: {
          "--theme-bg-gray-secondary": "#fafafa",
          brand: '#e29464',
          negativeTheme: 'rgba(226, 148, 100, 0.078)',
          footbar: '#a6603b',
        },
        'default-gray': '#d1b5a3',
        'highlight-gray': '#e6bfa5',
        gray: '#F1F1F1',
        grayLine: '#c9a890',
        dark: '#1E123F',
        keyColor: '#c2734d',
        valueColor: '#f6c3a1',
        borderPrimary: '#804c2f',
        surfaceGray: '#F7F7F7',
        menuBg: '#f7d3bf',
        surfaceHover: '#f6c3a1',
      },
      borderRadius: {
        lg: `var(--radius)`,
        md: `calc(var(--radius) - 2px)`,
        sm: "calc(var(--radius) - 4px)",
        '4xl': '2rem',
      },
      // fontFamily: {
      //   NoteSans: ['NoteSans', "sans-serif"],
      //   Inter: ['Inter'],
      //   Variable: ['Variable'],
      // },
      fontWeight: {
        titleBold: '900',
      },
      fontSize: {
        xs: ['0.75rem', '1rem'],
        s: ['0.875rem', '1.125rem'],
        m: ['1rem', '1.25rem'],
        l: ['1.25rem', '1.375rem'],
        xl: ['1.5rem', '1.625rem'],
        'label-xs': ['0.75rem', { lineHeight: '0.875rem', fontWeight: 400 }],
        'label-s': ['0.875rem', { lineHeight: '1rem', fontWeight: 400 }],
        'label-m': ['1.125rem', { lineHeight: '1.125rem', fontWeight: 500 }],
        link: ['1rem', { lineHeight: '1.375rem', fontWeight: 400 }],
        markup: ['1rem', { lineHeight: '1.375rem', fontWeight: 500 }],
        paragraph: ['1rem', { lineHeight: '1.25rem', fontWeight: 400 }],
        desc: ['1.125rem', { lineHeight: '2rem' }],
      },
      keyframes: {
        floatIn: {
          from: {
            transform: 'translateY(16px)',
            opacity: '0',
          },
          to: {
            transform: 'translateY(0px)',
            opacity: '1',
          },
        },
        fadeIn: {
          from: {
            opacity: '0',
          },
          to: {
            opacity: '1',
          },
        },
        messageFloatIn: {
          from: {
            transform: 'translateY(16px) translateX(-50%)',
            opacity: '0',
          },
          to: {
            transform: 'translateY(0px) translateX(-50%)',
            opacity: '1',
          },
        },
        spin: {
          from: {
            transform: 'rotate(0deg)',
          },
          to: {
            transform: 'rotate(360deg)',
          },
        },
        "accordion-down": {
          from: { height: "0" },
          to: { height: "var(--radix-accordion-content-height)" },
        },
        "accordion-up": {
          from: { height: "var(--radix-accordion-content-height)" },
          to: { height: "0" },
        },
      },
      animation: {
        'float-in': 'floatIn 0.3s ease-out forwards',
        'message-float-in': 'messageFloatIn 0.3s ease-out forwards',
        spin: 'spin 1s linear infinite',
        'fade-in': 'fadeIn 0.3s forwards',
        'fade-in-after-transition': 'fadeIn 0.3s forwards 0.15s',
        "accordion-down": "accordion-down 0.2s ease-out",
        "accordion-up": "accordion-up 0.2s ease-out",
      },
      lineHeight: {
        none: '1',
        tighter: '1.125',
        tight: '1.25',
        snug: '1.375',
        normal: '1.5',
        relaxed: '1.625',
        loose: '2',
        3: '.75rem',
        4: '1rem',
        5: '1.2rem',
        6: '1.5rem',
        7: '1.75rem',
        8: '2rem',
        9: '2.25rem',
        10: '2.5rem',
      },
      width: {
        90: '22.5rem',
      },
      minWidth: {
        10: '2.5rem',
        48: '12rem',
      },
      borderWidth: {
        1: '1px',
      },
      boxShadow: {
        box: '0px 4px 18px 0px rgba(25, 27, 29, 0.18)',
      },
    },
  },
  plugins: [Animate],
} satisfies Config;
