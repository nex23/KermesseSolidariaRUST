module.exports = {
    content: [
        "./frontend/src/**/*.rs",
        "./frontend/index.html",
    ],
    theme: {
        extend: {
            colors: {
                primary: '#FF6B6B', // Vibrant Red/Coral
                secondary: '#4ECDC4', // Teal
                dark: '#2D3436',
                light: '#F7F7F7',
            },
            fontFamily: {
                sans: ['Roboto', 'sans-serif'],
            },
        },
    },
    plugins: [],
}
