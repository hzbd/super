#!/bin/bash
set -e # Exit on first error

# --- Colors ---
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# --- Paths ---
WORK_DIR=$(pwd)
BIN_DIR="$WORK_DIR/target/release"
PAYLOAD_BIN="$BIN_DIR/payloads"
GENERATOR_BIN="$BIN_DIR/generator"
RUNNER_BIN="$BIN_DIR/runner"
PLOT_SCRIPT="$WORK_DIR/analysis/plot.py"
FINAL_RESULTS_DIR="$WORK_DIR/final_report"

# --- Step 0: Setup ---
echo -e "${BLUE}>>> Step 0: Building Benchmark Tools (Release Mode)...${NC}"
cargo build --release

# Install Python deps if needed
# pip install -r analysis/requirements.txt

mkdir -p "$FINAL_RESULTS_DIR"

# --- Run a single benchmark scenario ---
# Arg 1: scenario name (directory name)
# Arg 2: payload mode (idle, crash, log, mem-leak)
# Arg 3: process count
# Arg 4: duration (seconds)
run_scenario() {
    SCENARIO_NAME=$1
    MODE=$2
    COUNT=$3
    DURATION=$4

    echo -e "\n${YELLOW}======================================================${NC}"
    echo -e "${YELLOW}Running Scenario: ${SCENARIO_NAME} (Mode: ${MODE}, Count: ${COUNT})${NC}"
    echo -e "${YELLOW}======================================================${NC}"

    SCENARIO_DIR="$FINAL_RESULTS_DIR/$SCENARIO_NAME"
    CONFIG_DIR="$SCENARIO_DIR/configs"
    DATA_DIR="$SCENARIO_DIR/data"

    mkdir -p "$CONFIG_DIR"
    mkdir -p "$DATA_DIR"

    # 1. Generate configs
    echo -e "${BLUE}1. Generating configs...${NC}"
    "$GENERATOR_BIN" \
        --count "$COUNT" \
        --mode "$MODE" \
        --payload-path "$PAYLOAD_BIN" \
        --output-dir "$CONFIG_DIR"

    # 2. Run each target in sequence
    # Target list
    TARGETS=("super" "supervisor" "pm2")

    for TARGET in "${TARGETS[@]}"; do
        echo -e "${GREEN}Benchmarking: ${TARGET}...${NC}"

        # Run runner; write CSV
        "$RUNNER_BIN" \
            --target "$TARGET" \
            --config-dir "$CONFIG_DIR" \
            --duration "$DURATION" \
            --output-csv "$DATA_DIR/${TARGET}.csv"

        # Cool down to reduce thermal/cache bleed between runs
        echo "   Cooldown 5s..."
        sleep 5
    done

    # 3. Generate plots
    echo -e "${BLUE}3. Generating Plots...${NC}"
    python3 "$PLOT_SCRIPT" "$DATA_DIR"

    echo -e "${GREEN}Scenario '${SCENARIO_NAME}' completed. Graph saved to ${DATA_DIR}/benchmark_result.png${NC}"
}

# --- Scenario 1: Idle baseline ---
# Goal: compare baseline memory footprint
# 100 idle processes, 60s
run_scenario "01_baseline_idle" "idle" 100 60

# --- Scenario 2: Crash recovery storm ---
# Goal: compare CPU usage and recovery behavior
# 50 random crashers, 60s (kept moderate to avoid overloading the host)
run_scenario "02_crash_storm" "crash" 50 60

# --- Scenario 3: Log throughput pressure ---
# Goal: compare I/O and string-processing overhead
# 20 high-volume loggers, 60s
run_scenario "03_log_pressure" "log" 20 60

# --- Scenario 4: Memory leak simulation ---
# Goal: long-run stability (optional; shortened here for demo)
run_scenario "04_mem_leak" "mem-leak" 50 60

echo -e "\n${GREEN}All benchmarks completed. Check '${FINAL_RESULTS_DIR}' for reports.${NC}"
