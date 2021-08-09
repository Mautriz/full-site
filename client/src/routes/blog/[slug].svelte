<script lang="ts" context="module">
	import { getAssetAsString } from '$lib/ts/assets';
	import type { Load } from '@sveltejs/kit';
	export const load: Load = (ao) => {
		console.log(ao);
		// getAssetAsString()

		return {
			props: {
				ao
			}
		};
	};
</script>

<script lang="ts">
	export let ao: any;
</script>

<div>
	{JSON.stringify(ao)}
</div>
