# EQUATE ONLINE

A math-based board game inspired by Equate, playable by multiple people over the internet. Think Scrabble, but tiles are numbers and mathematical operators, and valid moves form correct equations on the board.

---

## MVP Scope

- 2-player online multiplayer via room codes
- Core Equate board and tile mechanics
- Real-time game state sync between players
- Valid equation validation (server-authoritative)
- Score tracking with premium square bonuses
- Simple lobby: create/join a room, then play

Out of scope for MVP (future):
- User accounts and persistent history
- Spectator mode
- More than 2 players
- AI opponents
- Self-hosted deployment packaging

---

## How It Works

Players take turns placing number and operator tiles on a 19x19 board to form valid mathematical equations (e.g. `3 + 4 = 7`). Every equation placed must be mathematically correct, and every new tile must connect to existing tiles — similar to Scrabble's crossword placement rules. Tiles can intersect with existing ones to form new equations in either direction.

Each tile type has a point value, and the board contains premium squares (double/triple tile score, double/triple equation score). The player with the highest score when the tile bag is empty wins.

---

## Tech Stack

### Monorepo Structure
- **pnpm workspaces** — manages frontend, backend, and shared packages in one repo
- `apps/client` — React frontend
- `apps/server` — Node.js backend
- `packages/game-engine` — shared game logic (validation, scoring, board state) used by both client and server

### Frontend (`apps/client`)
- **React + TypeScript** (via Vite) — component-driven UI for the board, tile rack, and lobby
- **Tailwind CSS** — utility-first styling
- **Socket.IO client** — real-time communication with the server
- **Zustand** — lightweight client-side state management for game state

### Backend (`apps/server`)
- **Node.js + Fastify + TypeScript** — HTTP API for room creation/joining
- **Socket.IO** — real-time bidirectional game events (moves, turn changes, scores)
- **Redis** — ephemeral game room state and session storage (fast, no persistence needed for MVP)

### Shared Game Engine (`packages/game-engine`)
- Pure TypeScript, no dependencies
- Board representation and tile placement logic
- Equation validation (parses and evaluates placed tile sequences)
- Scoring calculation (tile values + premium squares)
- Win condition detection
- Running this on the server makes the server authoritative; the client uses it for local move previews

### Infrastructure (MVP / local)
- **Docker + docker-compose** — containerizes the server and Redis for easy local and eventual home-server deployment

---

## Project Structure

```
equate/
├── apps/
│   ├── client/         # React frontend
│   └── server/         # Fastify + Socket.IO backend
├── packages/
│   └── game-engine/    # Shared rules, validation, scoring
├── docker-compose.yml
├── pnpm-workspace.yaml
└── README.md
```

---

## Getting Started (once scaffolded)

```bash
pnpm install
pnpm dev        # starts both client and server in watch mode
```
