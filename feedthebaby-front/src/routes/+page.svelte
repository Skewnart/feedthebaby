<script lang="ts">
	import { onMount, tick } from 'svelte';
	import '../app.css';

	import type { AlimentInNewIdDTO, AlimentOutDTO } from '$lib/models/aliment';
	import { get_all, post_bulk } from '$lib/api';
	import ConfirmDialog from '$lib/components/ConfirmDialog.svelte';
	import Toast from '$lib/components/Toast.svelte';
	
	const TITLE = "Feed the baby";
	const alimentGroups = [
		{ type: '10g', title: 'Petit rond (10g)' },
		{ type: '20g', title: 'Rectangle (20g)' },
		{ type: '25g', title: 'Rectangle (25g)' }
	] as const;

	function getTodayDate() {
		const today = new Date();
		const month = String(today.getMonth() + 1).padStart(2, '0');
		const day = String(today.getDate()).padStart(2, '0');
		return `${today.getFullYear()}-${month}-${day}`;
	}

	function formatDateForStorage(date: string) {
		const [year, month, day] = date.split('-');
		return `${day}/${month}/${year}`;
	}
	const props = $props();
	let error = $state("");
	let aliments = $state<(AlimentOutDTO & { quantity_change?: number })[] | undefined>(undefined);
	let isSaving = $state(false);
	let isSuccessToastVisible = $state(false);
	let isAddPanelOpen = $state(false);
	let deleteCandidateId = $state<number | undefined>(undefined);
	let nameInput = $state<HTMLInputElement | undefined>(undefined);
	let newAlimentName = $state("");
	let newQuantity = $state(1);
	let newAlimentType = $state<(typeof alimentGroups)[number]['type']>('10g');
	let newExpireAt = $state(getTodayDate());
		
	let alimentQuery = $state<string>("");
	const filteredAliments = $derived(
		aliments
		?.filter(f => f.persistence_type !== 'DELETE' && f.name.toLowerCase().includes(alimentQuery.toLowerCase()))
		.sort((a, b) => a.name.localeCompare(b.name, 'fr', { sensitivity: 'base' }))
	);
	const groupedAliments = $derived(
		alimentGroups.map(group => ({
			...group,
			aliments: filteredAliments?.filter(aliment => aliment.aliment_type === group.type) ?? []
		}))
	);
	const hasDuplicateAlimentName = $derived(
		newAlimentName.trim() !== "" && (aliments?.some(aliment =>
			aliment.persistence_type !== 'DELETE' &&
			aliment.aliment_type === newAlimentType &&
			aliment.name.trim().toLocaleLowerCase('fr') === newAlimentName.trim().toLocaleLowerCase('fr')
		) ?? false)
	);
	const hasAlreadyDeleteAliment = $derived(
		newAlimentName.trim() !== "" && (aliments?.some(aliment =>
			aliment.persistence_type === 'DELETE' &&
			aliment.aliment_type === newAlimentType &&
			aliment.name.trim().toLocaleLowerCase('fr') === newAlimentName.trim().toLocaleLowerCase('fr')
		) ?? false)
	);
	const deleteCandidateName = $derived(aliments?.find(aliment => aliment.id === deleteCandidateId)?.name);
	const hasPendingChanges = $derived(aliments?.some(aliment => aliment.persistence_type !== '') ?? false);

	if (!props.data) {
		error = "Erreur pendant le chargement des données sur le serveur";
	}
	const data = props.data;

	if (data.token == undefined) {
		error = "Le token n'est pas présent depuis le chargement de la page.";
	}
	const token = data.token;
	
	onMount(async () => {
		get_all(token).then(al => aliments = al.map(aliment => ({ ...aliment, persistence_type: '' })));
	});

	function addAliment(event: SubmitEvent) {
		event.preventDefault();
		const name = newAlimentName.trim();
		if (!name || !aliments || hasDuplicateAlimentName) return;

		const aliment = {
			id: Math.max(0, ...aliments.map(item => item.id)) + 1,
			name,
			quantity: newQuantity,
			expire_at: formatDateForStorage(newExpireAt),
			aliment_type: newAlimentType,
			persistence_type: 'CREATE'
		};
		aliments = [...aliments, aliment];
		newAlimentName = "";
		newQuantity = 1;
		newAlimentType = '10g';
		newExpireAt = getTodayDate();
		isAddPanelOpen = false;
	}

	function changeQuantity(id: number, amount: number) {
		if (!aliments) return;

		aliments = aliments.map(aliment => {
			if (aliment.id !== id) return aliment;

			const quantity = Math.max(0, aliment.quantity + amount);
			const change = quantity - aliment.quantity;
			if (change === 0) return aliment;

			if (quantity == 0) deleteCandidateId = id;

			return {
				...aliment,
				quantity,
				quantity_change: (aliment.quantity_change ?? 0) + change,
				persistence_type: aliment.persistence_type || 'UPDATE'
			};
		});
	}

	function confirmDeleteAliment() {
		if (!aliments || deleteCandidateId === undefined) return;

		const candidate = aliments.find(aliment => aliment.id === deleteCandidateId);
		if (candidate?.persistence_type === 'CREATE') {
			aliments = aliments.filter(aliment => aliment.id !== deleteCandidateId);
		} else if (candidate?.persistence_type === 'UPDATE') {
			aliments = aliments.map(aliment => aliment.id === deleteCandidateId
				? { ...aliment, persistence_type: 'DELETE' }
				: aliment
			);
		}
		deleteCandidateId = undefined;
	}

	function cancelDeleteAliment() {
		deleteCandidateId = undefined;
	}

	async function saveAliments() {
		if (!aliments || isSaving) return;

		const changedAliments = aliments.filter(aliment => aliment.persistence_type !== '');
		if (changedAliments.length === 0) return;

		isSaving = true;
		error = '';
		try {
			const payload = changedAliments.map(({ quantity_change, ...aliment }) => aliment);
			const response = await post_bulk(token, payload);
			if (!response.ok) {
				error = 'Erreur pendant la sauvegarde des aliments.';
				return;
			}
			const createdIdMappings = await response.json() as AlimentInNewIdDTO[];
			const newIdByOldId = new Map(createdIdMappings.map(({ old_id, new_id }) => [old_id, new_id]));

			aliments = aliments
				.filter(aliment => aliment.persistence_type !== 'DELETE')
				.map(({ quantity_change, ...aliment }) => ({
					...aliment,
					id: aliment.persistence_type === 'CREATE'
						? newIdByOldId.get(aliment.id) ?? aliment.id
						: aliment.id,
					persistence_type: ''
				}));
				
			isSuccessToastVisible = true;
				
		} catch {
			error = 'Erreur pendant la sauvegarde des aliments.';
		} finally {
			isSaving = false;
		}
	}

	async function openAddPanel() {
		isAddPanelOpen = true;
		await tick();
		nameInput?.focus();
	}

