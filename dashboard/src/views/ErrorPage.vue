<script setup lang="ts">
import { computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import {
  AlertTriangle, ShieldAlert, ServerCrash, Ghost,
  Home, RefreshCw, PlugZap
} from 'lucide-vue-next';

const route = useRoute();
const router = useRouter();

// Error code from route, default 404
const code = computed(() => route.params.code?.toString() || '404');

// Per-code title, message, icon, and color
const config = computed(() => {
  switch (code.value) {
    case '403':
      return {
        title: 'Access Denied',
        msg: "You don't have the necessary permissions to view this resource.",
        icon: ShieldAlert,
        color: 'text-warning'
      };
    case '500':
      return {
        title: 'Server Error',
        msg: "Something went wrong on our end. We're working on it.",
        icon: ServerCrash,
        color: 'text-error'
      };
    case '503':
      return {
        title: 'Service Unavailable',
        msg: "The service is temporarily unavailable. Please check the backend connection.",
        icon: PlugZap,
        color: 'text-error'
      };
    case '404':
    default:
      return {
        title: 'Page Not Found',
        msg: "Sorry, we couldn't find the page you're looking for. It might have been moved or doesn't exist.",
        icon: Ghost, // AlertTriangle also works
        color: 'text-primary'
      };
  }
});

function handleAction() {
  // 500/503: reload; otherwise go home
  if (code.value === '500' || code.value === '503') {
    window.location.reload();
  } else {
    router.push('/');
  }
}
</script>

<template>
  <div class="min-h-screen bg-base-200 flex items-center justify-center p-4">
    <div class="text-center space-y-6 max-w-md">

      <!-- Icon Area -->
      <div class="relative inline-block">
        <div class="w-24 h-24 bg-base-100 rounded-full flex items-center justify-center shadow-lg mx-auto">
          <!-- Dynamic icon and color -->
          <component :is="config.icon" class="w-12 h-12" :class="config.color" />
        </div>

        <!-- Decorative dots -->
        <div class="absolute -top-2 -right-2 w-4 h-4 bg-primary rounded-full animate-bounce"></div>
        <div class="absolute bottom-0 -left-2 w-3 h-3 bg-secondary rounded-full animate-pulse"></div>
      </div>

      <!-- Text Content -->
      <div class="space-y-2">
        <!-- Error code -->
        <h1 class="text-6xl font-black text-base-content tracking-tighter">{{ code }}</h1>
        <!-- Title -->
        <h2 class="text-2xl font-bold text-base-content/80">{{ config.title }}</h2>
        <!-- Description -->
        <p class="text-base-content/60">
          {{ config.msg }}
        </p>
      </div>

      <!-- Action Button -->
      <button
        class="btn btn-primary px-8 gap-2 shadow-lg shadow-primary/20 rounded-full"
        @click="handleAction"
      >
        <component :is="['500', '503'].includes(code) ? RefreshCw : Home" class="w-4 h-4" />
        {{ ['500', '503'].includes(code) ? 'Reload Page' : 'Back to Dashboard' }}
      </button>

    </div>
  </div>
</template>
