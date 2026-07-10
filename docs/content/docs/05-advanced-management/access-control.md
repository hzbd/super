---
title: "Access Control (RBAC)"
weight: 2
description: "Fine-grained permissions for teams."
---

### Role-Based Access Control 💎

When multiple engineers manage a system, you don't want everyone to have `root` access. The `security` plugin implements RBAC to separate concerns.

## Available Roles

| Role | Permissions | Ideal For |
| :--- | :--- | :--- |
| **Viewer** | **Read-Only**. Can view process status, logs, and metrics. Cannot modify anything. | Developers debugging issues, Dashboard displays. |
| **Operator** | **Operational Actions**. Can Start, Stop, Restart, and Signal processes. Cannot create/delete programs or manage tokens. | SREs, On-call engineers, Automated Watchdogs. |
| **Admin** | **Full Access**. Can create/delete programs, manage authentication tokens, and reload system config. | System Administrators, CI/CD Deployment pipelines. |

## Usage Example

### Scenario: The Junior Developer

You want a developer to be able to view logs and restart their service, but not delete the database service or change the root configuration.

**1. Create an Operator Token (Admin or root `auth_secret`):**

```bash
curl -X POST http://127.0.0.1:9002/api/auth/tokens \
  -H "Authorization: Bearer <admin-or-root-secret>" \
  -H "Content-Type: application/json" \
  -d '{"name":"dev-team","role":"operator"}'
```

**2. Developer usage:**

```bash
# Restart allowed
curl -X POST -H "Authorization: Bearer sk-..." \
  http://127.0.0.1:9002/api/programs/<id>/restart

# Delete denied (403 Forbidden)
curl -X DELETE -H "Authorization: Bearer sk-..." \
  http://127.0.0.1:9002/api/programs/<database-id>
```
