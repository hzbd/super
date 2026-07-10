---
title: "API Reference"
weight: 5
description: "HTTP REST API endpoints and JSON schemas."
---

Super exposes a RESTful API on port `9002` (default). All responses are in JSON format.

## Authentication

**Without `security` plugin**: No API authentication. The API is open on the bind address. Default bind is `127.0.0.1`.

**With `security` plugin loaded**: All API requests require `Authorization: Bearer <token>` (except health/docs whitelist). See [Authentication](/docs/05-advanced-management/authentication).

## Programs

### List Programs
Get a summary of all managed processes.

*   **GET** `/api/programs`

**Response:**
```json
[
  {
    "id": "a1b2c3d4-...",
    "name": "api-server",
    "status": "Running",
    "pid": 12345,
    "cpu_usage": 2.5,
    "mem_usage": 10485760
  }
]
```

### Create Program
Register a new process dynamically.

*   **POST** `/api/programs`

**Body:**
```json
{
  "name": "worker-1",
  "command": "./worker",
  "autostart": true,
  "autorestart": "unexpected",
  "exitcodes": [0],
  "startsecs": 10,
  "retry_limit": 3
}
```

| Field | Default | Description |
| :--- | :--- | :--- |
| `autorestart` | `unexpected` | `unexpected`, `true`, or `false` (Supervisor-compatible) |
| `exitcodes` | `[0]` | Exit codes treated as success when `autorestart=unexpected` |
| `startsecs` | `10` | Seconds of stable run before exit resets retry counter |

> **Migration note**: `autostart` controls boot-time start only. To disable crash auto-restart, set `"autorestart": "false"`.

### Get Details
Get full configuration and state for a specific program.

*   **GET** `/api/programs/{id}`

`{id}` is the program **UUID** (not the name). Resolve it from `GET /api/programs`.

### Update Program
Partially update an existing program. Only fields present in the body are changed; omitted fields are left unchanged.

*   **PUT** `/api/programs/{id}`

**Body** (all fields optional):

```json
{
  "command": "/usr/local/bin/my-app",
  "env": { "LOG_LEVEL": "debug" },
  "autorestart": "unexpected",
  "health_check": { "type": "http", "url": "http://127.0.0.1:8080/health" }
}
```

| Field | Description |
| :--- | :--- |
| `name`, `command`, `args`, `cwd`, `user`, `group` | Program identity and execution |
| `env`, `env_file` | Environment (`env_file` = `""` clears) |
| `autostart`, `retry_limit`, `autorestart`, `exitcodes`, `startsecs`, `stopsecs`, `priority` | Restart / stop behaviour |
| `depends_on`, `health_check`, `hooks` | Orchestration |
| `stdout_logfile`, `stderr_logfile` | Custom log paths |
| `artifact` | OTA binary update — see below |
| `cron` | Cron expression — see [Scheduled Tasks](/docs/02-essentials/scheduled-tasks). |
| `resource_limits` | 💎 Requires `isolation` plugin on Linux — stored in config always; enforced only when plugin is loaded |

> **Restart semantics**: Updating `command`, `env`, etc. **persists config only** — it does **not** restart a running process. Call `POST /api/programs/{id}/restart` explicitly, or change `artifact.checksum` to trigger an automatic OTA restart.

#### OTA update via API

When `artifact.checksum` differs from the stored value, Super starts the transactional OTA flow (download → verify → backup → swap → restart → health validate / rollback). See [Atomic OTA Updates](/docs/03-orchestration/ota-updates).

**Step 1 — resolve UUID:**

```bash
curl -s http://127.0.0.1:9002/api/programs \
  | jq -r '.[] | select(.name=="my-app") | .id'
```

**Step 2 — trigger update:**

