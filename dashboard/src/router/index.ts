import { createRouter, createWebHistory } from 'vue-router'
import MainLayout from '@/layout/MainLayout.vue'
import ProcessList from '@/views/ProcessList.vue'

const ErrorPage = () => import('@/views/ErrorPage.vue')

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/error/:code',
      name: 'Error',
      component: ErrorPage,
      meta: { title: 'Error' }
    },
    {
      path: '/',
      component: MainLayout,
      // Child routes render in MainLayout <RouterView />
      children: [
        {
          path: '', // Default child route
          name: 'Dashboard',
          component: ProcessList
        }
      ]
    },

    // Catch-all for undefined routes
    {
      path: '/:pathMatch(.*)*',
      redirect: '/error/404'
    }
  ]
})

export default router
