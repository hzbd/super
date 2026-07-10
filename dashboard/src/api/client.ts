// src/api/client.ts
import axios from 'axios';
import router from '@/router';

// Axios instance
const apiClient = axios.create({
  // baseURL empty; Vite proxy handles /api in dev
  timeout: 10000, // 10s timeout
  headers: {
    'Content-Type': 'application/json',
  },
});

// Response interceptor
apiClient.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response) {
      const status = error.response.status;

      // 403 Forbidden -> error page
      // (e.g. operator deleting token)
      if (status === 403) {
        router.push('/error/403');
      }

      // 5xx -> error page
      // (backend down or panic)
      else if (status >= 500) {
        // Treat all 5xx as fatal for now
        router.push(`/error/${status}`);
      }
    } else if (error.code === 'ERR_NETWORK') {
      // Network error or backend unreachable
      router.push('/error/503');
    }

    return Promise.reject(error);
  }
);

export default apiClient;