```bash
curl -X PUT "http://127.0.0.1:9002/api/programs/${PROGRAM_ID}" \
  -H "Content-Type: application/json" \
  -d '{
    "artifact": {
      "source": "https://example.com/builds/v2.0.0/app-linux-amd64",
      "checksum": "a1b2c3d4e5f6789abcdef0123456789abcdef0123456789abcdef0123456789",
      "destination": "/usr/local/bin/my-app",
      "extract": false,
      "restart_policy": "immediate"
    }
  }'
```

| `artifact` field | Description |
| :--- | :--- |
| `source` | Download URL |
| `checksum` | Expected SHA256 hex digest |
| `destination` | Path of the binary on disk |
| `extract` | `true` if the download is an archive to extract |
| `restart_policy` | `"immediate"` (swap then restart) — primary supported policy |

**With `security` plugin**: add `-H "Authorization: Bearer <token>"`.

**Response:** `200 OK` on success; `400` if the program is not found or validation fails.

### Control Actions

Perform lifecycle actions.

*   **POST** `/api/programs/{id}/start`
*   **POST** `/api/programs/{id}/stop` (Query param: `?force=true`)
*   **POST** `/api/programs/{id}/restart`

### Historical Logs

Read the last N lines from on-disk log files (`{uuid}.out` / `{uuid}.err`).

*   **GET** `/api/programs/{id}/logs`

**Query parameters:**

| Param | Default | Description |
| :--- | :--- | :--- |
| `tail` | `200` | Lines from end of file (max 5000) |
| `source` | both | `stdout` or `stderr` |

**Response:**
```json
{
  "id": "a1b2c3d4-...",
  "logs": [
    { "source": "stdout", "content": "line-1\nline-2\n" },
    { "source": "stderr", "content": "error line\n" }
  ]
}
```

### Send Signal

*   **POST** `/api/programs/{id}/signal`

**Body:**
```json
{
  "signal": "hup"
}
```

## System & Stack

### Apply Stack (Declarative)
Update the entire system state to match a JSON definition.

*   **PUT** `/api/stack`

**Body:**
```json
{
  "prune": true,
  "services": [ ... list of program configs ... ]
}
```

### Shutdown
Gracefully stop the daemon.

*   **POST** `/api/system/shutdown`

### System Stats
Host-level CPU and memory snapshot (refreshed every ~3s by the monitor thread).

*   **GET** `/api/system/stats`

**Response:**
```json
{
  "cpu_percent": 12.4,
  "memory_used_bytes": 4294967296,
  "memory_total_bytes": 17179869184,
  "timestamp": 1719820800
}
```

## Observability

### Prometheus Metrics
Export metrics in Prometheus text format.

*   **GET** `/metrics`

### Log Stream (WebSocket)
Stream stdout/stderr.

*   **WS** `/ws?id={program_id}`

## Batch Operations

Perform actions on multiple programs simultaneously.

*   **POST** `/api/programs/batch`

**Body:**
```json
{
  "target_ids": ["uuid-1", "uuid-2"], // Or omit and use "group_name": "backend"
  "select_all": false,
  "action": {
    "type": "Restart" // Or "Start", "Stop", "Remove", "Signal"
  }
}
```

## Security & Authentication (`security` plugin 💎)

> **Without the plugin**: These routes are not registered. Requests return **404 Not Found**.

Manage access tokens for API authorization.

### List Tokens
*   **GET** `/api/auth/tokens`

### Create Token
*   **POST** `/api/auth/tokens`

**Body:**
```json
{
  "name": "ci-deploy-bot",
  "role": "operator"
}
```

### Revoke Token
*   **DELETE** `/api/auth/tokens/{id}`


## System Configuration (licensed plugins 💎)

> **Without the relevant plugin**: Routes below return **404 Not Found**.

### Get License Info
*   **GET** `/api/system/license`

### Manage Notifications
View or hot-reload webhook channels.

*   **GET** `/api/system/notify`
*   **PUT** `/api/system/notify`
*   **POST** `/api/system/notify/test`