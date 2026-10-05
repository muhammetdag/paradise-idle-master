<script lang="ts">
  import { fade, scale } from 'svelte/transition';
  import { t } from '../stores/i18n';

  export let show = false;
  export let title = '';
  export let message = '';
  export let alertOnly = false;
  export let onConfirm: () => void;
  export let onCancel: () => void = () => {};
</script>

{#if show}
  <div class="modal-overlay" transition:fade={{ duration: 150 }} on:click={alertOnly ? onConfirm : onCancel}>
    <div class="modal-content" transition:scale={{ duration: 200, start: 0.95 }} on:click|stopPropagation>
      <div class="modal-header">
        <i class="fa-solid {alertOnly ? 'fa-circle-info' : 'fa-triangle-exclamation'} modal-icon" class:info={alertOnly}></i>
        <h3 class="modal-title">{title}</h3>
      </div>
      
      <p class="modal-message">{message}</p>
      
      <div class="modal-actions">
        {#if !alertOnly}
          <button class="btn-cancel" on:click={onCancel}>{$t('modal.cancel')}</button>
        {/if}
        <button class="btn-confirm" class:btn-info={alertOnly} on:click={onConfirm}>
          {alertOnly ? $t('modal.ok') : $t('modal.confirm')}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-content {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 12px;
    padding: 24px;
    width: 320px;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5);
  }

  .modal-header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 16px;
  }

  .modal-icon {
    font-size: 20px;
    color: #ef4444;
  }

  .modal-icon.info {
    color: #3b82f6;
  }

  .modal-title {
    margin: 0;
    font-size: 18px;
    font-weight: 700;
    color: #f8fafc;
  }

  .modal-message {
    margin: 0 0 24px 0;
    font-size: 14px;
    color: #94a3b8;
    line-height: 1.5;
  }

  .modal-actions {
    display: flex;
    gap: 12px;
  }

  button {
    flex: 1;
    height: 40px;
    border: none;
    border-radius: 6px;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-cancel {
    background: #334155;
    color: #e2e8f0;
  }

  .btn-cancel:hover {
    background: #475569;
  }

  .btn-confirm {
    background: #ef4444;
    color: white;
  }

  .btn-confirm:hover {
    background: #dc2626;
  }

  .btn-confirm.btn-info {
    background: #3b82f6;
  }

  .btn-confirm.btn-info:hover {
    background: #2563eb;
  }
</style>
