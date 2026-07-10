---
title: "Web UI"
weight: 5
description: "Using the built-in dashboard for monitoring and control."
---

Super includes a lightweight, single-page application (SPA) dashboard embedded directly in the `superd` binary. No external web server (like Nginx) is required.

## Accessing the Dashboard

By default, the dashboard is available at:

**http://localhost:9002**

{{< callout icon="sparkles" >}}
  Assuming `port = 9002` in your config
{{< /callout >}}

![OSS Dashboard](/images/oss_dash.01.png "OSS Dashboard")
![Program Detail](/images/oss_dash.02.png "Program Detail")

## Features

1.  **Overview**: See status (Running, Stopped, Fatal) of all processes at a glance.
2.  **System metrics**: Host CPU and memory sparklines at the top of the dashboard (refreshed every ~3s).
3.  **Process details**: Configuration, hooks, health checks, and environment in the detail drawer.
4.  **Log console**: Live stdout/stderr streaming for the selected process.
5.  **Actions**: Start, stop, or restart processes with one click.

## Security

The OSS version of the Web UI allows unrestricted access to anyone who can reach the port.

> **Security Tip**: If you are exposing the dashboard to a public network, you should:
>
> 1.  Use a reverse proxy (like Nginx) with Basic Auth.
> 2.  Or load the **`security`** licensed plugin, which enables built-in **token authentication** and **RBAC** for the API and dashboard.
