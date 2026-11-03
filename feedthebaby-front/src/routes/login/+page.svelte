<script lang="ts">

	import { fade } from 'svelte/transition';
	import "../../app.css";
    import { goto } from '$app/navigation';

    import { login, set_token } from '$lib/api';

	const TITLE = "Connexion";

	let password = $state('');
	let error = $state("");
	let callPending = $state(false);

	async function onsubmit(event: Event) {
		error = "";
		callPending = true;
		event.preventDefault();

		try {
			const login_response = await login(password);
			if (login_response.status !== 200) {
				console.log("error");
				error = "Mauvais mot de passe"
				callPending = false;
				return;
			}

			const token = await login_response.text();
			await set_token(token);

			goto('/');
		} catch (e) {
			error = "Une erreur est survenue";
			callPending = false;
		}
	}
</script>

<svelte:head>
	<title>{TITLE}</title>
</svelte:head>

<section class="min-h-screen flex items-center justify-center bg-base-200">
	<div class="w-full max-w-xs">
		<div class="card shadow-lg bg-base-100 p-8">
			<h1 class="text-3xl font-bold text-center mb-8">{TITLE}</h1>
			<form {onsubmit}>
				<div class="mb-4">
					<label class="input w-full">
						<svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
							<g
							stroke-linejoin="round"
							stroke-linecap="round"
							stroke-width="2.5"
							fill="none"
							stroke="currentColor"
							>
							<path
								d="M2.586 17.414A2 2 0 0 0 2 18.828V21a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h1a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h.172a2 2 0 0 0 1.414-.586l.814-.814a6.5 6.5 0 1 0-4-4z"
							></path>
							<circle cx="16.5" cy="7.5" r=".5" fill="currentColor"></circle>
							</g>
						</svg>
						<input
							id="password"
							type="password"
							placeholder="Mot de passe de connexion"
							required
							bind:value={password}
							disabled={callPending}
							autofocus
						/>
					</label>
				</div>

				<button class="btn btn-primary w-full" type="submit" disabled={callPending}>
					{#if callPending}
						<span class="loading loading-dots loading-xl"></span>
					{:else}
						Connexion
					{/if}
				</button>
			</form>

			{#if error}
				<div role="alert" class="alert alert-error mt-2" transition:fade>
					<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
					</svg>
					<span>{error}</span>
				</div>
			{/if}
		</div>
	</div>
</section>
