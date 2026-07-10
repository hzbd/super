<script setup lang="ts">
import { computed, ref } from 'vue';
import { useProgramStore } from '@/stores/program';
import type { ProcessStatus } from '@/types';
import { MoreHorizontal, Trash2, RotateCw, Loader2, Play, Square, AlertTriangle } from 'lucide-vue-next';

const props = defineProps<{
  id: string,
  status: ProcessStatus
}>();

const store = useProgramStore();

// Global busy state for this program
const isGlobalBusy = computed(() => store.operatingIds.has(props.id));

// Local action in progress
const currentAction = ref<'start' | 'stop' | 'restart' | 'remove' | null>(null);

// Confirmation modal state
const showModal = ref(false);
const pendingAction = ref<'start' | 'stop' | 'restart' | 'remove' | null>(null);

const isRunning = computed(() =>
  ['Running', 'Healthy', 'Starting', 'Backoff', 'Stopping', 'Waiting'].includes(props.status)
);

const canStart = computed(() => !isRunning.value && !isGlobalBusy.value);
const canStop = computed(() => isRunning.value && !isGlobalBusy.value);
const canRestart = computed(() => isRunning.value && !isGlobalBusy.value);

// Open confirmation modal
function requestConfirm(action: 'start' | 'stop' | 'restart' | 'remove') {
  pendingAction.value = action;
  showModal.value = true;

  // Blur dropdown trigger to close menu
  const elem = document.activeElement as HTMLElement;
  if (elem) elem.blur();
}

// Run confirmed action
async function executeAction() {
  if (!pendingAction.value) return;
  const action = pendingAction.value;

  showModal.value = false;
  currentAction.value = action;

  try {
    if (action === 'start') await store.startProgram(props.id);
    else if (action === 'stop') await store.stopProgram(props.id);
    else if (action === 'restart') await store.restartProgram(props.id);
    else if (action === 'remove') await store.removeProgram(props.id);
  } finally {
    currentAction.value = null;
    pendingAction.value = null;
  }
}

const modalContent = computed(() => {
  const action = pendingAction.value;
  switch (action) {
    case 'start': return { title: 'Start Process', msg: 'Are you sure you want to start this process?', color: 'sp-btn-primary', icon: Play };
    case 'stop': return { title: 'Stop Process', msg: 'Are you sure you want to stop this process?', color: 'sp-btn-danger', icon: Square };
    case 'restart': return { title: 'Restart Process', msg: 'Are you sure you want to restart this process?', color: 'sp-btn-warning', icon: RotateCw };
    case 'remove': return { title: 'Delete Configuration', msg: 'This will permanently remove the process configuration.', color: 'sp-btn-danger', icon: Trash2 };
    default: return { title: 'Confirm', msg: 'Are you sure?', color: 'sp-btn-primary', icon: AlertTriangle };
  }
});
</script>

<template>
  <div class="flex items-center justify-end gap-3 text-sm font-medium">

    <!-- Start Button -->
    <button
      class="flex items-center gap-1.5 transition-colors select-none"
      :class="canStart
        ? 'text-emerald-600 hover:text-emerald-700 hover:underline cursor-pointer'
        : 'text-base-content/20 cursor-not-allowed'"
      :disabled="!canStart"
      @click="requestConfirm('start')"
    >
      <Loader2 v-if="currentAction === 'start'" class="w-3.5 h-3.5 animate-spin" />
      <Play v-else class="w-3.5 h-3.5 fill-current opacity-80" />
      <span>Start</span>
    </button>

    <!-- Stop Button -->
    <button
      class="flex items-center gap-1.5 transition-colors select-none"
      :class="canStop
        ? 'text-rose-600 hover:text-rose-700 hover:underline cursor-pointer'
        : 'text-base-content/20 cursor-not-allowed'"
      :disabled="!canStop"
      @click="requestConfirm('stop')"
    >
      <Loader2 v-if="currentAction === 'stop'" class="w-3.5 h-3.5 animate-spin" />
      <Square v-else class="w-3.5 h-3.5 fill-current opacity-80" />
      <span>Stop</span>
    </button>

    <!-- More Dropdown -->
    <div class="dropdown dropdown-end dropdown-left">
      <div tabindex="0" role="button" class="btn btn-xs btn-square btn-ghost text-base-content/40 hover:text-base-content">
        <MoreHorizontal class="w-4 h-4" />
      </div>
      <ul tabindex="0" class="dropdown-content z-[100] menu p-1.5 shadow-xl bg-base-100 rounded-xl w-36 border border-base-200 text-sm">
        <li v-if="isRunning">
          <a @click="canRestart ? requestConfirm('restart') : null" class="py-2" :class="canRestart ? '' : 'pointer-events-none opacity-50'">
            <RotateCw class="w-4 h-4" :class="{ 'animate-spin': currentAction === 'restart' }" />
            Restart
          </a>
        </li>
        <div v-if="isRunning" class="h-px bg-base-200 my-1"></div>
        <li>
          <a @click="requestConfirm('remove')" class="text-error hover:bg-error/10 py-2">
            <Trash2 class="w-4 h-4" />
            Delete
          </a>
        </li>
      </ul>
    </div>

    <!-- Confirmation Modal (sp-btn) -->
    <dialog class="modal modal-bottom sm:modal-middle" :class="{ 'modal-open': showModal }">
      <div class="modal-box bg-base-100 text-base-content shadow-2xl">
        <h3 class="font-bold text-lg flex items-center gap-2">
          <component :is="modalContent.icon" class="w-5 h-5" />
          {{ modalContent.title }}
        </h3>
        <p class="py-4 text-base-content/70">{{ modalContent.msg }}</p>

        <!-- Custom button styles -->
        <div class="modal-action flex justify-end gap-3 mt-4">
          <button class="sp-btn sp-btn-cancel" @click="showModal = false">Cancel</button>
          <button class="sp-btn" :class="modalContent.color" @click="executeAction">Confirm</button>
        </div>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button @click="showModal = false">close</button>
      </form>
    </dialog>

  </div>
</template>
