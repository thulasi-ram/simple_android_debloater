/** @type {import('tailwindcss').Config} */
export default {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'
	],
	theme: {
		extend: {
			colors: {
				// primary: {
				// 	50: '#FFF5F2',
				// 	100: '#FFF1EE',
				// 	200: '#FFE4DE',
				// 	300: '#FFD5CC',
				// 	400: '#FFBCAD',
				// 	500: '#FE795D',
				// 	600: '#EF562F',
				// 	700: '#EB4F27',
				// 	800: '#CC4522',
				// 	900: '#A5371B'
				// },
				// https://uicolors.app/browse/tailwind-colors
        // by pasting the icon color
				primary: {
					50: '#f8f9ec',
					100: '#edf1d6',
					200: '#dee5b1',
					300: '#c7d482',
					400: '#aec05b',
					500: '#9fb543',
					600: '#70832d',
					700: '#576526',
					800: '#465123',
					900: '#3c4522',
					950: '#1f250e'
				}
			},
			fontSize: {
				xxs: '0.5rem'
			}
		}
	},
	plugins: [require('flowbite/plugin')],
	darkMode: 'class'
};
