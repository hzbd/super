import pandas as pd
import matplotlib.pyplot as plt
import sys
import os
import argparse

def plot(data_dir, mode):
    plt.figure(figsize=(10, 8))

    # 定义颜色
    colors = {'super': '#e74c3c', 'supervisor': '#3498db', 'pm2': '#2ecc71'}
    tools = ['super', 'supervisor', 'pm2'] if mode == 'compare' else ['super']

    # --- 子图 1: 内存 (最关键指标) ---
    plt.subplot(2, 1, 1)
    has_data = False
    for tool in tools:
        csv_path = os.path.join(data_dir, f"{tool}.csv")
        if os.path.exists(csv_path):
            df = pd.read_csv(csv_path)
            plt.plot(df['time_ms'] / 1000, df['memory_mb'],
                     label=tool.upper(), color=colors.get(tool), linewidth=2)
            has_data = True

    plt.title('Memory Usage (Lower is Better)')
    plt.ylabel('RSS Memory (MB)')
    plt.grid(True, linestyle='--', alpha=0.5)
    plt.legend()

    # --- 子图 2: CPU (稳定性指标) ---
    plt.subplot(2, 1, 2)
    for tool in tools:
        csv_path = os.path.join(data_dir, f"{tool}.csv")
        if os.path.exists(csv_path):
            df = pd.read_csv(csv_path)
            # 使用滑动窗口平滑曲线，让趋势更清晰
            plt.plot(df['time_ms'] / 1000, df['cpu_usage'].rolling(3).mean(),
                     label=tool.upper(), color=colors.get(tool), linewidth=1.5)

    plt.title('CPU Usage (Stability check)')
    plt.xlabel('Time (seconds)')
    plt.ylabel('CPU (%)')
    plt.grid(True, linestyle='--', alpha=0.5)

    if mode == 'self':
        plt.ylim(bottom=0) # 自身测试时，Y轴从0开始看波动

    plt.tight_layout()
    output_path = os.path.join(data_dir, 'report.png')
    plt.savefig(output_path)
    print(f"Graph saved to {output_path}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("dir", help="Directory containing csv files")
    parser.add_argument("--mode", choices=['compare', 'self'], default='compare')
    args = parser.parse_args()

    plot(args.dir, args.mode)
