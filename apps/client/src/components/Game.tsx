import { useGameStore } from '../store/gameStore'
import { gameSocket } from '../ws/client'
import { GameBoard } from './GameBoard'
import { TileRack } from './TileRack'

export function Game() {
  const { pendingTiles, clearPendingTiles, currentPlayer, playerId, roomCode, phase, scores } =
    useGameStore()

  const isMyTurn = currentPlayer === playerId
  const isGameOver = phase === 'game_over'

  const submitMove = () => {
    if (pendingTiles.length === 0) return
    gameSocket.send({ type: 'place_tiles', tiles: pendingTiles })
  }

  const passTurn = () => {
    gameSocket.send({ type: 'pass_turn' })
  }

  if (isGameOver) {
    const sorted = [...scores].sort((a, b) => b.score - a.score)
    const winner = sorted[0]
    return (
      <div className="flex flex-col items-center justify-center min-h-screen gap-6">
        <h1 className="text-5xl font-black text-yellow-400">GAME OVER</h1>
        <p className="text-2xl font-bold">{winner?.player_name} wins!</p>
        <ul className="space-y-2">
          {sorted.map((s) => (
            <li key={s.player_id} className="flex gap-8 justify-between text-lg">
              <span>{s.player_name}</span>
              <span className="font-bold tabular-nums">{s.score}</span>
            </li>
          ))}
        </ul>
      </div>
    )
  }

  const half = Math.ceil(scores.length / 2)
  const leftScores = scores.slice(0, half)
  const rightScores = scores.slice(half)

  const renderScores = (list: typeof scores) => (
    <div className="flex flex-col gap-1 min-w-28">
      {list.map((s) => (
        <div
          key={s.player_id}
          className={[
            'flex justify-between items-center px-2 py-1 rounded-lg text-sm',
            s.player_id === currentPlayer
              ? 'bg-yellow-400/15 border border-yellow-400/30'
              : 'border border-transparent',
          ].join(' ')}
        >
          <span className="font-medium truncate">
            {s.player_name}
            {s.player_id === playerId && <span className="text-gray-500 text-xs ml-1">(you)</span>}
            {s.player_id === currentPlayer && <span className="ml-1 text-yellow-400">●</span>}
          </span>
          <span className="font-bold tabular-nums ml-2">{s.score}</span>
        </div>
      ))}
    </div>
  )

  return (
    <div className="flex flex-col items-center gap-4 p-4 min-h-screen">
      {/* Header */}
      <div className="relative flex items-center justify-center gap-6 w-full max-w-5xl">
        {renderScores(leftScores)}
        <h1 className="text-2xl font-black text-yellow-400 tracking-widest shrink-0">EQUATE</h1>
        {renderScores(rightScores)}
        <span className="absolute right-0 text-gray-500 text-sm">
          Room: <span className="text-gray-300 font-mono">{roomCode}</span>
        </span>
      </div>

      {/* Board + controls */}
      <div className="flex flex-col items-center gap-4">
        <GameBoard />
        <TileRack />

        {isMyTurn ? (
          <div className="flex gap-3">
            <button
              onClick={submitMove}
              disabled={pendingTiles.length === 0}
              className="px-6 py-2 bg-yellow-400 text-gray-900 font-bold rounded-lg disabled:opacity-40 hover:bg-yellow-300 transition-colors"
            >
              Submit Move
            </button>
            <button
              onClick={clearPendingTiles}
              disabled={pendingTiles.length === 0}
              className="px-4 py-2 border border-gray-600 text-gray-300 rounded-lg disabled:opacity-40 hover:border-gray-400 transition-colors"
            >
              Clear
            </button>
            <button
              onClick={passTurn}
              className="px-4 py-2 border border-gray-600 text-gray-300 rounded-lg hover:border-gray-400 transition-colors"
            >
              Pass
            </button>
          </div>
        ) : (
          <p className="text-gray-500 text-sm animate-pulse">Waiting for opponent...</p>
        )}
      </div>
    </div>
  )
}
