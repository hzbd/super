<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  title: string;
  value: number;
  icon: any;
  colorClass: string;
  isActive?: boolean;
}>();

defineEmits(['click']);

// Decorative background derived from colorClass (fixes TS2532)
const decorativeBgClass = computed(() => {
  if (!props.colorClass) return '';
  const firstClass = props.colorClass.split(' ')[0];
  // Guard empty split result
  return firstClass ? firstClass.replace('text-', 'bg-') : '';
});
</script>

<template>
  <div
    class="bg-base-100 border rounded-xl p-5 cursor-pointer transition-all duration-200 group relative overflow-hidden"
    :class="[
      isActive
        ? 'border-primary ring-1 ring-primary shadow-md'
        : 'border-base-200 hover:border-base-300 hover:shadow-sm'
    ]"
    @click="$emit('click')"
  >
    <div class="flex justify-between items-start">
      <div>
        <p class="text-sm font-medium text-base-content/60">{{ title }}</p>
        <h3 class="text-3xl font-bold mt-2 tracking-tight">{{ value }}</h3>
      </div>

      <div
        class="p-3 rounded-lg transition-colors"
        :class="colorClass"
      >
        <component :is="icon" class="w-6 h-6" />
      </div>
    </div>

    <!-- Decorative background via computed class -->
    <div
      class="absolute -right-4 -bottom-4 w-24 h-24 rounded-full opacity-5 pointer-events-none transition-transform group-hover:scale-110"
      :class="decorativeBgClass"
    ></div>
  </div>
</template>
