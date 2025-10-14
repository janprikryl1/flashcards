module.exports = {
    mode: "jit",
    content: {
        files: ["src/**/*.rs", "index.html"],
    },
    darkMode: "class",
    variants: {
        extend: {},
    },
    theme: {
        extend: {
            colors: {
                'accent-dark': '#E64A19',
                'accent': '#FF5722',
                'background-dark': '#121212',
                'background': '#F7F7F7',
                'danger-dark': '#CC0000',
                'danger': '#FF4C4C',
                'foreground-dark': '#1E1E1E',
                'foreground': '#FFFFFF',
                'green-500': '#10B981',
                'green-600': '#059669',
                'neutral-dark': '#222222',
                'neutral': '#333333',
                'primary-dark': '#388E3C',
                'primary': '#4CAF50',
                'red-400': '#FCA5A5',
                'red-500': '#F87171',
                'secondary-dark': '#0000CD',
                'secondary': '#0000FF',
                'success-dark': '#007E33',
                'success': '#00C851',
            },
            fontFamily: {
                sans: ['Arial', 'Helvetica', 'sans-serif'],
                heading: ['Georgia', 'serif'],
                mono: ['Menlo', 'Monaco', 'monospace'],
            },
            boxShadow: {
                'sm': '0 1px 2px rgba(0, 0, 0, 0.05)',
                'md': '0 4px 6px rgba(0, 0, 0, 0.1)',
                'lg': '0 10px 15px rgba(0, 0, 0, 0.15)',
                'xl': '0 20px 25px rgba(0, 0, 0, 0.2)',
                '2xl': '0 25px 50px rgba(0, 0, 0, 0.25)',
                'inner': 'inset 0 2px 4px rgba(0, 0, 0, 0.06)',
            },
            spacing: {
                '4': '1rem',
                '8': '2rem',
                '16': '4rem',
                '32': '8rem',
                '64': '16rem',
            },
            borderRadius: {
                'sm': '0.125rem',
                'md': '0.375rem',
                'lg': '0.5rem',
                'full': '9999px',
            },
            transitionTimingFunction: {
                'in-out-quint': 'cubic-bezier(0.83, 0, 0.17, 1)',
            },
        },
    },
    safelist: [
        // Pattern-based safelisting
        {
            pattern: /^(bg|text|border|ring)-(red|blue|green|yellow|gray)-(100|200|300|400|500|600|700|800|900)$/,
        },
        {
            pattern: /^(top|bottom|left|right)-[0-9]+$/,
        },
        {
            pattern: /^(translate|scale|rotate)-(x|y)?-?[\d/]+$/,
        },
        {
            pattern: /^w-(\d+|full|min|max)/,
        },
        {
            pattern: /^h-(\d+|full|min|max|screen)/,
        },
        {
            pattern: /^rounded(-(sm|md|lg|full))?$/,
        },
        {
            pattern: /^text-(xs|sm|base|lg|xl|2xl|3xl|4xl)$/,
        },
        {
            pattern: /^p[trblxy]?-[0-9]+$/,
        },
        {
            pattern: /^m[trblxy]?-?[0-9]+$/,
        },
    ],
    plugins: [],
};