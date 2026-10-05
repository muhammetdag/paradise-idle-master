<script lang="ts">
  import { games } from '../stores/games';
  import { t } from '../stores/i18n';
  import { derived } from 'svelte/store';

  const stats = derived(games, $games => {
    let totalSeconds = 0;
    for (const g of $games) {
      if (g.elapsedTime) totalSeconds += g.elapsedTime;
    }
    return {
      total: $games.length,
      active: $games.filter(g => g.isRunning).length,
      stopped: $games.filter(g => !g.isRunning).length,
      totalSeconds
    };
  });

  function formatStatsTime(seconds: number): string {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    return `${hours}${$t('stats.hours')} ${minutes}${$t('stats.minutes')}`;
  }
</script>

<div class="stats-panel">
  <div class="stat-item">
    <div class="stat-value">{$stats.total}</div>
    <div class="stat-label">{$t('stats.total')}</div>
  </div>
  
  <div class="stat-item">
    <div class="stat-value active">{$stats.active}</div>
    <div class="stat-label">{$t('stats.active')}</div>
  </div>
  
  <div class="stat-item">
    <div class="stat-value stopped">{$stats.stopped}</div>
    <div class="stat-label">{$t('stats.stopped')}</div>
  </div>

  <div class="stat-divider"></div>

  <div class="stat-item">
    <div class="stat-value time">{formatStatsTime($stats.totalSeconds)}</div>
    <div class="stat-label">{$t('stats.totalTime')}</div>
  </div>
</div>

<style>
  .stats-panel {
    display: flex;
    align-items: center;
    justify-content: space-around;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 6px;
    padding: 12px;
  }

  .stat-item {
    text-align: center;
  }

  .stat-value {
    font-size: 20px;
    font-weight: 700;
    color: #3b82f6;
    margin-bottom: 2px;
  }

  .stat-value.active {
    color: #10b981;
  }

  .stat-value.stopped {
    color: #f59e0b;
  }

  .stat-value.time {
    color: #a855f7;
    font-size: 16px;
    padding-top: 2px;
  }

  .stat-label {
    font-size: 9px;
    color: #64748b;
    text-transform: uppercase;
    letter-spacing: 0.3px;
  }

  .stat-divider {
    width: 1px;
    height: 30px;
    background: #334155;
    margin: 0 5px;
  }
</style>
