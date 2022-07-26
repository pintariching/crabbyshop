/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
	"./src/*.{html,svelte}",
	"./src/routes/*.svelte"
  ],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
}
