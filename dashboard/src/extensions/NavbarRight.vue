<script setup lang="ts">
import { useProgramStore } from '@/stores/program';

const store = useProgramStore();
</script>

<template>
  <!-- OSS: live connection indicator -->
  <div
    class="flex items-center gap-2 px-3 py-1.5 rounded-full bg-base-200/50 border border-base-200"
    :title="store.isConnected ? 'Realtime Connected' : 'Disconnected'"
  >
    <span class="relative flex h-2 w-2">
      <span v-if="store.isConnected" class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
      <span class="relative inline-flex rounded-full h-2 w-2" :class="store.isConnected ? 'bg-emerald-500' : 'bg-rose-500'"></span>
    </span>
    <span
      class="text-xs font-semibold tracking-wide hidden sm:inline"
      :class="store.isConnected ? 'text-emerald-600' : 'text-rose-500'"
    >
      {{ store.isConnected ? 'LIVE' : 'OFFLINE' }}
    </span>
  </div>
</template>
