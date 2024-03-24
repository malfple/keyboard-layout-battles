import { purgeCss } from 'vite-plugin-tailwind-purgecss';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit(), purgeCss()],
	server: {
		// disable proxy because there is already proxy in hooks
		// proxy: {
		// 	'/api/': {
		// 		target: 'http://localhost:3007',
		// 		rewrite: (path) => path.replace(/^\/api/, '')
		// 	}
		// }
	}
});