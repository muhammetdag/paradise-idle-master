<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { games } from '$lib/stores/games';
  import { t, currentLanguage } from '$lib/stores/i18n';
  import GameSearch from '$lib/components/GameSearch.svelte';
  import StatsPanel from '$lib/components/StatsPanel.svelte';
  import GameItem from '$lib/components/GameItem.svelte';
  import ConfirmModal from '$lib/components/ConfirmModal.svelte';
  import { modal } from '$lib/stores/modal';

  async function startAll() {
    for (const game of $games) {
      if (!game.isRunning) {
        try {
          await invoke('start_idler', { appId: game.appId });
          games.start(game.appId);
        } catch (error) {
          console.error(`Failed to start ${game.name}:`, error);
        }
      }
    }
  }

  async function stopAll() {
    for (const game of $games) {
      if (game.isRunning) {
        try {
          await invoke('stop_idler', { appId: game.appId });
          games.stop(game.appId);
        } catch (error) {
          console.error(`Failed to stop ${game.name}:`, error);
        }
      }
    }
  }

  async function removeAll() {
    if ($games.length > 0) {
      modal.confirm({
        title: $t('modal.removeAll.title'),
        message: $t('modal.removeAll.message'),
        onConfirm: async () => {
          await stopAll();
          games.reset();
        }
      });
    }
  }

  function savePreset() {
    if ($games.length === 0) return;
    const cleanGames = $games.map(g => ({ appId: g.appId, name: g.name, image: g.image }));
    localStorage.setItem('paradise_preset_1', JSON.stringify(cleanGames));
    modal.alert($t('modal.savePreset.title'), $t('modal.savePreset.message'));
  }

  function loadPreset() {
    const data = localStorage.getItem('paradise_preset_1');
    if (data) {
      try {
        const parsed = JSON.parse(data);
        for (const g of parsed) {
          games.add(g.appId, g.name, g.image);
        }
      } catch (e) {
        console.error(e);
      }
    } else {
      modal.alert($t('modal.loadPreset.error'), $t('modal.loadPreset.errorMessage'));
    }
  }

  function toggleLanguage() {
    currentLanguage.toggle();
  }

  let searchQuery = '';
  $: filteredGames = $games.filter(game => {
    if (!searchQuery) return true;
    return game.name.toLowerCase().includes(searchQuery.toLowerCase()) || game.appId.toString().includes(searchQuery);
  });
</script>

