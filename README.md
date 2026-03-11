# EQUATE ONLINE

A math-based board game inspired by Equate, playable by multiple people over the internet. Think Scrabble, but tiles are numbers and mathematical operators, and valid moves form correct equations on the board.

---

## MVP Scope

- 2-player online multiplayer via room codes
- Core Equate board and tile mechanics
- Real-time game state sync between players via native WebSockets
- Valid equation validation (server-authoritative, Rust game engine)
- Client-side move preview (same Rust engine compiled to WASM)
- Score tracking with premium square bonuses
- Simple lobby: create a room, share the code, play

Out of scope for MVP (future):
- User accounts and persistent history
- Spectator mode
- More than 2 players
- AI opponents
- PWA / Tauri desktop packaging
- Self-hosted deployment packaging

---

## How It Works

Players take turns placing number and operator tiles on a 19×19 board to form valid mathematical equations (e.g. `3 + 4 = 7`). Every equation placed must be mathematically correct, and every new tile must connect to existing tiles — similar to Scrabble's crossword placement rules. Consecutive number tiles form multi-digit numbers (e.g. `1` `2` = 12). Tiles can intersect with existing ones to form new equations in the perpendicular direction.

Each tile type has a point value, and the board contains premium squares (double/triple tile score, double/triple equation score). The player with the highest score when the tile bag is empty wins.

---

## Tech Stack

### Monorepo Structure

Two toolchain workspaces coexisting at the repo root:

- **Cargo workspace** — `apps/server` + `packages/game-engine`
- **pnpm workspace** — `apps/client`
- **`justfile`** — root-level task runner for dev, build, and WASM compilation

```
equate/
├── apps/
│   ├── client/             # React frontend
│   └── server/             # Axum backend
├── packages/
│   └── game-engine/        # Shared Rust game logic
├── Cargo.toml              # Cargo workspace root
├── pnpm-workspace.yaml
├── justfile
└── README.md
```

### Frontend (`apps/client`)
- **React + TypeScript** (via Vite) — component-driven UI for the board, tile rack, and lobby
- **Tailwind CSS** — utility-first styling
- **Zustand** — lightweight client-side game state
- **Native WebSocket API** — real-time communication with the server
- **`vite-plugin-wasm`** — loads the game engine WASM module for client-side move preview

### Backend (`apps/server`)
- **Rust + Axum + Tokio** — async HTTP + WebSocket server
- **Native WebSockets** (Axum built-in) — real-time bidirectional game events
- **Redis** — ephemeral game room state (fast, no persistence needed for MVP)
- **`game-engine`** crate used directly as a library dependency

### Shared Game Engine (`packages/game-engine`)
- Pure Rust, minimal dependencies
- Board representation and tile placement logic
- Equation validation (tokenizes and evaluates tile sequences with proper operator precedence)
- Scoring (tile point values × premium square multipliers)
- Win condition detection
- **Dual-compiled**: used as a native library by the server; compiled to **WASM via `wasm-pack`** for client-side move preview
- Server is authoritative — the client uses WASM for instant local feedback only

### Infrastructure (MVP / local)
- **Docker + docker-compose** — containerizes the server and Redis

---

## Getting Started

### Prerequisites
- Rust + Cargo (`rustup`)
- `wasm-pack` (`cargo install wasm-pack`)
- Node.js + pnpm (`npm i -g pnpm`)
- `just` (`cargo install just`)
- Redis (local or via Docker)

### Development

```bash
# Install frontend dependencies
pnpm install

# Build the game engine WASM for the client
just build-wasm

# Start both server and client in watch mode
just dev
```

### Individual commands

```bash
just client      # Start Vite dev server
just server      # Start Axum server
just test        # Run Rust tests
just build-wasm  # Compile game-engine → WASM → apps/client/src/wasm/
```

---

## Game Rules Summary

- **Board**: 19×19 grid with premium squares
- **Tiles**: digits 0–9, operators `+` `−` `×` `÷`, and `=`
- **Each turn**: place 1+ tiles in a single row or column
- **Valid move**: all affected rows/columns must form complete, correct equations
- **Consecutive digits**: adjacent number tiles form multi-digit numbers
- **First move**: must cover the center square (9, 9)
- **Scoring**: sum of tile point values × tile multipliers × equation multipliers for newly placed tiles
- **Game ends**: tile bag is empty and no valid moves remain
