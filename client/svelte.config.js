import preprocess from 'svelte-preprocess';
import PurgeSvelte from 'purgecss-from-svelte';
import purgecss from '@fullhuman/postcss-purgecss';

const isProd = process.env.NODE_ENV === 'production';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	compilerOptions: {
		immutable: true
	},
	preprocess: preprocess({
		// postcss: isProd && {
		// 	plugins: [
		// 		purgecss({
		// 			content: ['./src/**/*.svelte'],
		// 			extractors: [
		// 				{
		// 					extractor: (content) => PurgeSvelte.extract(content),
		// 					extensions: ['svelte']
		// 				}
		// 			],
		// 			safelist: ['html', 'body']
		// 		})
		// 	]
		// }
	}),
	kit: {
		// ssr: false,
		// hydrate the <div id="svelte"> element in src/app.html
		target: '#svelte'
	}
};

export default config;