<div class="app">
  <header class="header">
    <div class="social-buttons">
      <a href="https://paradisedev.org" target="_blank" class="social-btn" title="Website">
        <i class="fa-solid fa-globe"></i>
      </a>
      <a href="https://discord.gg/paradisedev" target="_blank" class="social-btn" title="Discord">
        <i class="fa-brands fa-discord"></i>
      </a>
    </div>
    <div class="header-content">
      <h1 class="title">
        {$t('app.title')}
      </h1>
      <button class="lang-toggle" on:click={toggleLanguage} title="Change Language">
        <i class="fa-solid fa-language"></i>
        {$currentLanguage === 'en' ? 'EN' : 'TR'}
      </button>
    </div>
    <span class="version">{$t('app.copyright')}</span>
  </header>

  <main class="main">
    <StatsPanel />
    <GameSearch />
    
    <div class="games-section">
      <div class="section-header">
        <h2 class="section-title">
          <i class="fa-solid fa-list-ul"></i>
          {$t('games.title')}
        </h2>
        {#if $games.length > 0}
          <div class="list-search-container">
            <i class="fa-solid fa-search list-search-icon"></i>
            <input 
              type="text" 
              bind:value={searchQuery} 
              placeholder={$t('games.searchList')}
              class="list-search-input"
            />
          </div>
          <div class="group-controls">
            <button class="btn-icon-control" on:click={loadPreset} title={$t('games.loadFavorites')}>
              <i class="fa-solid fa-download"></i>
            </button>
            <button class="btn-icon-control" on:click={savePreset} title={$t('games.saveFavorites')}>
              <i class="fa-solid fa-star"></i>
            </button>
            <button class="btn-remove-all" on:click={removeAll}>
              <i class="fa-solid fa-trash-can"></i> {$t('games.removeAll')}
            </button>
          </div>
        {:else}
          <div class="group-controls placeholder">
            <button class="btn-icon-control preset-load-only" on:click={loadPreset} title={$t('games.loadFavorites')}>
              <i class="fa-solid fa-download"></i> {$t('games.loadFavorites')}
            </button>
          </div>
        {/if}
      </div>
      <div class="games-list">
        {#if $games.length === 0}
          <div class="empty-state">
            <div class="empty-icon">
              <i class="fa-solid fa-bullseye"></i>
            </div>
            <p class="empty-text">{$t('games.empty')}</p>
          </div>
        {:else if searchQuery && filteredGames.length === 0}
          <div class="empty-state">
            <div class="empty-icon">
              <i class="fa-solid fa-search"></i>
            </div>
            <p class="empty-text">{$t('games.noResults')}</p>
          </div>
        {:else}
          {#each filteredGames as game (game.appId)}
            <GameItem {game} />
          {/each}
        {/if}
      </div>
    </div>

    {#if $games.length > 0}
      <div class="controls">
        <button class="btn-control btn-start" on:click={startAll}>
          <i class="fa-solid fa-play"></i> {$t('controls.startAll')}
        </button>
        <button class="btn-control btn-stop" on:click={stopAll}>
          <i class="fa-solid fa-stop"></i> {$t('controls.stopAll')}
        </button>
      </div>
    {/if}
  </main>

  <ConfirmModal 
    show={$modal.show}
    title={$modal.title}
    message={$modal.message}
    alertOnly={$modal.alertOnly}
    onConfirm={modal.handleConfirm}
    onCancel={modal.close}
  />
</div>

<style>
  :global(*) {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background: #0f172a;
    color: #e2e8f0;
    overflow: hidden;
  }

  .app {
    width: 420px;
    height: 720px;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .header {
    background: #1e293b;
    padding: 16px;
    text-align: center;
    border-bottom: 1px solid #334155;
    flex-shrink: 0;
    position: relative;
  }

  .social-buttons {
    position: absolute;
    left: 12px;
    top: 12px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    z-index: 10;
  }

  .social-btn {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #334155;
    color: #e2e8f0;
    border-radius: 6px;
    font-size: 14px;
    text-decoration: none;
    transition: all 0.2s;
    cursor: pointer;
  }

  .social-btn:hover {
    background: #3b82f6;
    color: white;
  }

  .header-content {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    position: relative;
  }

  .title {
    margin: 0;
    font-size: 22px;
    font-weight: 700;
    color: #3b82f6;
  }

  .lang-toggle {
    position: absolute;
    right: 0;
    background: #334155;
    color: #e2e8f0;
    border: none;
    border-radius: 6px;
    padding: 6px 10px;
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .lang-toggle:hover {
    background: #3b82f6;
    color: white;
  }

  .main {
    flex: 1;
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    overflow: hidden;
  }

  .games-section {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 8px;
    flex-shrink: 0;
  }

  .section-title {
    margin: 0;
    font-size: 13px;
    font-weight: 600;
    color: #94a3b8;
  }

  .btn-remove-all {
    background: #1e293b;
    color: #ef4444;
    border: 1px solid #334155;
    border-radius: 4px;
    font-size: 10px;
    font-weight: 600;
    padding: 5px 10px;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .btn-remove-all:hover {
    background: #ef4444;
    color: white;
    border-color: #ef4444;
  }

  .group-controls {
    display: flex;
    align-items: center;
    gap: 4px;
    flex-shrink: 0;
  }
  
  .group-controls.placeholder {
    margin-left: auto;
  }

  .btn-icon-control {
    background: #1e293b;
    color: #e2e8f0;
    border: 1px solid #334155;
    border-radius: 4px;
    font-size: 11px;
    padding: 5px 8px;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .btn-icon-control:hover {
    background: #3b82f6;
    color: white;
    border-color: #3b82f6;
  }

  .preset-load-only {
    color: #3b82f6;
    font-weight: 600;
  }

  .list-search-container {
    flex: 1;
    margin: 0 10px;
    position: relative;
    display: flex;
    align-items: center;
  }

  .list-search-icon {
    position: absolute;
    left: 8px;
    font-size: 11px;
    color: #64748b;
  }

  .list-search-input {
    width: 100%;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 4px;
    color: #e2e8f0;
    font-size: 11px;
    padding: 5px 8px 5px 24px;
    outline: none;
    transition: border-color 0.2s;
  }

  .list-search-input:focus {
    border-color: #3b82f6;
  }

  .list-search-input::placeholder {
    color: #64748b;
  }

  .games-list {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 8px;
    overflow-y: auto;
    padding-right: 4px;
  }

  .games-list::-webkit-scrollbar {
    width: 6px;
  }

  .games-list::-webkit-scrollbar-track {
    background: #1e293b;
    border-radius: 3px;
  }

  .games-list::-webkit-scrollbar-thumb {
    background: #334155;
    border-radius: 3px;
  }

  .games-list::-webkit-scrollbar-thumb:hover {
    background: #475569;
  }

  .empty-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 20px;
  }

  .empty-icon {
    font-size: 40px;
    margin-bottom: 8px;
    color: #475569;
    opacity: 0.5;
  }

  .empty-text {
    margin: 0;
    font-size: 14px;
    color: #64748b;
  }

  .controls {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
    flex-shrink: 0;
  }

  .btn-control {
    height: 40px;
    border: none;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-start {
    background: #10b981;
    color: white;
  }

  .btn-start:hover {
    background: #059669;
  }

  .btn-stop {
    background: #ef4444;
    color: white;
  }

  .btn-stop:hover {
    background: #dc2626;
  }
  .version {
    display: block;
    margin-top: 4px;
    font-size: 11px;
    color: #64748b;
  }
</style>
