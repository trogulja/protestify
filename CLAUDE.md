# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Protestify is a Tauri v2 desktop application that serves as a companion tool for productive.io cucumber-js tests. It helps visualize and manage BDD/Cucumber test features, scenarios, organizations, teams, and owners.

## Tech Stack

- **Frontend**: SvelteKit 2 + Svelte 5 with TypeScript
- **Styling**: TailwindCSS + DaisyUI
- **Backend**: Rust (Tauri v2)
- **Package Manager**: pnpm

## Common Commands

```bash
# Development
pnpm install        # Install dependencies
pnpm dev            # Start Vite dev server (frontend only)
pnpm tauri dev      # Start full Tauri dev environment (frontend + Rust backend)

# Type checking
pnpm check          # Run svelte-check for TypeScript errors
pnpm check:watch    # Watch mode for type checking

# Building
pnpm build          # Build frontend only
pnpm build:app      # Full app build (requires secret.sh with signing keys)
pnpm tauri build    # Build Tauri app

# Rust-specific (run from src-tauri/)
cargo build         # Build Rust backend
cargo test          # Run Rust tests
cargo clippy        # Run linter
```

## Architecture

### Frontend (src/)
- `src/routes/` - SvelteKit file-based routing with pages for features, scenarios, organizations, owners, teams, settings
- `src/lib/model/` - TypeScript models matching Rust data structures (Feature, Scenario, Organization, Owner, Team)
- `src/lib/store/` - Svelte stores for app state management
- `src/lib/ui/` - Reusable Svelte components

### Backend (src-tauri/src/)
- `lib.rs` - Main entry point, defines Tauri commands exposed to frontend
- `features_reader.rs` - Parses Gherkin .feature files to extract features and scenarios
- `organizations_reader.rs` - Parses YAML organization definitions
- `e2e_locator.rs` - Locates E2E test repositories on the filesystem
- `file_reader.rs` - File content reading utilities
- `cucumber_runner.rs` - Runs cucumber tests

### Tauri Commands (invoke from frontend)
- `get_features(base_path)` - Parse all feature files from a directory
- `get_organizations(file_path)` - Parse organization YAML file
- `find_e2e_repo()` - Auto-locate E2E test repository
- `validate_e2e_repo(path)` - Check if path is valid E2E repo
- `get_file_contents(file_path, scenario_name)` - Read file with optional scenario highlighting
- `run_e2e(folder_path, feature_file, scenario_name)` - Execute cucumber tests

## Key Patterns

- Frontend invokes Rust backend via `@tauri-apps/api` invoke calls
- Rust functions return `serde_json::Value` with either `{ ok: ... }` or `{ err: ... }` patterns
- App uses Tauri plugins for: clipboard, dialog, fs, global shortcuts, HTTP, notifications, process, shell, store, updater, window-state
