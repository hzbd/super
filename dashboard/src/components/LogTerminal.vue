<script setup lang="ts">
import { onMounted, onBeforeUnmount, ref, watch } from 'vue';
import { Terminal } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
import 'xterm/css/xterm.css';

const props = defineProps<{
  logs: string[]; // Optional historical log lines
}>();

const terminalContainer = ref<HTMLElement | null>(null);
let term: Terminal | null = null;
let fitAddon: FitAddon | null = null;

onMounted(() => {
  if (!terminalContainer.value) return;

  // Initialize xterm
  term = new Terminal({
    cursorBlink: true,
    fontSize: 12,
    fontFamily: 'Menlo, Monaco, "Courier New", monospace',
    theme: {
      background: '#1e1e1e',
      foreground: '#d4d4d4',
    },
    disableStdin: true, // Read-only
    convertEol: true,   // Normalize line endings
  });

  fitAddon = new FitAddon();
  term.loadAddon(fitAddon);

  term.open(terminalContainer.value);
  fitAddon.fit();

  // Write initial logs
  props.logs.forEach(line => term?.writeln(line));

  // Refit on window resize
  window.addEventListener('resize', handleResize);
});

onBeforeUnmount(() => {
  window.removeEventListener('resize', handleResize);
  term?.dispose();
});

function handleResize() {
  fitAddon?.fit();
}

// Exposed API for parent components
defineExpose({
  writeLine: (line: string, source: 'stdout' | 'stderr') => {
    if (!term) return;
    // stderr in red (ANSI escape)
    if (source === 'stderr') {
      term.writeln(`\x1b[31m${line}\x1b[0m`);
    } else {
      term.writeln(line);
    }
  },
  clear: () => term?.clear(),
  fit: () => setTimeout(() => fitAddon?.fit(), 100) // Delay fit until container is laid out
});
</script>

<template>
  <div class="h-full w-full bg-[#1e1e1e] p-2 rounded-lg overflow-hidden">
    <div ref="terminalContainer" class="h-full w-full"></div>
  </div>
</template>
