import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
// import path from 'path'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
      '@extensions': fileURLToPath(new URL('./src/extensions', import.meta.url))
    }
  },
  // Dev server config (required for API/WS proxy)
  server: {
    // Listen on all interfaces for LAN access
    host: '0.0.0.0',
    // Proxy config
    proxy: {
      // Proxy /api requests to the Rust backend
      '/api': {
        target: 'http://127.0.0.1:9002', // Rust backend port
        changeOrigin: true,
        // Do NOT add rewrite: (path) => path.replace(/^\/api/, '')
        // unless Rust routes omit the /api prefix
      },
      // Proxy WebSocket traffic
      '/ws': {
        target: 'ws://127.0.0.1:9002',
        ws: true,
        changeOrigin: true
      }
    }
  }
})
