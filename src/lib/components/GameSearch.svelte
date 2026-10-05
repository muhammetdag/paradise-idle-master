<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { SteamSearchResponse, SteamSearchItem } from '../types';
  import { games } from '../stores/games';
  import { modal } from '../stores/modal';
  import { t } from '../stores/i18n';

  let searchQuery = '';
  let searchResults: SteamSearchItem[] = [];
  let isSearching = false;
  let showResults = false;
  let searchTimeout: number;

  async function searchGames() {
    if (searchQuery.trim().length < 2) {
      searchResults = [];
      showResults = false;
      return;
    }

    isSearching = true;
    try {
      const data = await invoke<SteamSearchResponse>('search_steam_games', { 
        query: searchQuery 
      });
      searchResults = data.items.slice(0, 8);
      showResults = true;
    } catch (error) {
      console.error('Search failed:', error);
      searchResults = [];
    } finally {
      isSearching = false;
    }
  }

  function handleInput() {
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(searchGames, 300);
  }

  function selectGame(game: SteamSearchItem) {
    games.add(game.id.toString(), game.name, game.tiny_image);
    searchQuery = '';
    searchResults = [];
    showResults = false;
  }

  async function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter' && searchQuery.trim().length > 0) {
      const query = searchQuery.trim();
      
      // If query is numeric, fetch details and add
      if (/^\d+$/.test(query)) {
        isSearching = true;
        try {
          const details = await invoke<{ name: string; header_image: string }>(
            'get_steam_game_details', 
            { appId: query }
          );
          games.add(query, details.name, details.header_image);
          searchQuery = '';
          searchResults = [];
          showResults = false;
        } catch (error) {
          console.error('Failed to get game details:', error);
          modal.alert($t('modal.appId.error'), $t('modal.appId.errorMessage'));
        } finally {
          isSearching = false;
        }
      } else if (searchResults.length > 0) {
        // If not numeric but has results, select the first one
        selectGame(searchResults[0]);
      }
    }
  }

  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (!target.closest('.search-container')) {
      showResults = false;
    }
  }
</script>

<svelte:window on:click={handleClickOutside} />

<div class="search-container">
  <div class="search-input-wrapper">
    <span class="search-icon">
      <i class="fa-solid fa-magnifying-glass"></i>
    </span>
    <input
      type="text"
      bind:value={searchQuery}
      on:input={handleInput}
      on:keydown={handleKeydown}
      placeholder={$t('search.placeholder')}
      class="search-input"
    />
    {#if isSearching}
      <div class="search-spinner">
        <i class="fa-solid fa-spinner fa-spin"></i>
      </div>
    {/if}
  </div>

  {#if showResults && searchResults.length > 0}
    <div class="search-results">
      {#each searchResults as game}
        <button class="result-item" on:click={() => selectGame(game)}>
          <img src={game.tiny_image} alt={game.name} class="result-image" />
          <div class="result-info">
            <div class="result-name">{game.name}</div>
            <div class="result-appid">ID: {game.id}</div>
          </div>
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .search-container {
    position: relative;
  }

  .search-input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  .search-icon {
    position: absolute;
    left: 12px;
    font-size: 14px;
    pointer-events: none;
  }

  .search-input {
    width: 100%;
    height: 40px;
    padding: 0 40px 0 36px;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 6px;
    color: #e2e8f0;
    font-size: 13px;
    transition: border-color 0.2s;
  }

  .search-input:focus {
    outline: none;
    border-color: #3b82f6;
  }

  .search-input::placeholder {
    color: #64748b;
  }

  .search-spinner {
    position: absolute;
    right: 12px;
    font-size: 14px;
  }

  .search-results {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    right: 0;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 6px;
    max-height: 280px;
    overflow-y: auto;
    z-index: 100;
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.4);
  }

  .search-results::-webkit-scrollbar {
    width: 6px;
  }

  .search-results::-webkit-scrollbar-track {
    background: #1e293b;
  }

  .search-results::-webkit-scrollbar-thumb {
    background: #334155;
    border-radius: 3px;
  }

  .result-item {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px;
    background: transparent;
    border: none;
    border-bottom: 1px solid #334155;
    cursor: pointer;
    transition: background 0.15s;
  }

  .result-item:last-child {
    border-bottom: none;
  }

  .result-item:hover {
    background: #334155;
  }

  .result-image {
    width: 50px;
    height: 25px;
    object-fit: cover;
    border-radius: 3px;
  }

  .result-info {
    flex: 1;
    text-align: left;
    min-width: 0;
  }

  .result-name {
    color: #e2e8f0;
    font-size: 12px;
    font-weight: 500;
    margin-bottom: 2px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .result-appid {
    color: #64748b;
    font-size: 10px;
  }
</style>
