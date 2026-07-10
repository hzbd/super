<script setup lang="ts">
import { onMounted, ref, computed, watch } from 'vue';
import { useProgramStore } from '@/stores/program';
import { useSettingsStore } from '@/stores/settings';
import StatusBadge from '@/components/StatusBadge.vue';
import ActionButtons from '@/components/ActionButtons.vue';
import BasePagination from '@/components/BasePagination.vue';
import StatCard from '@/components/StatCard.vue';
import ProcessDetailDrawer from '@/components/ProcessDetailDrawer.vue';
import EmptyProgramsState from '@/components/EmptyProgramsState.vue';
import SystemMetricsPanel from '@/components/SystemMetricsPanel.vue';
import {
  RefreshCw, AlertCircle, Box, Search,
  Activity, PlayCircle, StopCircle, AlertTriangle, Folder, AlertOctagon
} from 'lucide-vue-next';
import { format } from 'date-fns';

const store = useProgramStore();
const settingsStore = useSettingsStore();

const searchQuery = ref('');
const currentPage = ref(1);
// const pageSize = ref(10);

const activeFilter = ref<string>('ALL');
const selectedGroup = ref<string>('');

const showDrawer = ref(false);
const selectedProcessId = ref<string | null>(null);

const uniqueGroups = computed(() => {
  const groups = new Set<string>();
  store.programs.forEach(p => {
    if (p.group) groups.add(p.group);
  });
  return Array.from(groups).sort();
});

const stats = computed(() => {
  const all = store.programs;
  return {
    total: all.length,
    running: all.filter(p => ['Running', 'Healthy', 'Starting'].includes(p.status)).length,
    stopped: all.filter(p => ['Stopped', 'Stopping'].includes(p.status)).length,
    problem: all.filter(p => ['Fatal', 'Backoff'].includes(p.status)).length,
  };
});

const filteredPrograms = computed(() => {
  let result = store.programs;

  if (activeFilter.value !== 'ALL') {
    if (activeFilter.value === 'Running') {
      result = result.filter(p => ['Running', 'Healthy', 'Starting'].includes(p.status));
    } else if (activeFilter.value === 'Stopped') {
      result = result.filter(p => ['Stopped', 'Stopping'].includes(p.status));
    } else if (activeFilter.value === 'Fatal') {
      result = result.filter(p => ['Fatal', 'Backoff'].includes(p.status));
    }
  }

  if (selectedGroup.value) {
    result = result.filter(p => p.group === selectedGroup.value);
  }

  const query = searchQuery.value.toLowerCase().trim();
  if (query) {
    result = result.filter(p =>
      p.name.toLowerCase().includes(query) ||
      p.id.toLowerCase().includes(query) ||
      (p.group && p.group.toLowerCase().includes(query))
    );
  }
  return result;
});

// const paginatedPrograms = computed(() => {
//   const start = (currentPage.value - 1) * pageSize.value;
//   const end = start + pageSize.value;
//   return filteredPrograms.value.slice(start, end);
// });

const paginatedPrograms = computed(() => {
  const start = (currentPage.value - 1) * settingsStore.defaultPageSize;
  const end = start + settingsStore.defaultPageSize;
  return filteredPrograms.value.slice(start, end);
});

// Reset to page 1 when filters or page size change
watch([searchQuery, () => settingsStore.defaultPageSize, activeFilter, selectedGroup], () => {
  currentPage.value = 1;
});

onMounted(() => { store.fetchPrograms(); });

function openDetails(id: string) {
  selectedProcessId.value = id;
  showDrawer.value = true;
}

function setFilter(status: string) {
  activeFilter.value = activeFilter.value === status ? 'ALL' : status;
}

function toggleGroupFilter(group: string) {
  selectedGroup.value = selectedGroup.value === group ? '' : group;
}

function formatUptime(sec?: number) {
  if (sec === undefined || sec === null) return '-';
  if (sec < 60) return `${sec}s`;
  const min = Math.floor(sec / 60);
  if (min < 60) return `${min}m ${sec % 60}s`;
  const hr = Math.floor(min / 60);
  if (hr < 24) return `${hr}h ${min % 60}m`;
  return `${Math.floor(hr / 24)}d ${hr % 24}h`;
}

function formatTime(timestamp: number) {
  if (!timestamp) return '-';
  return format(new Date(timestamp * 1000), 'yyyy-MM-dd HH:mm');
}

const hasActiveFilters = computed(() =>
  activeFilter.value !== 'ALL' || !!selectedGroup.value || !!searchQuery.value.trim()
);
</script>

