/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        "primary": "#27557A",
        "secondary": "#09814A",
        "error": "#D62839",
        "bg": "#393E41",
        "bg-lighter": "#555B5F",
        "bg-darker": "#26282A",
        "text": "#fff"
      }
    },
  },
  plugins: [],
}
