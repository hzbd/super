<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { Cpu, MemoryStick } from 'lucide-vue-next';
import { API_PATHS } from '@/api/paths';

interface SystemStats {
  cpu_percent: number;
  memory_used_bytes: number;
  memory_total_bytes: number;
  timestamp: number;
}

const current = ref<SystemStats | null>(null);
const cpuHistory = ref<number[]>([]);
const memHistory = ref<number[]>([]);
const MAX_POINTS = 60;

let timer: ReturnType<typeof setInterval> | null = null;

async function fetchStats() {
  try {
    const res = await fetch(API_PATHS.SYSTEM.STATS);
    if (!res.ok) return;
    const data: SystemStats = await res.json();
    current.value = data;

    cpuHistory.value = [...cpuHistory.value, data.cpu_percent].slice(-MAX_POINTS);
    const memPct = data.memory_total_bytes > 0
      ? (data.memory_used_bytes / data.memory_total_bytes) * 100
      : 0;
    memHistory.value = [...memHistory.value, memPct].slice(-MAX_POINTS);
  } catch {
    // best-effort polling
  }
}

function formatBytes(bytes: number): string {
  if (bytes >= 1024 ** 3) return `${(bytes / 1024 ** 3).toFixed(1)} GB`;
  if (bytes >= 1024 ** 2) return `${(bytes / 1024 ** 2).toFixed(0)} MB`;
  return `${(bytes / 1024).toFixed(0)} KB`;
}

function sparklinePoints(values: number[]): string {
  if (values.length < 2) return '';
  const max = Math.max(...values, 1);
  const step = 100 / (values.length - 1);
  return values
    .map((v, i) => {
      const x = i * step;
      const y = 28 - (v / max) * 24;
      return `${x},${y}`;
    })
    .join(' ');
}

const cpuLabel = computed(() =>
  current.value ? `${current.value.cpu_percent.toFixed(1)}%` : '—'
);

const memLabel = computed(() => {
  if (!current.value) return '—';
  const { memory_used_bytes, memory_total_bytes } = current.value;
  if (memory_total_bytes === 0) return '—';
  const pct = (memory_used_bytes / memory_total_bytes) * 100;
  return `${pct.toFixed(0)}% · ${formatBytes(memory_used_bytes)}`;
});

onMounted(() => {
  fetchStats();
  timer = setInterval(fetchStats, 3000);
});

onUnmounted(() => {
  if (timer) clearInterval(timer);
});
</script>

<template>
  <div class="grid grid-cols-1 md:grid-cols-2 gap-3 sm:gap-4">
    <div class="card bg-base-100 border border-base-200 shadow-sm p-4">
      <div class="flex items-center justify-between mb-2">
        <div class="flex items-center gap-2 text-sm font-medium text-base-content/70">
          <Cpu class="w-4 h-4 text-violet-500" />
          System CPU
        </div>
        <span class="font-mono text-sm font-semibold text-base-content">{{ cpuLabel }}</span>
      </div>
      <svg viewBox="0 0 100 30" class="w-full h-10 text-violet-500" preserveAspectRatio="none">
        <polyline
          v-if="sparklinePoints(cpuHistory)"
          fill="none"
          stroke="currentColor"
          stroke-width="1.5"
          stroke-linejoin="round"
          stroke-linecap="round"
          :points="sparklinePoints(cpuHistory)"
        />
      </svg>
    </div>

    <div class="card bg-base-100 border border-base-200 shadow-sm p-4">
      <div class="flex items-center justify-between mb-2">
        <div class="flex items-center gap-2 text-sm font-medium text-base-content/70">
          <MemoryStick class="w-4 h-4 text-sky-500" />
          System Memory
        </div>
        <span class="font-mono text-sm font-semibold text-base-content">{{ memLabel }}</span>
      </div>
      <svg viewBox="0 0 100 30" class="w-full h-10 text-sky-500" preserveAspectRatio="none">
        <polyline
          v-if="sparklinePoints(memHistory)"
          fill="none"
          stroke="currentColor"
          stroke-width="1.5"
          stroke-linejoin="round"
          stroke-linecap="round"
          :points="sparklinePoints(memHistory)"
        />
      </svg>
    </div>
  </div>
</template>
