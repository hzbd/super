---
title: "Web UI"
weight: 5
description: "Dashboard via the commercial ui plugin; OSS is API and CLI only."
---

## OSS vs commercial

| Edition | Web UI at `/` |
| :--- | :--- |
| **OSS** (no plugins) | Static notice — **no dashboard**. Use `super` CLI or `/api/*`. |
| **Licensed** (`ui` plugin) | Full SPA dashboard embedded in `plugins/ui.{so,dylib}`. |

OSS `superd` does **not** embed a dashboard binary. The commercial **`ui`** plugin ships the Vue dashboard (`super-plugins/dashboard`) via `super_plugin_ui_v1`.

## Accessing the dashboard (licensed)

With the `ui` plugin loaded and authorized in `[license].key`:

**http://localhost:9002**

{{< callout icon="sparkles" >}}
  Assuming `port = 9002` in your config
{{< /callout >}}

![Dashboard overview](/images/oss_dash.01.png "Dashboard overview")
![Program detail](/images/oss_dash.02.png "Program detail")

### Build & deploy the ui plugin

Dashboard assets are built in the private **`super-plugins`** repo:

```bash
cd super-plugins
make frontend    # dashboard/dist
make plugins     # → dist/plugins/ui.dylib (or .so)
cp dist/plugins/ui.* "$SUPER_ROOT/plugins/"
```

Restart `superd` after updating plugins.

## Features (ui plugin)

1.  **Overview**: Process status (Running, Stopped, Fatal) at a glance.
2.  **System metrics**: Host CPU and memory sparklines (~3s refresh).
3.  **Process details**: Configuration, hooks, health checks, environment.
4.  **Log console**: Live stdout/stderr streaming.
5.  **Actions**: Start, stop, or restart processes.
6.  **License page**: Subscription info when `[license].key` is configured (`GET /api/system/license`).

## Security

**OSS (no `security` plugin):** The API and any static page at `/` are reachable without authentication — restrict network access.

**With `security` plugin:** Token authentication and RBAC apply to the API and dashboard. Log in via `super login <auth_secret>` or create API tokens.

> **Security tip:** When exposing beyond localhost, load the **`security`** plugin and set a strong `auth_secret` in `conf/super.toml`.
