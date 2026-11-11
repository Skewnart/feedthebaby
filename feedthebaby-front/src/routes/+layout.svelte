<script>
	import { page } from '$app/state';

	let { children } = $props();
</script>

<div data-theme="lemonade" class="min-h-screen bg-base-200">
 	{@render children()}
</div>