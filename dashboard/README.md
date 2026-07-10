# Dashboard (archived OSS embed)

The OSS embedded dashboard is **removed**. Web UI is delivered only via the commercial **`ui` plugin** (`ui.so` with embedded `dashboard/dist`).

- **OSS:** `superd` + CLI + HTTP API only.
- **Subscription:** build `super-plugins/dashboard`, then `scripts/build-plugin-ui.sh` → `plugins/ui.so`.

Dashboard source of truth: `super-plugins/dashboard`.
