---
title: "Orchestration"
weight: 3
description: "Manage complex systems with dependencies, health checks, and atomic updates."
---

Running a single process is easy. Running a system where Service A depends on Service B, and Service B needs to be updated without downtime, is hard.

Super simplifies this with built-in orchestration primitives.

### In this section

*   [**Dependencies**](./dependencies): Define startup order and topology.
*   [**Health Checks**](./health-checks): Ensure services are actually ready, not just running.
*   [**Lifecycle Hooks**](./lifecycle-hooks): Run scripts before or after process events.
*   [**System Events**](./system-events): Complete catalog of daemon events and where to configure reactions.
*   [**Event Hooks**](./event-hooks): Local scripts on system events (JSON on stdin).
*   [**Atomic OTA Updates**](./ota-updates): The fail-safe mechanism for updating binaries.