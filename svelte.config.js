import { vitePreprocess } from '@sveltejs/kit/vite';
import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter({
			// should be index.html and not app.html since vite converts app.html to index.html
			// tauri expects index.html to be present in ../build directory
			fallback: 'index.html' 
		})
	}
};

export default config;
