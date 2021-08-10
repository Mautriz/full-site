<script lang="ts" context="module">
	import { getBlogPost } from '$lib/ts/assets';
	import type { Load } from '@sveltejs/kit';
	import marked from 'marked'
	export const prerender = true
	export const load: Load = async ({ fetch, page }) => {
		const { slug } = page.params;
		let post = marked(await getBlogPost(slug + '.md', fetch));
		return {
			props: {
				post
			}
		};
	};
</script>

<script lang="ts">
	export let post: string;
</script>

<div>
	{@html post}
</div>
