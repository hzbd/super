---
title: "Terms of Service"
linkTitle: "Terms"
description: "Terms for Super Pro purchase, delivery, and support."
toc: true
---

**Last updated:** 2026-07-15  
**Contact:** support@sconts.com  
**Operator:** Project Super Team

## 1. Scope

These Terms apply to **Super Pro** (official paid plugins and license keys) — purchase, delivery, and related support.

The **Project Super** Community Edition on GitHub remains under its open-source licenses and is **not** governed by these commercial Terms.

## 2. What you buy

One Super Pro subscription unlocks official plugins on the same **`superd` / `super`** binaries you already run, typically including:

- **security** — API auth, RBAC, audit (required for licensed startup)
- **ui** — Dashboard
- **notify** — webhook notifications
- **isolation** — Linux cgroup limits (Linux)

After purchase you receive: a plugin archive for your platform, a signed license key, and a short config snippet (e.g. `[license].key` and `auth_secret`).

See the [feature matrix](/docs/07-editions/feature-matrix/); purchase guide: [Get Super Pro](/go/pro/).

## 3. Checkout and payment

Checkout and payment are completed on a **third-party platform** (currently **Afdian / 爱发电**). Super does not process card or wallet payments itself. **That platform’s terms apply to payment, settlement, and refunds initiated through their checkout.** We receive only the order details needed to fulfill (see [Privacy Policy](/legal/privacy/)).

**Super Pro** is sold for **annual coverage**: one annual payment maps to a **365-day** license.

Open-source supporter tips **do not** include official plugins or a license key.

## 4. Delivery

After payment clears and required order notes are complete (display name, OS + arch, email), we aim to deliver within **24 hours** by email or platform message. Delivery is digital only.

## 5. License use

- The key is for your licensed use of official plugins with a compatible `superd` major version.
- Do not redistribute private plugin packages or license keys to third parties.
- We may refuse fulfillment or revoke a key for fraud, chargeback abuse, or material breach (a refunded key is void).
- Phase 1 uses offline signed keys plus expiry checks; there is **no** always-on license server required to start.

## 6. Refunds

Payment and refund mechanics are governed first by **Afdian’s platform rules**. In addition:

- **Before** plugins and key are delivered: we will support a full refund via the platform where possible.
- **After** delivery: generally no refund; material fulfillment errors may be handled case by case.
- **Wrong monthly tip instead of annual:** we will help switch to annual payment or refund — we will **not** issue a 365-day key for a short monthly tip.
- A refunded or charged-back order voids any associated license key.

## 7. Support and changes

Support is best-effort via **support@sconts.com**. We may change prices, checkout channels, or the official plugin set; material changes will be noted on [Get Super Pro](/go/pro/) or the docs site. Paid plugins require a **currently valid** license.

## 8. Disclaimer and limitation of liability

Software is provided “as is.” To the maximum extent permitted by law, the operator is not liable for indirect, incidental, or lost-profit damages; total liability to you will not exceed the amounts you actually paid for Super Pro in the **12 months** before the claim.

You are responsible for how you deploy Super (bind address, secrets, network exposure). Community Edition has no API auth by default; licensed deployments must load the security plugin and configure it correctly.

## 9. Final interpretation

The developer reserves the final right of interpretation of these Terms.

## 10. Contact

Questions about these Terms: **support@sconts.com**.
