<script setup lang="ts">
import { Box, Terminal, Plus } from 'lucide-vue-next';
import { RouterLink } from 'vue-router';

defineProps<{
  premium?: boolean;
}>();
</script>

<template>
  <div class="card bg-base-100 border border-base-200 shadow-sm py-16 flex flex-col items-center text-center">
    <div class="w-16 h-16 bg-primary/10 rounded-full flex items-center justify-center mb-5 text-primary/70">
      <Box class="w-8 h-8" />
    </div>
    <h3 class="font-semibold text-lg text-base-content">No programs yet</h3>
    <p class="text-base-content/50 max-w-md mt-2 text-sm leading-relaxed">
      Super is running, but nothing is managed yet. Add your first process to start monitoring uptime, logs, and health.
    </p>

    <div v-if="premium" class="mt-8">
      <RouterLink
        to="/programs/new"
        class="btn btn-primary gap-2 shadow-sm"
      >
        <Plus class="w-4 h-4" />
        Create First Program
      </RouterLink>
    </div>

    <div v-else class="mt-8 w-full max-w-lg text-left space-y-3">
      <div class="rounded-lg bg-base-200/60 border border-base-200 px-4 py-3 font-mono text-xs text-base-content/80 flex items-start gap-3">
        <Terminal class="w-4 h-4 shrink-0 mt-0.5 text-base-content/40" />
        <code>super add ./my-app --name my-app --autostart</code>
      </div>
      <p class="text-xs text-base-content/45 text-center">
        Or define programs in <code class="font-mono">super.toml</code> / via <code class="font-mono">POST /api/programs</code>
      </p>
    </div>
  </div>
</template>
