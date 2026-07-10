#!/bin/bash

BIN="../../target/debug/super"

echo "Seeding test programs..."

# --- 1. Long-running baseline services ---
# Usage: add [flags] <command> [args...]
$BIN add --name "redis-primary" --group "database" /bin/sleep 10000
$BIN add --name "postgres-main" --group "database" /bin/sleep 10000
$BIN add --name "nginx-gateway" --group "web" /bin/sleep 10000

# --- 2. Mock web service (cwd must precede command) ---
$BIN add --name "python-api" --group "web" --cwd "/tmp" python3 -m http.server 9091

# --- 3. High-volume log generator (mind shell quoting) ---
$BIN add --name "log-generator" --group "tools" /bin/sh -c "while true; do echo '[INFO] Log entry at \$(date)'; sleep 0.5; done"

# --- 4. Service that always crashes ---
$BIN add --name "crasher-service" --group "chaos" /bin/sh -c "echo 'I am going to crash...'; sleep 2; exit 1"

# --- 5. Limited retry service ---
$BIN add --name "fatal-service" --group "chaos" --retry-limit 3 /bin/sh -c "echo 'Failing...'; exit 1"

# --- 6. Environment variable test ---
$BIN add --name "env-checker" --group "tools" -e "API_KEY=sk-123456" -e "DEBUG=true" /bin/sh -c "echo \"My API Key is \$API_KEY\"; sleep 10000"

# --- 7. Multiple instances ---
$BIN add --name "image-processor" --group "workers" --numprocs 3 --process-name "worker-{num}" /bin/sleep 10000

# --- 8. Working directory ---
$BIN add --name "cwd-checker" --cwd "/var/log" /bin/ls -la

# --- 9. Mock compute workload ---
$BIN add --name "calc-engine" --group "compute" /bin/sleep 5000

# --- 10. Dependency chain (depends-on is repeatable) ---
$BIN add --name "backend-core" --group "app" --depends-on "redis-primary" --depends-on "postgres-main" /bin/sleep 10000

# --- 11. Miscellaneous services ---
$BIN add --name "prometheus" --group "monitoring" /bin/sleep 10000
$BIN add --name "grafana" --group "monitoring" /bin/sleep 10000
$BIN add --name "node-exporter" --group "monitoring" /bin/sleep 10000
$BIN add --name "backup-job" --group "maintenance" /bin/sleep 3600
$BIN add --name "cleanup-job" --group "maintenance" /bin/sleep 3600

# --- 12. Network command ---
$BIN add --name "ping-google" --group "network" ping 8.8.8.8

# --- 13. Manual start only ---
$BIN add --name "manual-task" --group "tools" --autostart false echo "I only run when you click start"

echo "Done: 20+ test programs added. Refresh the dashboard to verify."
