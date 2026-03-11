import { create } from 'zustand'

import { Board, PlacedTile, PlayerInfo, PlayerScore, Tile } from '../types'

type GamePhase = 'lobby' | 'waiting' | 'playing' | 'game_over'

interface GameStore {
  // Identity
  roomCode: string | null
  playerId: string | null
  playerName: string

  // Game state
  phase: GamePhase
  board: Board | null
  rack: Tile[]
  players: PlayerInfo[]
  scores: PlayerScore[]
  currentPlayer: string | null

  // Move staging
  pendingTiles: PlacedTile[]
  selectedRackIndex: number | null
  equalsSelected: boolean

  // Actions
  setPlayerName: (name: string) => void
  setSession: (roomCode: string, playerId: string) => void
  setWaiting: () => void
  setGameStarted: (board: Board, rack: Tile[], players: PlayerInfo[], currentPlayer: string) => void
  setMoveAccepted: (board: Board, scores: PlayerScore[], nextPlayer: string, newRack: Tile[] | null) => void
  setCurrentPlayer: (playerId: string) => void
  setGameOver: (scores: PlayerScore[], winner: string) => void

  addPendingTile: (tile: PlacedTile) => void
  removePendingTile: (row: number, col: number) => void
  clearPendingTiles: () => void
  selectRackTile: (index: number | null) => void
  selectEquals: () => void
}

export const useGameStore = create<GameStore>((set) => ({
  roomCode: null,
  playerId: null,
  playerName: '',
  phase: 'lobby',
  board: null,
  rack: [],
  players: [],
  scores: [],
  currentPlayer: null,
  pendingTiles: [],
  selectedRackIndex: null,
  equalsSelected: false,

  setPlayerName: (name) => set({ playerName: name }),

  setSession: (roomCode, playerId) => set({ roomCode, playerId }),

  setWaiting: () => set({ phase: 'waiting' }),

  setGameStarted: (board, rack, players, currentPlayer) =>
    set({
      board,
      rack,
      players,
      currentPlayer,
      phase: 'playing',
      scores: players.map((p) => ({ player_id: p.id, player_name: p.name, score: 0 })),
    }),

  setMoveAccepted: (board, scores, nextPlayer, newRack) =>
    set((state) => ({
      board,
      scores,
      currentPlayer: nextPlayer,
      rack: newRack ?? state.rack,
      pendingTiles: [],
    })),

  setCurrentPlayer: (currentPlayer) => set({ currentPlayer }),

  setGameOver: (scores, _winner) => set({ scores, phase: 'game_over' }),

  addPendingTile: (tile) =>
    set((state) => ({ pendingTiles: [...state.pendingTiles, tile] })),

  removePendingTile: (row, col) =>
    set((state) => ({
      pendingTiles: state.pendingTiles.filter((t) => !(t.row === row && t.col === col)),
    })),

  clearPendingTiles: () => set({ pendingTiles: [], selectedRackIndex: null, equalsSelected: false }),

  selectRackTile: (index) => set({ selectedRackIndex: index, equalsSelected: false }),

  selectEquals: () => set((state) => ({
    equalsSelected: !state.equalsSelected,
    selectedRackIndex: null,
  })),
}))
