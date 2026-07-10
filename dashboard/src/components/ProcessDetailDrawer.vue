<script setup lang="ts">
import { ref, watch, onUnmounted, nextTick, computed } from 'vue';
import {
  X, Terminal as TerminalIcon, FileText, Cpu,
  Copy, Check, Folder, User, PlayCircle, Layers, Activity,
  Link2, TerminalSquare, AlertTriangle, Clock
} from 'lucide-vue-next';
import { useProgramStore } from '@/stores/program';
import apiClient from '@/api/client';
import { API_PATHS } from '@/api/paths';
import LogTerminal from '@/components/LogTerminal.vue';
import StatusBadge from '@/components/StatusBadge.vue';
import ActionButtons from '@/components/ActionButtons.vue';
import type { ProgramDetail, ProgramLogsResponse } from '@/types';
import { useClipboard } from '@vueuse/core';

const props = defineProps<{
  modelValue: boolean;
  processId: string | null;
}>();

const emit = defineEmits(['update:modelValue', 'change-process']);

const store = useProgramStore();
const { copy, copied } = useClipboard();

const activeTab = ref<'logs' | 'config'>('logs');
const logMode = ref<'live' | 'history'>('live');
const historyLines = ref(200);
const historyLoading = ref(false);
const historyError = ref<string | null>(null);
const historyContent = ref('');
const terminalRef = ref<InstanceType<typeof LogTerminal> | null>(null);
const isLoadingDetail = ref(false);
const detailData = ref<ProgramDetail | null>(null);

const summaryData = computed(() =>
  store.programs.find(p => p.id === props.processId)
);

const fullCommand = computed(() => {
  if (!detailData.value) return '';
  const cfg = detailData.value.config;
  return [cfg.command, ...cfg.args].join(' ');
});

const hasHooks = computed(() => {
  const h = detailData.value?.config.hooks;
  if (!h) return false;
  return Object.values(h).some(cmd => !!cmd);
});

watch(activeTab, (newTab) => {
  if (newTab === 'logs' && logMode.value === 'live') {
    nextTick(() => terminalRef.value?.fit());
  }
});

watch(logMode, async (mode) => {
  if (mode === 'live') {
    nextTick(() => terminalRef.value?.fit());
  } else if (props.processId && props.modelValue) {
    await fetchHistory(props.processId);
  }
});

async function fetchHistory(id: string) {
  historyLoading.value = true;
  historyError.value = null;
  try {
    const res = await apiClient.get<ProgramLogsResponse>(API_PATHS.PROGRAMS.LOGS(id, historyLines.value));
    historyContent.value = res.data.logs
      .map((f) => (res.data.logs.length > 1 ? `--- ${f.source} ---\n${f.content}` : f.content))
      .join('\n') || '(no log files yet)';
  } catch {
    historyError.value = 'Failed to load historical logs';
    historyContent.value = '';
  } finally {
    historyLoading.value = false;
  }
}

watch(() => props.processId, async (newId) => {
  if (!newId) return;
  detailData.value = null;

  if (summaryData.value) {
    store.unsubscribeLog(summaryData.value.id);
  }

  if (props.modelValue) {
    terminalRef.value?.clear();
    store.subscribeLog(newId, (line, source) => {
      terminalRef.value?.writeLine(line, source);
    });

    await fetchDetail(newId);

    if (activeTab.value === 'logs') {
        nextTick(() => terminalRef.value?.fit());
    }
  }
});

watch(() => props.modelValue, (isOpen) => {
  if (!isOpen && props.processId) {
    store.unsubscribeLog(props.processId);
  } else if (isOpen && props.processId) {
    fetchDetail(props.processId);
    setTimeout(() => {
        if (activeTab.value === 'logs') terminalRef.value?.fit();
    }, 300);
  }
});

async function fetchDetail(id: string) {
  isLoadingDetail.value = true;
  try {
    const res = await apiClient.get<ProgramDetail>(API_PATHS.PROGRAMS.DETAIL(id));
    detailData.value = res.data;
  } catch (e) {
    console.error('Failed to fetch details', e);
  } finally {
    isLoadingDetail.value = false;
  }
}

function jumpToDependency(depName: string) {
  const target = store.programs.find(p => p.name === depName);
  if (target) {
    emit('change-process', target.id);
  } else {
    alert(`Dependency "${depName}" not found in current process list.`);
  }
}

onUnmounted(() => {
  if (props.processId) store.unsubscribeLog(props.processId);
});

function close() {
  emit('update:modelValue', false);
}
</script>

