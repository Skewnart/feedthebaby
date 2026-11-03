<script lang="ts">
    export interface ConfirmParams {
        title?: string;
        message?: string;
        confirmLabel?: string;
        cancelLabel?: string;
        onConfirm?: () => void;
        onCancel?: () => void;
    }

    type ConfirmDialogProps = ConfirmParams & {
        visible?: boolean;
    };

    let { visible = false,
        title = 'Confirmer',
        message = '',
        confirmLabel = 'Confirmer',
        cancelLabel = 'Annuler',
        onConfirm,
        onCancel
    }: ConfirmDialogProps = $props();

    let dialog: HTMLDialogElement | null = null;

    $effect(() => {
        if (!dialog) return;
        if (visible) {
            dialog.showModal();
        } else {
            if (dialog.open) {
                dialog.close();
            }
        }
    });

    function handleCancel() {
        onCancel?.();
    }

    function handleConfirm() {
        dialog?.close();
        onConfirm?.();
    }
</script>

<dialog bind:this={dialog} class="modal" oncancel={handleCancel}>
    <div class="modal-box">
        <h3 class="text-lg font-bold">{title}</h3>
        <p class="py-4">{message}</p>
        <div class="modal-action">
            <button class="btn btn-error" type="button" onclick={handleConfirm}>{confirmLabel}</button>
            <button class="btn btn-secondary" type="button" onclick={handleCancel}>{cancelLabel}</button>
        </div>
    </div>
</dialog>
