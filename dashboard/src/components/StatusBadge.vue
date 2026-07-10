<script setup lang="ts">
import type { ProcessStatus } from '@/types';

defineProps<{
  status: ProcessStatus
}>();

// Status-to-badge color mapping
const colorMap: Record<ProcessStatus, string> = {
  Running: 'badge-success',
  Healthy: 'badge-success',
  Stopped: 'badge-neutral',
  Fatal: 'badge-error',
  Backoff: 'badge-warning',
  Starting: 'badge-info',
  Stopping: 'badge-warning',
  Waiting: 'badge-ghost'
};
</script>

<template>
  <div class="badge gap-2" :class="colorMap[status] || 'badge-ghost'">
    <!-- Pulsing dot for active states -->
    <span v-if="['Running', 'Healthy'].includes(status)" class="w-2 h-2 rounded-full bg-current animate-pulse"></span>
    {{ status }}
  </div>
</template>
