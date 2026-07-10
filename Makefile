# ==========================================
# SuperProcess OSS Build System
# ==========================================

# Variable Definitions
FRONTEND_DIR := dashboard
TARGET_DIR   := target/release
# Define all OSS binaries (Server 'superd' and CLI 'super')
BINARIES     := --bin superd --bin super

# Colors for output
GREEN  := \033[0;32m
YELLOW := \033[0;33m
BLUE   := \033[0;34m
NC     := \033[0m # No Color

# Default target
.PHONY: all
all: build

# Help message
.PHONY: help
help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  build       Build all OSS binaries (Frontend + Backend)"
	@echo "  frontend    Build only the Frontend (Vue/Vite)"
	@echo "  backend     Build only the Rust Backend (assumes frontend is ready)"
	@echo "  clean       Clean up build artifacts (dist and target)"
	@echo "  check       Run cargo check"
	@echo "  docker      Build containerpi/super image (native arch, local load)"
	@echo "  docker-multi  Build and push linux/amd64 image"
	@echo ""

# ==========================================
# Core Build Tasks
# ==========================================

# 1. Build Frontend
.PHONY: frontend
frontend:
	@echo "$(BLUE)📦 Building Frontend (OSS)...$(NC)"
	@cd $(FRONTEND_DIR) && npm install
	@# Explicitly set VITE_EDITION=oss
	@cd $(FRONTEND_DIR) && VITE_EDITION=oss npm run build
	@echo "$(GREEN)✅ Frontend build complete.$(NC)"

# 2. Build Backend (Depends on Frontend)
.PHONY: backend
backend:
	@echo "$(BLUE)🦀 Building Rust Binaries (OSS)...$(NC)"
	@# Build both 'superd' and 'super' (CLI)
	@cargo build --release $(BINARIES)
	@echo "$(GREEN)✅ Backend build complete.$(NC)"

# 3. Full Build Workflow
.PHONY: build
build: frontend backend
	@echo "$(GREEN)🎉 All OSS binaries built successfully!$(NC)"
	@echo "📂 Locations:"
	@echo "   - Server: $(TARGET_DIR)/superd"
	@echo "   - CLI:    $(TARGET_DIR)/super"

# ==========================================
# Helper Tasks
# ==========================================

# Clean build artifacts
.PHONY: clean
clean:
	@echo "$(YELLOW)🧹 Cleaning up...$(NC)"
	@cargo clean
	@rm -rf $(FRONTEND_DIR)/dist
	@rm -rf $(FRONTEND_DIR)/node_modules
	@echo "$(GREEN)✅ Clean complete.$(NC)"

# Fast check (no build)
.PHONY: check
check:
	@cargo check

# Local docs preview (Hugo adjusts paths for localhost — do not open public/ as files)
.PHONY: docs-serve
docs-serve:
	@echo "$(BLUE)📖 Docs preview → http://localhost:1313/$(NC)"
	cd docs && hugo server -D --disableFastRender

# Docker image (build context = repo root)
DOCKER_IMAGE := containerpi/super:latest
DOCKERFILE   := dockerbuild/Dockerfile
DOCKER_PLATFORMS := linux/amd64

.PHONY: docker
docker:
	@echo "$(BLUE)🐳 Building $(DOCKER_IMAGE) (native arch)...$(NC)"
	docker buildx build --load -f $(DOCKERFILE) -t $(DOCKER_IMAGE) .
	@echo "$(GREEN)✅ Docker image ready: $(DOCKER_IMAGE)$(NC)"

.PHONY: docker-multi
docker-multi:
	@echo "$(BLUE)🐳 Building $(DOCKER_IMAGE) for $(DOCKER_PLATFORMS)...$(NC)"
	docker buildx build --platform $(DOCKER_PLATFORMS) -f $(DOCKERFILE) -t $(DOCKER_IMAGE) --push .
	@echo "$(GREEN)✅ Image pushed: $(DOCKER_IMAGE) ($(DOCKER_PLATFORMS))$(NC)"