<template>
  <div
    v-if="modelValue"
    class="fixed inset-0 z-[100] bg-black/20 backdrop-blur-sm transition-opacity"
    @click="close"
  ></div>

  <div
    class="fixed inset-y-0 right-0 z-[101] w-full md:w-[850px] bg-base-100 shadow-2xl transform transition-transform duration-300 ease-in-out flex flex-col border-l border-base-200"
    :class="modelValue ? 'translate-x-0' : 'translate-x-full'"
  >
    <div v-if="processId" class="flex flex-col h-full">

      <div class="px-4 md:px-6 py-3 md:py-4 border-b border-base-200 flex items-center justify-between bg-base-100 shrink-0">
        <div class="flex items-center gap-3 overflow-hidden">
          <div class="w-10 h-10 bg-primary/10 rounded-xl flex items-center justify-center text-primary shadow-sm shrink-0">
            <Cpu class="w-6 h-6" />
          </div>
          <div class="min-w-0">
            <h2 class="text-lg md:text-xl font-bold tracking-tight truncate pr-2">{{ summaryData?.name || 'Loading...' }}</h2>
            <div class="flex items-center gap-2 text-xs text-base-content/50 font-mono mt-0.5">
              <span>{{ processId.slice(0, 8) }}</span>
              <span v-if="summaryData?.pid" class="px-1.5 py-0.5 bg-base-200 rounded text-base-content/70 hidden sm:inline-block">
                PID: {{ summaryData.pid }}
              </span>
            </div>
          </div>
        </div>

        <div class="flex items-center gap-2 md:gap-4 shrink-0">
          <div class="hidden md:flex items-center gap-4">
            <StatusBadge :status="summaryData?.status || 'Stopped'" />
            <div class="w-px h-6 bg-base-300"></div>
            <ActionButtons :id="processId" :status="summaryData?.status || 'Stopped'" />
            <div class="w-px h-6 bg-base-300"></div>
          </div>
          <button class="btn btn-sm btn-circle btn-ghost hover:bg-base-200" @click="close">
            <X class="w-5 h-5" />
          </button>
        </div>
      </div>

      <div class="md:hidden px-4 py-3 border-b border-base-200 bg-base-50/50 flex items-center justify-between shrink-0">
        <StatusBadge :status="summaryData?.status || 'Stopped'" />
        <ActionButtons :id="processId" :status="summaryData?.status || 'Stopped'" />
      </div>

      <!-- Error banner -->
      <div v-if="(detailData as any)?.last_error || (detailData as any)?.lastError" class="px-6 py-3 bg-rose-50 border-b border-rose-100 flex items-start gap-3 shrink-0">
        <div class="text-rose-600 mt-0.5">
          <AlertTriangle class="w-4 h-4" />
        </div>
        <div class="flex-1">
          <h3 class="text-sm font-bold text-rose-700">Launch Failed</h3>
          <p class="text-xs text-rose-600 mt-1 font-mono break-all">
            {{ (detailData as any).last_error || (detailData as any).lastError }}
          </p>
        </div>
      </div>

      <div class="px-4 md:px-6 border-b border-base-200 bg-base-50/50 shrink-0">
        <div class="flex gap-6 md:gap-8">
          <button
            class="flex items-center gap-2 py-3 text-sm font-medium border-b-2 transition-all"
            :class="activeTab === 'logs' ? 'border-primary text-primary' : 'border-transparent text-base-content/60 hover:text-base-content'"
            @click="activeTab = 'logs'"
          >
            <TerminalIcon class="w-4 h-4" />
            Live Logs
          </button>
          <button
            class="flex items-center gap-2 py-3 text-sm font-medium border-b-2 transition-all"
            :class="activeTab === 'config' ? 'border-primary text-primary' : 'border-transparent text-base-content/60 hover:text-base-content'"
            @click="activeTab = 'config'"
          >
            <FileText class="w-4 h-4" />
            Configuration
          </button>
        </div>
      </div>

      <div class="flex-1 overflow-hidden relative bg-base-50">
        <!-- Logs -->
        <div v-show="activeTab === 'logs'" class="absolute inset-0 flex flex-col">
          <div class="flex items-center justify-between gap-2 px-4 py-2 border-b border-base-200 bg-base-100 shrink-0">
            <div class="flex gap-1">
              <button class="btn btn-xs" :class="logMode === 'live' ? 'btn-primary' : 'btn-ghost'" @click="logMode = 'live'">Live</button>
              <button class="btn btn-xs" :class="logMode === 'history' ? 'btn-primary' : 'btn-ghost'" @click="logMode = 'history'">History</button>
            </div>
            <div v-if="logMode === 'history'" class="flex items-center gap-2">
              <select v-model.number="historyLines" class="select select-xs select-bordered" @change="processId && fetchHistory(processId)">
                <option :value="100">100</option>
                <option :value="200">200</option>
                <option :value="500">500</option>
              </select>
              <button class="btn btn-xs btn-ghost" :disabled="historyLoading || !processId" @click="processId && fetchHistory(processId)">Refresh</button>
            </div>
          </div>
          <div v-if="logMode === 'live'" class="flex-1 p-0 bg-[#1e1e1e] min-h-0">
            <LogTerminal ref="terminalRef" :logs="[]" />
          </div>
          <div v-else class="flex-1 min-h-0 overflow-auto bg-[#1e1e1e] p-4">
            <div v-if="historyLoading" class="text-gray-400 text-sm">Loading...</div>
            <div v-else-if="historyError" class="text-rose-400 text-sm">{{ historyError }}</div>
            <pre v-else class="text-xs text-gray-200 font-mono whitespace-pre-wrap break-words">{{ historyContent }}</pre>
          </div>
        </div>

        <!-- Config -->
        <div v-show="activeTab === 'config'" class="absolute inset-0 overflow-y-auto p-4 md:p-6 lg:p-8">
          <div v-if="isLoadingDetail && !detailData" class="flex justify-center py-10">
            <span class="loading loading-spinner loading-md"></span>
          </div>

          <div v-else-if="detailData" class="flex flex-col gap-6 md:gap-8 max-w-4xl pb-10">
            <!-- Context -->
            <section>
              <h3 class="text-xs font-bold text-base-content/40 uppercase tracking-wider mb-3 flex items-center gap-2">
                <PlayCircle class="w-4 h-4" /> Execution Context
              </h3>
              <div class="bg-base-100 border border-base-200 rounded-xl shadow-sm overflow-hidden">
                <div class="p-4 border-b border-base-200">
                  <div class="text-xs font-medium text-base-content/50 mb-1">Command</div>
                  <div class="flex items-center justify-between gap-4 bg-base-200/50 p-3 rounded-lg group border border-base-200/50">
                    <code class="font-mono text-sm text-base-content/80 break-all leading-relaxed">{{ fullCommand }}</code>
                    <button
                      class="btn btn-xs btn-square btn-ghost opacity-0 group-hover:opacity-100 transition-opacity"
                      @click="copy(fullCommand)"
                      title="Copy Command"
                    >
                      <Check v-if="copied" class="w-3.5 h-3.5 text-success" />
                      <Copy v-else class="w-3.5 h-3.5" />
                    </button>
                  </div>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-2 divide-y md:divide-y-0 md:divide-x divide-base-200">
                  <div class="p-4">
                    <div class="flex items-center gap-2 text-xs font-medium text-base-content/50 mb-1">
                      <Folder class="w-3.5 h-3.5" /> Working Directory
                    </div>
                    <div class="font-mono text-sm">{{ detailData.config.cwd || '(default)' }}</div>
                  </div>
                  <div class="p-4">
                    <div class="flex items-center gap-2 text-xs font-medium text-base-content/50 mb-1">
                      <User class="w-3.5 h-3.5" /> User / Group
                    </div>
                    <div class="font-mono text-sm">
                      {{ detailData.config.user || 'root' }}
                      <span v-if="detailData.config.group" class="text-base-content/40">/ {{ detailData.config.group }}</span>
                    </div>
                  </div>
                </div>
              </div>
            </section>

            <!-- Env -->
            <section v-if="detailData.config.env && Object.keys(detailData.config.env).length > 0">
              <h3 class="text-xs font-bold text-base-content/40 uppercase tracking-wider mb-3 flex items-center gap-2">
                <Layers class="w-4 h-4" /> Environment Variables
              </h3>
              <div class="bg-base-100 border border-base-200 rounded-xl shadow-sm overflow-hidden">
                <div class="overflow-x-auto">
                  <table class="table table-sm w-full">
                    <thead class="bg-base-200/50 text-base-content/60">
                      <tr>
                        <th class="w-1/3 pl-4">Key</th>
                        <th class="pl-4">Value</th>
                      </tr>
                    </thead>
                    <tbody class="font-mono text-xs">
                      <tr v-for="(val, key) in detailData.config.env" :key="key" class="border-b border-base-100 last:border-0 hover:bg-base-50">
                        <td class="pl-4 font-semibold text-primary/80">{{ key }}</td>
                        <td class="pl-4 text-base-content/70 break-all py-2">{{ val }}</td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>
            </section>

            <!-- Lifecycle & Advanced -->
            <section class="grid grid-cols-1 md:grid-cols-2 gap-6 items-start">
              <div>
                <h3 class="text-xs font-bold text-base-content/40 uppercase tracking-wider mb-3 flex items-center gap-2">
                  <Activity class="w-4 h-4" /> Lifecycle
                </h3>
                <div class="bg-base-100 border border-base-200 rounded-xl shadow-sm overflow-hidden">
                  <div class="p-4 space-y-4">
                    <div class="flex justify-between items-center">
                      <span class="text-sm text-base-content/70">Autostart</span>
                      <span class="badge badge-sm" :class="detailData.config.autostart ? 'badge-success badge-outline' : 'badge-ghost'">
                        {{ detailData.config.autostart ? 'Enabled' : 'Disabled' }}
                      </span>
                    </div>
                    <div class="flex justify-between items-center">
                      <span class="text-sm text-base-content/70">Retry Limit</span>
                      <span class="font-mono text-sm font-bold">{{ detailData.config.retry_limit }} times</span>
                    </div>
                    <div class="flex justify-between items-center">
                      <span class="text-sm text-base-content/70">Autorestart</span>
                      <span class="font-mono text-sm">{{ detailData.config.autorestart || 'unexpected' }}</span>
                    </div>
                    <!-- Cron -->
                    <div v-if="(detailData.config as any).cron" class="pt-4 border-t border-base-200">
                      <div class="text-xs font-bold text-base-content/40 uppercase tracking-wider mb-2 flex items-center gap-1.5">
                        <Clock class="w-3.5 h-3.5" /> Cron Schedule
                      </div>
                      <div class="bg-primary/5 text-primary border border-primary/20 rounded px-2 py-1.5 font-mono text-xs inline-block">
                        {{ (detailData.config as any).cron }}
                      </div>
                    </div>
                  </div>
                  <!-- Depends On -->
                  <div v-if="detailData.config.depends_on.length > 0" class="border-t border-base-200 bg-base-50/50 p-4">
                    <div class="text-xs font-bold text-base-content/40 uppercase tracking-wider mb-2 flex items-center gap-1.5">
                      <Link2 class="w-3 h-3" /> Depends On
                    </div>
                    <div class="flex flex-wrap gap-2">
                      <span v-for="dep in detailData.config.depends_on" :key="dep" class="badge badge-neutral gap-1 cursor-pointer hover:bg-neutral-focus hover:text-neutral-content transition-all border-base-300" @click="jumpToDependency(dep)">
                        {{ dep }}
                      </span>
                    </div>
                  </div>
                </div>
              </div>

              <div v-if="hasHooks || detailData.config.health_check">
                <h3 class="text-xs font-bold text-base-content/40 uppercase tracking-wider mb-3 flex items-center gap-2">
                  <TerminalSquare class="w-4 h-4" /> Advanced
                </h3>
                <div class="flex flex-col gap-4">
                  <div v-if="detailData.config.health_check" class="bg-base-100 border border-base-200 rounded-xl shadow-sm p-4">
                    <div class="flex items-center gap-2 mb-2">
                      <span class="text-sm font-semibold">Health Check</span>
                    </div>
                    <div class="bg-base-200/50 rounded-lg p-3 font-mono text-xs text-base-content border border-base-200 break-all">
                      <div v-if="detailData.config.health_check.type === 'tcp'">
                        <span class="badge badge-xs badge-success badge-outline font-bold mr-2">TCP</span>
                        {{ detailData.config.health_check.host }}:{{ detailData.config.health_check.port }}
                      </div>
                      <div v-else-if="detailData.config.health_check.type === 'http'">
                        <div class="flex items-center gap-2 mb-1">
                          <span class="badge badge-xs badge-info badge-outline font-bold">HTTP</span>
                          <span class="font-bold uppercase">{{ detailData.config.health_check.method || 'GET' }}</span>
                        </div>
                        <div class="opacity-80">{{ detailData.config.health_check.url }}</div>
                      </div>
                      <div v-else-if="detailData.config.health_check.type === 'exec'">
                        <span class="badge badge-xs badge-warning badge-outline w-fit font-bold mb-1">EXEC</span>
                        <div class="pl-2 border-l-2 border-base-300">$ {{ detailData.config.health_check.command }}</div>
                      </div>
                    </div>
                  </div>

                  <div v-if="hasHooks" class="bg-base-100 border border-base-200 rounded-xl shadow-sm p-4">
                    <div class="flex items-center gap-2 mb-3">
                      <span class="text-sm font-semibold">Hooks</span>
                    </div>
                    <div class="space-y-3">
                        <div v-for="(cmd, key) in detailData.config.hooks" :key="key">
                            <div v-if="cmd" class="flex flex-col gap-1">
                                <span class="badge badge-sm badge-ghost border border-base-300 font-mono text-xs w-fit uppercase">{{ key.toString().replace('_', '-') }}</span>
                                <div class="bg-base-200/40 rounded p-2 font-mono text-xs text-base-content/80 border border-base-200 break-all">
                                    {{ cmd }}
                                </div>
                            </div>
                        </div>
                    </div>
                  </div>
                </div>
              </div>
            </section>

          </div>
        </div>
      </div>
    </div>
  </div>
</template>