</script>

<svelte:head>
	<title>{TITLE}</title>
	<meta name="description" content="What's in the fridge ?" />
</svelte:head>

<div class="flex justify-center">
	<div class="md:w-3/4 w-full mt-4 mx-4">
		<header class="mt-10 mb-8">
			<h1 class="text-3xl md:text-4xl font-semibold">Qu'est-ce qu'il y a dans mon frigo ?</h1>
			<p class="italic text-lg mt-2">du moins dans le congel</p>
		</header>
		
		{#if error}
			<div role="alert" class="alert alert-error">
				<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
				</svg>
				<span>{error}</span>
			</div>
		{/if}

		{#if !!aliments}
			<div class="mb-2">
				<input class="input input-bordered w-full" placeholder="Écrivez pour commencer à chercher..." bind:value={alimentQuery} aria-label="Recherche dossiers" />
			</div>
			<div class="overflow-x-auto mt-6">
				<table class="table bg-base-100 rounded-none" aria-label="Aliments">
					<thead>
						<tr>
							<th class="text-left">Nom</th>
							<th class="text-center">Quantité</th>
							<th class="text-right">Date de conception</th>
						</tr>
					</thead>
					{#if alimentQuery && filteredAliments?.length === 0}
						<tbody>
							<tr>
								<td colspan="3" class="text-center opacity-60">Aucun dossier ne correspond à la recherche</td>
							</tr>
						</tbody>
					{:else}
						{#each groupedAliments as group}
							<tbody>
								<tr>
									<th colspan="3" scope="rowgroup" class="text-left normal-case text-base font-semibold opacity-100">{group.title}</th>
								</tr>
								{#if group.aliments.length === 0}
									<tr>
										<td colspan="3" class="text-center opacity-60">Aucun aliment dans ce groupe</td>
									</tr>
								{:else}
								{#each group.aliments as aliment}
									<tr>
										<td class="text-left">{aliment.name}</td>
										<td class="text-center">
											<div class="inline-flex items-center gap-2">
												<button class="btn btn-ghost btn-square btn-xs" type="button" aria-label="Diminuer la quantité de {aliment.name}" disabled={aliment.quantity <= 0} onclick={() => changeQuantity(aliment.id, -1)}>-</button>
												<span class="min-w-6 text-center">{aliment.quantity}</span>
												<button class="btn btn-ghost btn-square btn-xs" type="button" aria-label="Augmenter la quantité de {aliment.name}" onclick={() => changeQuantity(aliment.id, 1)}>+</button>
												{#if aliment.quantity_change}
													<span class="font-semibold tabular-nums" class:text-green-800={aliment.quantity_change > 0} class:text-red-800={aliment.quantity_change < 0}>({aliment.quantity_change > 0 ? '+' : ''}{aliment.quantity_change})</span>
												{/if}
											</div>
										</td>
										<td class="text-right">{aliment.expire_at}</td>
									</tr>
								{/each}
								{/if}
							</tbody>
							{/each}
						{/if}
				</table>
			</div>
			<button class="btn btn-primary btn-block my-4" type="button" onclick={openAddPanel}>Ajouter un aliment</button>
			{#if hasPendingChanges}
				<button class="btn btn-secondary btn-block mb-4" type="button" onclick={saveAliments} disabled={isSaving}>
					{isSaving ? 'Sauvegarde…' : 'Sauvegarder'}
				</button>
			{/if}
		{:else}
			<div class="flex justify-center">
				<span class="loading loading-spinner text-primary"></span>
			</div>
		{/if}
	</div>
</div>

{#if isAddPanelOpen}
	<button class="fixed inset-0 z-40 cursor-default bg-black/40" type="button" aria-label="Fermer le panneau d'ajout" onclick={() => isAddPanelOpen = false}></button>
	<dialog open class="fixed inset-y-0 right-0 z-50 m-0 w-full max-w-md overflow-y-auto border-0 bg-base-100 p-6 shadow-2xl" style="position: fixed; inset: 0 0 0 auto; margin: 0; width: min(100%, 28rem); height: 100dvh; max-height: none;" aria-labelledby="add-aliment-title">
		<div class="mb-8 flex items-center justify-between gap-4">
			<h2 id="add-aliment-title" class="text-2xl font-semibold">Ajouter un aliment</h2>
			<button class="btn btn-ghost btn-sm" type="button" aria-label="Fermer" onclick={() => isAddPanelOpen = false}>✕</button>
		</div>
		<form class="flex flex-col gap-5" onsubmit={addAliment}>
			<label class="form-control w-full">
				<span class="label-text mb-2">Nom</span>
				<input class="input input-bordered w-full" type="text" bind:this={nameInput} bind:value={newAlimentName} aria-invalid={hasDuplicateAlimentName} required />
			</label>
			{#if hasDuplicateAlimentName}
				<p class="text-sm text-error" role="alert">Un aliment avec ce nom existe déjà dans ce groupe.</p>
			{/if}
			{#if hasAlreadyDeleteAliment}
				<p class="text-sm text-error" role="alert">Vous venez de supprimer cet aliment. Merci de sauvegarder avant de continuer.</p>
			{/if}
			<label class="form-control w-full">
				<span class="label-text mb-2">Quantité de base</span>
				<input class="input input-bordered w-full" type="number" min="1" step="1" bind:value={newQuantity} required />
			</label>
			<label class="form-control w-full">
				<span class="label-text mb-2">Groupe</span>
				<select class="select select-bordered w-full" bind:value={newAlimentType}>
					{#each alimentGroups as group}
						<option value={group.type}>{group.type}</option>
					{/each}
				</select>
			</label>
			<label class="form-control w-full">
				<span class="label-text mb-2">Date de conception</span>
				<input class="input input-bordered w-full" type="date" bind:value={newExpireAt} required />
			</label>
			<div class="mt-4 flex justify-end gap-3">
				<button class="btn btn-ghost" type="button" onclick={() => isAddPanelOpen = false}>Annuler</button>
				<button class="btn btn-primary" type="submit" disabled={hasDuplicateAlimentName}>Ajouter</button>
			</div>
		</form>
	</dialog>
{/if}

<ConfirmDialog
	visible={deleteCandidateId !== undefined}
	title="Supprimer cet aliment ?"
	message={deleteCandidateName ? `Voulez-vous supprimer ${deleteCandidateName} ?` : 'Voulez-vous supprimer cet aliment ?'}
	confirmLabel="Oui, supprimer"
	cancelLabel="Non, garder"
	onConfirm={confirmDeleteAliment}
	onCancel={cancelDeleteAliment}
/>

<Toast
	visible={isSuccessToastVisible}
	message="La sauvegarde des aliments a réussi."
	alertType="success"
	onTimeoutEnds={() => isSuccessToastVisible = false}
/>