---
title: "Web UI"
weight: 6
description: "Dashboard via the optional ui plugin; OSS is API and CLI only."
imageZoom: true
aliases:
  - /docs/02-essentials/web-ui/
  - /docs/02-essentials/web-ui
---

> **Licensed plugin 💎:** The dashboard requires the **`ui`** plugin in `[license].key`. OSS `superd` serves a static notice at `/` instead.

## OSS vs subscription

| Edition | Web UI at `/` |
| :--- | :--- |
| **OSS** (no plugins) | Static notice — **no dashboard**. Use `super` CLI or `/api/*`. |
| **Licensed** | Full dashboard served by the authorized UI plugin. |

OSS `superd` does **not** embed a dashboard binary. The optional **`ui`** plugin serves the web dashboard via `super_plugin_ui_v1`.

## Accessing the dashboard (licensed)

With the `ui` plugin loaded and authorized in `[license].key`:

**http://localhost:9002**

{{< callout icon="sparkles" >}}
  Assuming `port = 9002` in your config
{{< /callout >}}

Log in with your `auth_secret` when the **`security`** plugin is enabled (`super login <secret>`). See [Authentication](/docs/05-advanced-management/authentication).

## Dashboard tour

Screenshots below are from a licensed deployment. Use the tabs to browse each area — images are capped in width; **click to enlarge**.

{{< tabs >}}

  {{< tab name="Overview" icon="view-grid" >}}
Process list with host CPU/memory metrics (from the machine running **superd**), status filters, search, and topology view.

{{< ui-screenshot src="/images/overview.png" alt="Dashboard overview — process list and host metrics" >}}
  {{< /tab >}}

  {{< tab name="Program detail" icon="cog" >}}
Configuration drawer: command, hooks, health checks, resource limits, and environment for a selected program.

{{< ui-screenshot src="/images/program_config.png" alt="Program configuration drawer" >}}
  {{< /tab >}}

  {{< tab name="Logs" icon="terminal" >}}
Live stdout/stderr streaming from the process detail drawer.

{{< ui-screenshot src="/images/program_logtails.png" alt="Live program log tail" >}}
  {{< /tab >}}

  {{< tab name="Hot reload" icon="refresh" >}}
Reload plugin or dashboard assets without a full daemon restart (development workflow).

{{< ui-screenshot src="/images/reload_online.png" alt="Online reload controls" >}}
  {{< /tab >}}

  {{< tab name="Notifications" icon="bell" >}}
Notification channels and routing when the **`notify`** plugin is licensed (see [Event notifications](/docs/05-advanced-management/event-notifications)).

{{< ui-screenshot src="/images/notify.png" alt="Notification settings" >}}
  {{< /tab >}}

{{< /tabs >}}

## Deploy the ui plugin

Install the **`ui`** plugin library from your subscription delivery package into `$SUPER_ROOT/plugins/`.

Restart `superd` after updating plugins.

## Feature summary

| Area | What you get |
| :--- | :--- |
| **Overview** | Process counts, host metrics, filters, list/graph views |
| **Program detail** | Config, hooks, health checks, live logs, start/stop/restart |
| **Hot reload** | Reload plugins/dashboard without restarting `superd` |
| **Notifications** | Channel config (`notify` plugin) |

The dashboard also includes create/edit forms, a [declarative stack editor](/docs/04-production-scenarios/delivery/declarative-stack), API token management, and a license page — not shown above.

## Security

**OSS (no `security` plugin):** The API and any static page at `/` are reachable without authentication — restrict network access.

**With `security` plugin:** Token authentication and RBAC apply to the API and dashboard. Log in via `super login <auth_secret>` or create API tokens. See [Access control](/docs/05-advanced-management/access-control) and [Authentication](/docs/05-advanced-management/authentication).

> **Security tip:** When exposing beyond localhost, load the **`security`** plugin and set a strong `auth_secret` in `conf/super.toml`.
