# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Install frontend dependencies
pnpm install

# Compile game engine to WASM (required before running client)
just build-wasm

# Run both server and client in watch mode
just dev

# Run server only (Rust, port 3001)
just server

# Run client only (Vite, port 5173)
just client

# Run all Rust tests
just test

# Run tests for a specific package
cargo test -p game-engine

# Run a single test by name
cargo test -p game-engine <test_name>

# Clean all build artifacts
just clean
```

## Architecture

This is a real-time multiplayer math board game (Equate — math-based Scrabble) with three main components in a monorepo:

### `packages/game-engine` (Rust, dual-compiled)
The core game logic, compiled both as a native Rust library (used by the server) and as WASM (used by the client for move preview). Key modules:
- **`board.rs`** — 19×19 `Board` with `Cell`s; each cell has an optional `Tile` and optional `PremiumSquare` (DoubleTile, TripleTile, DoubleEquation, TripleEquation). Premium layout is hardcoded to official Equate rules.
- **`tile.rs`** — `TileKind` enum (Number, Fraction, Operator, Equals); tile bag with 190 tiles total.
- **`validation.rs`** — Validates that placed tiles form a single row/column, connect to existing tiles, and all resulting equations are mathematically valid. Uses proper operator precedence; consecutive digit tiles form multi-digit numbers; a Number tile immediately before a Fraction forms a mixed number.
- **`scoring.rs`** — Scores all tiles in an equation (not just new ones); symbol multipliers (2S/3S) apply only to newly placed tiles; equation multipliers (2E/3E) apply to the full equation total.
- **`lib.rs`** — `GameState` (board + tile bag); `apply_move()` is the main entry point; WASM exports `validate_move_wasm()` and `score_move_wasm()` for client-side preview.

### `apps/server` (Rust + Axum)
Server-authoritative game logic over WebSockets. Rooms are stored in `Arc<RwLock<HashMap<String, Room>>>`.
- **`room.rs`** — `Room` holds players (max 4), game state, and per-player mpsc channels (not a broadcast channel) for targeted messages.
- **`ws_handler.rs`** — On WebSocket connect: registers player, starts game when all players connected, spawns two Tokio tasks (mpsc→WS forwarder and WS message handler).
- **`messages.rs`** — `ClientMessage` (PlaceTiles, ExchangeTiles, PassTurn) and `ServerMessage` (RoomJoined, GameStarted, MoveAccepted, MoveRejected, TurnChanged, GameOver, Error) — must stay in sync with `apps/client/src/types.ts`.
- **`routes.rs`** — REST endpoints: `POST /api/rooms`, `GET /api/rooms/:code`, `POST /api/rooms/:code/join`.
- Listens on `0.0.0.0:3001`.

### `apps/client` (React + TypeScript + Tailwind + Zustand)
- **`types.ts`** — TypeScript mirrors of all Rust types. Keep in sync with `messages.rs` and game-engine types.
- **`store/gameStore.ts`** — Zustand store; game phases: `lobby → waiting → playing → game_over`. Tracks board, rack, pending (staged) tiles, scores.
- **`ws/client.ts`** — `GameSocket` class wrapping native WebSocket; proxied through Vite dev server to `localhost:3001`.
- **`components/Game.tsx`** — Main game UI; staged tiles tracked in `pendingTiles` until "Submit Move" is clicked.
- Vite proxies `/api` and `/ws` to `http://localhost:3001` in dev.

### Key Invariants
- The game engine is the single source of truth for validation and scoring — the server calls it natively; the client calls it via WASM only for preview.
- `ExchangeTiles` in `ws_handler.rs` is currently a TODO.
- RACK_SIZE is 9 (not 7 like Scrabble).
- The board center is `(9, 9)` (0-indexed) and is a DoubleEquation square; the first move must cover it.
