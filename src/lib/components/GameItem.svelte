<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { games } from '../stores/games';
  import { modal } from '../stores/modal';
  import { t } from '../stores/i18n';
  import type { Game } from '../types';
  import { onDestroy } from 'svelte';

  export let game: Game;

  let timerInterval: number;

  function formatTime(seconds: number): string {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }

  async function toggleGame() {
    if (game.isRunning) {
      await stopGame();
    } else {
      await startGame();
    }
  }

  async function startGame() {
    try {
      await invoke('start_idler', { appId: game.appId });
      games.start(game.appId);
      
      timerInterval = setInterval(() => {
        if (game.startTime) {
          const elapsed = Math.floor((Date.now() - game.startTime) / 1000);
          games.updateElapsed(game.appId, elapsed);
        }
      }, 1000);
    } catch (error) {
      console.error('Failed to start game:', error);
      modal.alert($t('modal.gameStart.error'), `${$t('modal.gameStart.errorMessage')}: ${error}`);
    }
  }

  async function stopGame() {
    try {
      await invoke('stop_idler', { appId: game.appId });
      games.stop(game.appId);
      clearInterval(timerInterval);
    } catch (error) {
      console.error('Failed to stop game:', error);
    }
  }

  async function removeGame() {
    if (game.isRunning) {
      await stopGame();
    }
    games.remove(game.appId);
  }

  onDestroy(() => {
    if (timerInterval) clearInterval(timerInterval);
  });

</script>

<div class="game-item" class:running={game.isRunning}>
  <div class="game-main">
    <div class="game-icon">
      {#if game.image}
        <img src={game.image} alt={game.name} class="game-image" />
      {:else}
        <i class="fa-solid fa-gamepad"></i>
      {/if}
    </div>
    
    <div class="game-info">
      <div class="game-name">{game.name}</div>
      <div class="game-meta">
        <span class="game-appid">ID: {game.appId}</span>
        <span class="game-timer">{formatTime(game.elapsedTime)}</span>
      </div>
    </div>

    <button class="btn-remove" on:click={removeGame} title={$t('game.remove')}>
      <i class="fa-solid fa-xmark"></i>
    </button>
  </div>

  <div class="game-controls">
    <div class="game-status">
      <span class="status-dot" class:active={game.isRunning}></span>
      <span class="status-text">{game.isRunning ? $t('game.status.active') : $t('game.status.ready')}</span>
    </div>

    <button class="btn-action" on:click={toggleGame}>
      <i class="fa-solid {game.isRunning ? 'fa-stop' : 'fa-play'}" style="margin-right: 4px;"></i>
      {game.isRunning ? $t('game.stop') : $t('game.start')}
    </button>
  </div>
</div>

<style>
  .game-item {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 6px;
    padding: 10px;
    transition: border-color 0.2s;
  }

  .game-item:hover {
    border-color: #475569;
  }

  .game-item.running {
    border-color: #10b981;
  }

  .game-main {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 8px;
  }

  .game-icon {
    width: 44px;
    height: 22px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 18px;
    color: #64748b;
    flex-shrink: 0;
    overflow: hidden;
    border-radius: 3px;
    background: #0f172a;
  }

  .game-image {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .game-info {
    flex: 1;
    min-width: 0;
  }

  .game-name {
    font-size: 13px;
    font-weight: 600;
    color: #e2e8f0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin-bottom: 3px;
  }

  .game-meta {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 10px;
    color: #64748b;
  }

  .game-appid {
  }

  .game-timer {
    font-family: 'Consolas', monospace;
  }

  .btn-remove {
    width: 28px;
    height: 28px;
    background: transparent;
    color: #e2e8f0;
    border: none;
    border-radius: 4px;
    font-size: 14px;
    cursor: pointer;
    transition: background 0.2s;
    flex-shrink: 0;
  }

  .btn-remove:hover {
    background: #ef4444;
  }

  .game-controls {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
  }

  .game-status {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .status-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #64748b;
  }

  .status-dot.active {
    background: #10b981;
    animation: pulse 2s infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.4; }
  }

  .status-text {
    font-size: 11px;
    font-weight: 600;
    color: #94a3b8;
  }

  .btn-action {
    padding: 6px 16px;
    background: #10b981;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-action:hover {
    background: #059669;
  }

  .game-item.running .btn-action {
    background: #ef4444;
  }

  .game-item.running .btn-action:hover {
    background: #dc2626;
  }
</style>
