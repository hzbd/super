<script setup lang="ts">
import { RouterView, RouterLink } from 'vue-router';
import {
  LayoutDashboard, Server, Settings, RefreshCw, ChevronDown,
  Menu, X, Sun, Moon, AlertTriangle
} from 'lucide-vue-next';
import { useProgramStore } from '@/stores/program';
import { ref } from 'vue';
import { useDark, useToggle } from '@vueuse/core';

// Extension slot imports
import NavbarRight from '@extensions/NavbarRight.vue';
import AppFooter from '@extensions/AppFooter.vue';

const store = useProgramStore();
const mobileMenuOpen = ref(false);
const showReloadModal = ref(false); // Modal visibility

const isDark = useDark({
  selector: 'html',
  attribute: 'data-theme',
  valueDark: 'dark',
  valueLight: 'light',
});
const toggleDark = useToggle(isDark);

function toggleMobileMenu() {
  mobileMenuOpen.value = !mobileMenuOpen.value;
}

// Open reload confirmation
function requestReload() {
  showReloadModal.value = true;
  mobileMenuOpen.value = false;
  const elem = document.activeElement as HTMLElement;
  if (elem) elem.blur();
}

// Confirm and reload config
function confirmReload() {
  showReloadModal.value = false;
  store.reloadSystem();
}
</script>

<template>
  <div class="h-screen w-full bg-base-200 text-base-content font-sans selection:bg-primary/10 flex flex-col overflow-hidden">

    <!-- Navbar -->
    <nav class="relative h-16 shrink-0 z-50 bg-base-100 border-b border-base-200 flex items-center justify-between px-4 sm:px-6 lg:px-8">

      <!-- Left -->
      <div class="flex items-center gap-3 z-10 w-[250px]">
        <button class="md:hidden btn btn-square btn-sm btn-ghost" @click="toggleMobileMenu">
          <Menu v-if="!mobileMenuOpen" class="w-5 h-5" />
          <X v-else class="w-5 h-5" />
        </button>
        <div class="flex items-center gap-3">
          <img
              src="/super.png"
              alt="Super Process Manager"
              class="h-10 w-auto object-contain drop-shadow-sm hover:scale-105 transition-transform cursor-pointer"
              @click="$router.push('/')"
            />
        </div>
      </div>

      <!-- Center -->
      <div class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 hidden md:flex items-center gap-2">
        <RouterLink
          to="/"
          class="flex items-center gap-2.5 px-5 py-2 rounded-lg text-sm font-medium transition-all duration-200"
          active-class="bg-neutral text-neutral-content shadow-md"
          class-active="text-base-content/60 hover:text-base-content hover:bg-base-200"
        >
          <LayoutDashboard class="w-4 h-4" />
          <span>Overview</span>
        </RouterLink>

        <div class="dropdown">
          <div tabindex="0" role="button" class="flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium text-base-content/60 hover:text-base-content hover:bg-base-200 transition-colors cursor-pointer">
            <Settings class="w-4 h-4" />
            <span>Manage</span>
            <ChevronDown class="w-3 h-3 opacity-50" />
          </div>
          <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow-xl bg-base-100 rounded-xl w-52 border border-base-200 mt-2">
            <li>
              <!-- requestReload handler -->
              <a @click="requestReload" class="py-2.5">
                <RefreshCw class="w-4 h-4" />
                Reload Config
              </a>
            </li>
          </ul>
        </div>
      </div>

      <!-- Right -->
      <div class="flex items-center justify-end gap-3 z-10 w-[250px]">
        <button class="btn btn-circle btn-sm btn-ghost text-base-content/70 hover:text-base-content hover:bg-base-200" @click="toggleDark()">
          <component :is="isDark ? Moon : Sun" class="w-5 h-5" />
        </button>
        <div class="h-6 w-px bg-base-300"></div>
        <NavbarRight />
      </div>
    </nav>

    <!-- Mobile Menu -->
    <div v-if="mobileMenuOpen" class="absolute top-16 left-0 w-full bg-base-100 border-b border-base-200 shadow-xl z-40 md:hidden flex flex-col p-4 gap-2">
      <RouterLink to="/" class="btn btn-ghost justify-start gap-3" active-class="btn-active" @click="mobileMenuOpen = false">
        <LayoutDashboard class="w-4 h-4" /> Overview
      </RouterLink>
      <div class="divider my-0"></div>
      <button class="btn btn-ghost justify-start gap-3" @click="requestReload">
        <RefreshCw class="w-4 h-4" /> Reload Config
      </button>
      <button class="btn btn-ghost justify-start gap-3" @click="toggleDark()">
        <component :is="isDark ? Moon : Sun" class="w-4 h-4" /> {{ isDark ? 'Dark Mode' : 'Light Mode' }}
      </button>
    </div>

    <!-- Main Content -->
    <main class="flex-1 overflow-y-auto p-4 sm:p-6 lg:p-8 scroll-smooth" @click="mobileMenuOpen = false">
      <div class="max-w-[1600px] mx-auto">
        <RouterView />
      </div>
    </main>

    <AppFooter />

    <!-- Reload Modal (sp-btn) -->
    <dialog class="modal modal-bottom sm:modal-middle" :class="{ 'modal-open': showReloadModal }">
      <div class="modal-box bg-base-100 text-base-content shadow-2xl">
        <h3 class="font-bold text-lg flex items-center gap-2">
          <AlertTriangle class="w-5 h-5 text-warning" />
          Reload Configuration?
        </h3>
        <p class="py-4 text-base-content/70">
          This will reload the configuration from disk. Any removed programs will be stopped, and new ones will be added.
        </p>
        <div class="modal-action flex justify-end gap-3 mt-6">
          <button class="sp-btn sp-btn-cancel" @click="showReloadModal = false">Cancel</button>
          <button class="sp-btn sp-btn-warning" @click="confirmReload">Reload</button>
        </div>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button @click="showReloadModal = false">close</button>
      </form>
    </dialog>

  </div>
</template>

<style scoped>
.router-link-active {
  background-color: oklch(var(--n));
  color: oklch(var(--nc));
  box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);
}
:where([data-theme="dark"]) .router-link-active {
  background-color: oklch(var(--bc));
  color: oklch(var(--b1));
}
</style>
