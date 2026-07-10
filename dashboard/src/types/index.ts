export type ProcessStatus =
  | 'Stopped' | 'Starting' | 'Running' | 'Backoff'
  | 'Fatal' | 'Stopping' | 'Waiting' | 'Healthy';

export interface Program {
  id: string;
  name: string;
  group?: string;
  status: ProcessStatus;

  // snake_case fields for Rust backend
  pid?: number;
  uptime_sec?: number;
  updated_at: number;
  last_error?: string;
}

// WebSocket message types
export type WsMessageType = 'StatusChange' | 'Log';

export interface StatusChangePayload {
  id: string;
  status: ProcessStatus; // Reuses ProcessStatus
  name: string;
}

export interface LogPayload {
  id: string;
  source: 'stdout' | 'stderr';
  line: string;
}

// Union type matching Rust enum
export type WsMessage =
  | { type: 'StatusChange'; payload: StatusChangePayload }
  | { type: 'Log'; payload: LogPayload };

export interface ProgramLogFile {
  source: string;
  content: string;
}

export interface ProgramLogsResponse {
  id: string;
  logs: ProgramLogFile[];
}

// Full config (matches Rust ProgramConfig)
export interface ProgramConfig {
  name: string;
  command: string;
  args: string[];
  env: Record<string, string>;
  cwd?: string;
  user?: string;
  group?: string;
  autostart: boolean;
  retry_limit: number;
  autorestart?: 'unexpected' | 'true' | 'false';
  exitcodes?: number[];
  startsecs?: number;
  depends_on: string[];

  // Nested config
  hooks?: {
    pre_start?: string;
    post_start?: string;
    pre_stop?: string;
    post_stop?: string;
  };
  health_check?: {
    type: 'tcp' | 'http' | 'exec';
    host?: string;
    port?: number;
    url?: string;
    method?: string;
    command?: string;
  };

  cron?: string;
  artifact?: {
    source: string;
    checksum: string;
    destination: string;
    restart_policy: string;
  };
}

// Detail response (matches Rust ProgramInfo)
export interface ProgramDetail {
  id: string;
  state: ProcessStatus;
  pid?: number;
  config: ProgramConfig; // Includes all static config
  last_error?: string;
}
