import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import router from './router'
import { createPinia } from 'pinia' // 1. Import Pinia

const app = createApp(App)

app.use(createPinia()) // 2. Register Pinia
app.use(router)

app.mount('#app')
