<script lang="ts">
	import { kratos } from '$lib/ts/kratos';
	import { onMount } from 'svelte';

	let flow = null;

	$: google = flow?.ui?.nodes.find((e) => e.attributes.name === 'provider');
	$: csrf = flow?.ui?.nodes.find((e) => e.attributes.name === 'csrf_token');

	$: console.log(flow, google, csrf);

	onMount(async () => {
		flow = await kratos.getLoginFlow().then((r) => r.json());
	});
</script>

<div>
	<p>Il flow è:</p>

	{#if flow}
		<form action={flow.ui.action} method={flow.ui.method}>
			<input {...csrf.attributes} />
			<input {...google.attributes} />
		</form>
	{/if}
</div>
