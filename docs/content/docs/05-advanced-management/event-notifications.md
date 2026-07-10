---
title: "Event Notifications"
weight: 5
description: "Real-time alerts via Webhooks, Slack, DingTalk, Lark, and Teams."
---

### Webhook System 💎

Super acts as an intelligent observer. Instead of just logging errors, it can actively push events to external systems like Slack, Microsoft Teams, or your company's internal IM tools.

## Configuration (`notify.toml`)

Notifications are configured in a separate file `conf/notify.toml`. This allows you to hot-reload alerting rules without restarting your processes.

> **Not `super.toml` `[webhook]`:** The OSS config schema includes an optional `[webhook]` section in `super.toml`, but it is **not wired at runtime** (parsed only). Licensed alerting uses `notify.toml` with the `notify` plugin. See [Config Reference — `[webhook]` reserved](/docs/06-internals/config-reference#webhook--reserved-not-active).

### Built-in Presets
You do not need to write complex JSON templates for popular platforms. Super includes built-in rich-text templates. Just set the `type` field to one of the supported presets:
*   `slack`
*   `dingtalk` (钉钉)
*   `lark` / `feishu` (飞书)
*   `wecom` / `wechat` (企业微信)
*   `teams` (Microsoft Teams)
*   `webhook` (Standard JSON payload)

**Example: Sending alerts to Slack and Lark**

```toml
[[channels]]
id = "slack-ops"
name = "Ops Team Slack"
type = "slack"
# Trigger on specific events or "*" for all
triggers = ["process_fatal", "process_backoff"]
include_log_tail = true

[channels.config]
url = "https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX"


[[channels]]
id = "lark-alert"
name = "Backend On-Call"
type = "lark"
triggers = ["*"]

[channels.config]
url = "https://open.feishu.cn/open-apis/bot/v2/hook/..."
```

## Supported Events

Use these strings in `triggers`. Full payload reference: [System Events](/docs/03-orchestration/system-events).

*   `process_started`: A process spawned successfully.
*   `process_fatal`: A process crashed and stopped (or exhausted retries). **Includes stderr tail.**
*   `process_backoff`: A process crashed but is restarting (flapping).
*   `process_recovered`: A previously crashing process has become healthy.
*   `system_startup`: The daemon started.
*   `system_shutdown`: The daemon is shutting down.
*   `*`: All of the above.

## Custom Webhook Payload

If you use the `webhook` type, Super sends a structured JSON payload. If a `secret` is configured, it includes an `X-Super-Signature` header (HMAC-SHA256) so you can verify the sender.

```json
{
  "id": "uuid...",
  "timestamp": "2023-10-27T10:00:00Z",
  "event": "process_fatal",
  "system": {
    "hostname": "prod-server-1",
    "version": "1.1.9"
  },
  "summary": "[Fatal] worker on prod-server-1: Stopped after 3 retries.",
  "markdown": "### Process Fatal Alert\n- Service: worker...",
  "data": { /* raw event data */ },
  "log_tail": "Error: Connection refused..."
}
```

This rich payload allows you to build custom integrations or sophisticated alerting rules in your own backend.