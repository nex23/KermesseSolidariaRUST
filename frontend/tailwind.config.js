module.exports = {
    content: [
        "./src/**/*.rs",
        "./index.html",
    ],
    theme: {
        extend: {
            colors: {
                primary: '#F97316', // Orange-500
                secondary: '#0D9488', // Teal-600
                dark: '#1F2937', // Gray-800
                light: '#F9FAFB', // Gray-50
                kermesse: {
                    red: '#DC2626',
                    orange: '#EA580C',
                    yellow: '#FACC15',
                }
            },
            fontFamily: {
                sans: ['Inter', 'sans-serif'],
                display: ['Outfit', 'sans-serif'],
            },
        },
    },
    plugins: [],
}