<template>
  <div class="flex flex-col gap-8 pb-10">

    <SystemMetricsPanel />

    <!-- Stats Cards -->
    <div class="grid grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4 shrink-0">
      <StatCard
        title="Total"
        :value="stats.total"
        :icon="Activity"
        color-class="text-blue-600 bg-blue-50"
        :is-active="activeFilter === 'ALL'"
        @click="setFilter('ALL')"
      />
      <StatCard
        title="Running"
        :value="stats.running"
        :icon="PlayCircle"
        color-class="text-emerald-600 bg-emerald-50"
        :is-active="activeFilter === 'Running'"
        @click="setFilter('Running')"
      />
      <StatCard
        title="Stopped"
        :value="stats.stopped"
        :icon="StopCircle"
        color-class="text-base-content/60 bg-base-200"
        :is-active="activeFilter === 'Stopped'"
        @click="setFilter('Stopped')"
      />
      <StatCard
        title="Issues"
        :value="stats.problem"
        :icon="AlertTriangle"
        color-class="text-rose-600 bg-rose-50"
        :is-active="activeFilter === 'Fatal'"
        @click="setFilter('Fatal')"
      />
    </div>

    <!-- Error Banner -->
    <div v-if="store.error" class="alert alert-error shadow-sm rounded-xl">
      <AlertCircle class="w-5 h-5 stroke-current" />
      <span class="text-sm font-medium">{{ store.error }}</span>
      <button class="btn btn-xs btn-ghost" @click="store.fetchPrograms()">Retry</button>
    </div>

    <!-- Empty: no programs configured -->
    <EmptyProgramsState v-if="!store.isLoading && store.programs.length === 0" />

    <!-- List Card -->
    <div v-else class="bg-base-100 rounded-xl border border-base-200 shadow-sm overflow-hidden flex flex-col">

      <!-- Toolbar -->
      <div class="px-6 py-4 border-b border-base-200 flex flex-col xl:flex-row justify-between items-center gap-4 bg-base-100">
        <div class="flex items-center gap-3 w-full xl:w-auto">
          <h2 class="font-semibold text-base-content whitespace-nowrap">
            {{ activeFilter === 'ALL' ? 'All Processes' : `${activeFilter} Processes` }}
          </h2>
          <span class="badge badge-sm badge-neutral text-xs font-mono">{{ filteredPrograms.length }}</span>
          <div v-if="selectedGroup" class="badge badge-primary gap-1 cursor-pointer" @click="selectedGroup = ''">
            Group: {{ selectedGroup }}
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-3 h-3 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
          </div>
        </div>

        <div class="flex flex-col sm:flex-row items-center gap-3 w-full xl:w-auto">
          <div class="relative w-full sm:w-48">
            <Folder class="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-base-content/40" />
            <select
              v-model="selectedGroup"
              class="select select-sm select-bordered w-full pl-9 rounded-lg focus:select-primary font-normal"
            >
              <option value="">All Groups</option>
              <option v-for="group in uniqueGroups" :key="group" :value="group">{{ group }}</option>
            </select>
          </div>
          <div class="relative w-full sm:w-64">
            <Search class="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-base-content/40" />
            <input
              v-model="searchQuery"
              type="text"
              placeholder="Search..."
              class="input input-sm input-bordered w-full pl-9 focus:input-primary rounded-lg transition-all"
            />
          </div>
          <button class="btn btn-sm btn-outline border-base-300 font-normal gap-2 rounded-lg" @click="store.fetchPrograms()" :disabled="store.isLoading">
            <RefreshCw class="w-3.5 h-3.5" :class="{ 'animate-spin': store.isLoading }" />
            <span class="hidden sm:inline">Refresh</span>
          </button>
        </div>
      </div>

      <!-- Table Area -->
      <div class="overflow-x-auto w-full">
        <table class="table w-full whitespace-nowrap">
          <thead class="text-xs uppercase text-base-content/60 font-medium">
            <tr>
              <th class="sticky top-0 bg-base-100 z-20 border-b border-base-200 pl-6 py-4 shadow-sm">Name / ID</th>
              <th class="sticky top-0 bg-base-100 z-20 border-b border-base-200 py-4 shadow-sm">Group</th>
              <th class="sticky top-0 bg-base-100 z-20 border-b border-base-200 py-4 shadow-sm">Status</th>
              <th class="sticky top-0 bg-base-100 z-20 border-b border-base-200 py-4 shadow-sm">PID</th>
              <th class="sticky top-0 bg-base-100 z-20 border-b border-base-200 py-4 shadow-sm">Uptime</th>
              <th class="sticky top-0 bg-base-100 z-20 border-b border-base-200 py-4 shadow-sm">Updated</th>
              <th class="sticky top-0 bg-base-100 z-20 border-b border-base-200 pr-6 py-4 text-center w-[240px] shadow-sm">Actions</th>
            </tr>
          </thead>

          <tbody class="text-sm">
            <tr v-if="store.isLoading && store.programs.length === 0">
              <td colspan="7" class="h-64 text-center">
                <span class="loading loading-spinner loading-md text-primary/50"></span>
              </td>
            </tr>

            <tr v-else-if="filteredPrograms.length === 0">
              <td colspan="7" class="h-64 text-center">
                <div class="flex flex-col items-center justify-center gap-3 text-base-content/30">
                  <Box class="w-10 h-10" />
                  <p class="font-medium">No processes match your filters</p>
                  <p class="text-xs" v-if="hasActiveFilters">Try clearing filters or search.</p>
                </div>
              </td>
            </tr>

            <tr v-else v-for="proc in paginatedPrograms" :key="proc.id" class="group border-b border-base-100 hover:bg-base-50 transition-colors">
              <td class="pl-6 py-3">
                <div class="flex flex-col">
                  <span
                    class="font-semibold text-base-content/90 hover:text-primary cursor-pointer transition-colors"
                    @click="openDetails(proc.id)"
                  >
                    {{ proc.name }}
                  </span>
                  <span class="font-mono text-xs text-base-content/40 select-all">{{ proc.id.slice(0, 8) }}</span>
                </div>
              </td>

              <td class="py-3">
                <div
                  v-if="proc.group"
                  class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-base-200 text-base-content/70 hover:bg-primary/10 hover:text-primary cursor-pointer transition-colors"
                  :class="{ 'bg-primary text-primary-content hover:bg-primary hover:text-primary-content': selectedGroup === proc.group }"
                  @click="toggleGroupFilter(proc.group)"
                  title="Filter by this group"
                >
                  {{ proc.group }}
                </div>
                <span v-else class="text-base-content/20">-</span>
              </td>

              <!-- Error indicator icon -->
              <td class="py-3">
                <div class="flex items-center gap-2">
                  <StatusBadge :status="proc.status" />

                  <!-- Error Icon & Tooltip -->
                  <div
                    v-if="(proc as any).last_error || (proc as any).lastError"
                    class="group/err relative flex items-center"
                  >
                    <div class="text-rose-500 cursor-help animate-pulse">
                      <AlertOctagon class="w-4 h-4" />
                    </div>

                    <!-- Tooltip positioning -->
                    <!-- bottom-full mb-2: show above icon with spacing -->
                    <!-- left-1/2 -translate-x-1/2: center horizontally -->
                    <!-- max-w-[260px]: cap width -->
                    <!-- whitespace-normal break-words: allow wrapping -->
                    <div class="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 w-max max-w-[260px] p-2.5 bg-rose-600 text-white text-xs rounded-lg shadow-xl opacity-0 group-hover/err:opacity-100 transition-opacity pointer-events-none z-50 whitespace-normal break-words leading-tight text-center">
                      {{ (proc as any).last_error || (proc as any).lastError }}

                      <!-- Arrow pointing down -->
                      <div class="absolute top-full left-1/2 -translate-x-1/2 border-4 border-transparent border-t-rose-600"></div>
                    </div>
                  </div>
                </div>
              </td>

              <td class="py-3 font-mono text-xs text-base-content/60">{{ proc.pid || '-' }}</td>
              <td class="py-3 font-mono text-xs text-base-content/60">{{ formatUptime(proc.uptime_sec) }}</td>
              <td class="py-3 text-xs text-base-content/60 font-mono tabular-nums">{{ formatTime(proc.updated_at) }}</td>

              <td class="pr-6 py-3">
                <div class="flex justify-center">
                  <ActionButtons :id="proc.id" :status="proc.status" />
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <BasePagination
        v-model:page="currentPage"
        v-model:pageSize="settingsStore.defaultPageSize"
        :total="filteredPrograms.length"
        :disabled="store.isLoading"
      />
    </div>

    <ProcessDetailDrawer
      v-model="showDrawer"
      :process-id="selectedProcessId"
      @change-process="(id) => selectedProcessId = id"
    />
  </div>
</template>
