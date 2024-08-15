# Makefile for Tauri project

# Project Metadata
AUTHOR = @Aero25x & @stepanBD

# Default target: Help
.DEFAULT_GOAL := help

# Help: Show available commands
help:
	@echo "Usage: make [command]"
	@echo ""
	@echo "Available commands:"
	@echo "  help       Show this help message"
	@echo "  dev        Run the Tauri application in development mode"
	@echo "  build      Build the Tauri application for production"
	@echo "  clean      Clean the project (remove target directory)"
	@echo "  author     Show the author information"

# Dev: Run Tauri in development mode
dev:
	cargo tauri dev

# Build: Build the Tauri application for production
build:
	cargo tauri build

# Clean: Remove target directory
clean:
	cargo clean

# Author: Show author information
author:
	@echo "Author: $(AUTHOR)"

.PHONY: help dev build clean author
