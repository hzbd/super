---
title: "Internals & Reference"
weight: 6
description: "Architecture design, technical FAQ, and complete API references."
---

Super is built on the philosophy of **Transparency**. We believe you should understand how your process manager works under the hood so you can trust it with your critical infrastructure.

### In this section

#### Architecture
*   [**OSS repository boundary**](./oss-boundary): What belongs in this repo vs subscription runtime.
*   [**Design Philosophy**](./design-philosophy): Why Rust? How does the Actor Model work? What is the WAL?
*   [**FAQ**](./faq): Technical deep-dives into Zombies, Signals, and Systemd comparisons.

#### Reference Manuals
*   [**Config Reference**](./config-reference): Complete `super.toml` schema.
*   [**CLI Reference**](./cli-reference): Command-line arguments and flags.
*   [**API Reference**](./api-reference): HTTP endpoints and JSON schemas.
