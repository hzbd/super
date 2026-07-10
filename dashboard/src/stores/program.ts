import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { useWebSocket } from '@vueuse/core'
import apiClient from '@/api/client'
import { API_PATHS } from '@/api/paths'
import type { Program, WsMessage } from '@/types'

// Log callback type
type LogCallback = (line: string, source: 'stdout' | 'stderr') => void;

export const useProgramStore = defineStore('program', () => {
  // --- State ---
  const programs = ref<Program[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // IDs with in-flight actions (button loading)
  const operatingIds = ref<Set<string>>(new Set())

  // Log subscriptions (key: program ID, value: callback)
  const logListeners = new Map<string, LogCallback>();

  // --- WebSocket Setup ---
  // Auto-select ws/wss from page protocol
  const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
  const host = window.location.host;

  // Append token from localStorage when present
  // OSS: no token (empty query); Premium: backend auth via ?token=
  const token = localStorage.getItem('super_token');
  const query = token ? `?token=${token}` : '';

  // Production: connect to host/ws; dev: Vite proxies /ws
  const wsUrl = `${protocol}//${host}/ws${query}`;

  const { status: wsStatus } = useWebSocket(wsUrl, {
    autoReconnect: true,
    heartbeat: {
      message: 'ping',
      interval: 30000,
    },
    onMessage: (_, event) => {
      try {
        const msg: WsMessage = JSON.parse(event.data);
        handleWsMessage(msg);
      } catch (e) {
        // console.warn('WS parse error:', event.data);
      }
    }
  });

  const isConnected = computed(() => wsStatus.value === 'OPEN');

  // --- WebSocket message handling ---
  function handleWsMessage(msg: WsMessage) {
    // Status change
    if (msg.type === 'StatusChange') {
      const { id, status } = msg.payload;
      // Update matching process in place
      const target = programs.value.find(p => p.id === id);
      if (target) {
        target.status = status;
        // Clear operating flag when action completes
        if (operatingIds.value.has(id)) {
          operatingIds.value.delete(id);
        }
      } else {
        // New process: optionally refresh list
        // fetchPrograms();
      }
    }
    // Log line
    else if (msg.type === 'Log') {
      const { id, source, line } = msg.payload;
      // Deliver to subscribed component, if any
      const callback = logListeners.get(id);
      if (callback) {
        callback(line, source);
      }
    }
  }

  // --- Actions ---

  // Fetch program list
  async function fetchPrograms() {
    isLoading.value = true
    error.value = null
    try {
      const res = await apiClient.get<Program[]>(API_PATHS.PROGRAMS.LIST)
      programs.value = res.data || []
    } catch (err: any) {
      console.error('Fetch failed:', err)
      error.value = err.response?.data || err.message || 'Failed to connect to backend'
      programs.value = []
    } finally {
      isLoading.value = false
    }
  }

  // Start / stop / restart
  async function performAction(action: 'start' | 'stop' | 'restart', id: string) {
    if (operatingIds.value.has(id)) return;
    operatingIds.value.add(id)
    try {
      let url = '';
      switch (action) {
        case 'start': url = API_PATHS.PROGRAMS.START(id); break;
        case 'stop': url = API_PATHS.PROGRAMS.STOP(id); break;
        case 'restart': url = API_PATHS.PROGRAMS.RESTART(id); break;
      }
      await apiClient.post(url)
      // Rely on WebSocket for status updates; no manual refetch
    } catch (err: any) {
      console.error(`Failed to ${action}:`, err)
      alert(`Operation failed: ${err.message}`)
      operatingIds.value.delete(id) // Clear loading on failure
    }
  }

  // Remove program
  async function removeProgram(id: string) {
    if (operatingIds.value.has(id)) return;
    if (!confirm('Are you sure you want to remove this process configuration?')) return;

    operatingIds.value.add(id)
    try {
      await apiClient.delete(API_PATHS.PROGRAMS.REMOVE(id))
      // Delete does not emit status events; refresh list manually
      await fetchPrograms()
    } catch (err: any) {
      alert(`Remove failed: ${err.message}`)
    } finally {
      operatingIds.value.delete(id)
    }
  }

  // Log subscription management
  function subscribeLog(id: string, callback: LogCallback) {
    logListeners.set(id, callback);
  }

  function unsubscribeLog(id: string) {
    logListeners.delete(id);
  }

  // Reload system config
  async function reloadSystem() {
    try {
      isLoading.value = true;
      await apiClient.post(API_PATHS.SYSTEM.RELOAD);
      // Refetch list after config reload
      await fetchPrograms();
      // TODO: replace alert with toast
      alert('System configuration reloaded successfully.');
    } catch (err: any) {
      console.error('Reload failed:', err);
      alert(`Reload failed: ${err.message}`);
    } finally {
      isLoading.value = false;
    }
  }

  return {
    programs,
    isLoading,
    error,
    operatingIds,
    isConnected,
    reloadSystem,
    fetchPrograms,
    removeProgram,
    subscribeLog,
    unsubscribeLog,
    startProgram: (id: string) => performAction('start', id),
    stopProgram: (id: string) => performAction('stop', id),
    restartProgram: (id: string) => performAction('restart', id),
  }
})
