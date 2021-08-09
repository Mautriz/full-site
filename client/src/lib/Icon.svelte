<script>
	import { svgIcons } from './ts/icons';
	export let icon = 'award-fill';

	$: iconObject = svgIcons[icon];
</script>

<svg class="text-success" width="32" height="32" viewBox={iconObject.viewBox} fill="currentColor">
	{@html iconObject.svg}
</svg>
