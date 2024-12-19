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
        'text-black': 'rgb(38, 38, 38)',
        'bg-gray-doc2x': '#e3e4e588',
        'body-bg': '#e3e4e588',
        'black-translucent': 'rgba(0, 0, 0, 0.7)',
        'bg-gray': 'rgba(79,89,102,.039)',
        border: "hsl(var(--border))",
        input: "hsl(var(--input))",
        ring: "hsl(var(--ring))",
        background: "hsl(var(--background))",
        foreground: "hsl(var(--foreground))",
        destructive: {
          DEFAULT: "hsl(var(--destructive))",
          foreground: "hsl(var(--destructive-foreground))",
        },
        muted: {
          DEFAULT: "hsl(var(--muted))",
          foreground: "hsl(var(--muted-foreground))",
        },
        accent: {
          DEFAULT: "hsl(var(--accent))",
          foreground: "hsl(var(--accent-foreground))",
        },
        popover: {
          DEFAULT: "hsl(var(--popover))",
          foreground: "hsl(var(--popover-foreground))",
        },
        card: {
          DEFAULT: "hsl(var(--card))",
          foreground: "hsl(var(--card-foreground))",
        },
        primary: '#080808',
        secondary: '#8C8C8C',
        third: '#9E9EA7',
        customCyan: '#01DEEA',
        theme: {
          "--theme-bg-gray-secondary": "#fafafa",
          brand: '#7748F9',
          // negativeTheme: '#CFBEFF',
          negativeTheme: 'rgba(104,65,234,.078)',

          footbar: '#1E123F',
        },
        'default-gray': '#9E9EA7',
        'highlight-gray': '#e3e1e1',
        gray: '#F1F1F1',
        grayLine: '#e3e1e1',
        dark: '#1E123F',
        keyColor: '#1677C8',
        valueColor: '#FE7B2F',
        borderPrimary: '#D8D8D8',
        surfaceGray: '#F7F7F7',
        menuBg: 'rgba(0, 0, 0, 0.03)',
        surfaceHover: '#F2F0FE',
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
