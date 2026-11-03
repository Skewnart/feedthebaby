<script>
	import { page } from '$app/state';

	let { children } = $props();
</script>

<div class="min-h-screen bg-base-200">
 	{@render children()}
</div>