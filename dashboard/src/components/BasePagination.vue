<script setup lang="ts">
import { computed } from 'vue';
import { ChevronLeft, ChevronRight } from 'lucide-vue-next';

const props = withDefaults(defineProps<{
  total: number;
  page?: number;
  pageSize?: number;
  pageSizeOptions?: number[];
  disabled?: boolean;
}>(), {
  page: 1,
  pageSize: 10,
  pageSizeOptions: () => [10, 20, 50, 100],
  disabled: false
});

const emit = defineEmits<{
  (e: 'update:page', value: number): void;
  (e: 'update:pageSize', value: number): void;
}>();

// Page-size options shown in the dropdown
// Always include current pageSize so the select never appears blank
const displayOptions = computed(() => {
  const opts = new Set(props.pageSizeOptions);
  if (props.pageSize) {
    opts.add(props.pageSize);
  }
  // Sort ascending
  return Array.from(opts).sort((a, b) => a - b);
});

const totalPages = computed(() => Math.ceil(props.total / props.pageSize) || 1);
const start = computed(() => (props.total === 0 ? 0 : (props.page - 1) * props.pageSize + 1));
const end = computed(() => Math.min(start.value + props.pageSize - 1, props.total));

function changePage(newPage: number) {
  if (newPage >= 1 && newPage <= totalPages.value && !props.disabled) {
    emit('update:page', newPage);
  }
}
</script>

<template>
  <!-- flex-col on mobile, sm:flex-row on larger screens -->
  <div class="flex flex-col sm:flex-row items-center justify-between px-4 sm:px-6 py-4 border-t border-base-200 bg-base-50/50 text-sm gap-4">

    <!-- Left: Info Text -->
    <!-- Mobile: below controls (order-2); desktop: left (order-1) -->
    <div class="text-base-content/50 font-medium order-2 sm:order-1 text-center sm:text-left w-full sm:w-auto">
      Showing <span class="text-base-content font-bold">{{ start }}-{{ end }}</span> of <span class="text-base-content font-bold">{{ total }}</span>
    </div>

    <!-- Right: Controls -->
    <!-- Mobile: above info (order-1) for easier access -->
    <div class="flex items-center justify-between sm:justify-end w-full sm:w-auto gap-4 order-1 sm:order-2">

      <!-- Page Size Selector -->
      <div class="flex items-center gap-2">
        <span class="text-base-content/50 whitespace-nowrap">Rows:</span>
        <select
          :value="pageSize"
          @change="e => { emit('update:pageSize', Number((e.target as any).value)); emit('update:page', 1); }"
          :disabled="disabled"
          class="select select-bordered select-xs w-16 h-8 min-h-0 rounded bg-base-100 border-base-300 focus:outline-none focus:border-primary font-medium"
        >
          <option v-for="opt in displayOptions" :key="opt" :value="opt">{{ opt }}</option>
        </select>
      </div>

      <!-- Pagination Buttons -->
      <div class="flex items-center gap-1">
        <button
          class="btn btn-xs h-8 w-8 min-h-0 p-0 rounded-md border border-base-300 bg-base-100 hover:bg-base-200 hover:border-base-400 disabled:bg-transparent disabled:border-base-200 disabled:opacity-50"
          :disabled="page === 1 || disabled"
          @click="changePage(page - 1)"
        >
          <ChevronLeft class="w-4 h-4 text-base-content/70" />
        </button>

        <div class="px-2 text-base-content/70 font-medium min-w-[60px] text-center select-none">
          Page <span class="text-base-content font-bold">{{ page }}</span>
        </div>

        <button
          class="btn btn-xs h-8 w-8 min-h-0 p-0 rounded-md border border-base-300 bg-base-100 hover:bg-base-200 hover:border-base-400 disabled:bg-transparent disabled:border-base-200 disabled:opacity-50"
          :disabled="page === totalPages || disabled"
          @click="changePage(page + 1)"
        >
          <ChevronRight class="w-4 h-4 text-base-content/70" />
        </button>
      </div>

    </div>
  </div>
</template>
