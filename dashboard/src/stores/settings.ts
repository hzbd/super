import { defineStore } from 'pinia';
import { useStorage } from '@vueuse/core';

export const useSettingsStore = defineStore('settings', () => {
  // Persist page size via useStorage
  // arg1: localStorage key
  // arg2: default value (10)
  const defaultPageSize = useStorage<number>('super-pref-page-size', 10);

  return {
    defaultPageSize,
  };
});
