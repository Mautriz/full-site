<script lang="ts">
	import { kratos } from '$lib/ts/kratos';
	import { onMount } from 'svelte';

	let session = null;

	onMount(async () => {
		session = await kratos.getSession().then((r) => r.json());
	});
</script>

<svelte:head>
	<title>Model Maker by Mauro Insacco</title>
</svelte:head>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

{#if session}
	<div>
		<div>My session is:</div>
		<hr />
		<pre>{JSON.stringify(session, null, 4)}</pre>
	</div>
{/if}

<button on:click={kratos.logout}>Logout</button>
