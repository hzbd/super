// src/api/paths.ts

const BASE_URL = '/api';

export const API_PATHS = {
  // System endpoints
  SYSTEM: {
    INFO: `${BASE_URL}/system/info`,
    RELOAD: `${BASE_URL}/system/reload`,
    STATS: `${BASE_URL}/system/stats`,
  },

  // Program endpoints
  PROGRAMS: {
    // GET /api/programs
    LIST: `${BASE_URL}/programs`,

    // POST /api/programs
    CREATE: `${BASE_URL}/programs`,

    // GET /api/programs/:id
    DETAIL: (id: string) => `${BASE_URL}/programs/${id}`,

    // POST /api/programs/:id/start|stop|restart
    START: (id: string) => `${BASE_URL}/programs/${id}/start`,
    STOP: (id: string) => `${BASE_URL}/programs/${id}/stop`,
    RESTART: (id: string) => `${BASE_URL}/programs/${id}/restart`,

    // DELETE /api/programs/:id
    REMOVE: (id: string) => `${BASE_URL}/programs/${id}`,

    LOGS: (id: string, tail = 200, source?: string) => {
      const params = new URLSearchParams({ tail: String(tail) });
      if (source) params.set('source', source);
      return `${BASE_URL}/programs/${id}/logs?${params}`;
    },
  },
  
};
