<script setup lang="ts">
import { Scale, Bug } from 'lucide-vue-next';

const appVersion = (window as any).__SUPER_CONFIG__?.version || 'dev';
</script>

<template>
  <footer class="shrink-0 py-6 flex flex-wrap justify-center items-center gap-x-3 gap-y-2 text-xs font-medium text-base-content/40 z-40">

    <!-- Copyright -->
    <span class="cursor-default">&copy; 2026 SPT.</span>

    <span class="w-1 h-1 rounded-full bg-base-300"></span>

    <!-- License (MIT) -->
    <a
      href="https://opensource.org/licenses/MIT"
      target="_blank"
      class="hover:text-primary transition-colors flex items-center gap-1"
      title="View License"
    >
      <Scale class="w-3 h-3" />
      <span>MIT</span>
    </a>

    <span class="w-1 h-1 rounded-full bg-base-300"></span>

    <!-- Issues -->
    <a
      href="https://github.com/hzbd/super/issues"
      target="_blank"
      class="flex items-center gap-1 hover:text-warning transition-colors group"
    >
      <Bug class="w-3.5 h-3.5 group-hover:text-orange-500 transition-colors" />
      <span>Report an Issue</span>
    </a>

    <span class="w-1 h-1 rounded-full bg-base-300"></span>

    <!-- Version -->
    <span class="font-mono opacity-80 cursor-help" title="Current Version">
      v{{ appVersion }}
    </span>

  </footer>
</template>
