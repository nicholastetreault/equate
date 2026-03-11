import { useGameStore } from '../store/gameStore'

export function ScoreBoard() {
  const { scores, currentPlayer, playerId } = useGameStore()

  return (
    <div className="bg-gray-800 rounded-xl border border-gray-700 p-4 min-w-44">
      <h2 className="text-xs font-bold text-gray-400 uppercase tracking-wider mb-3">Scores</h2>
      <ul className="space-y-2">
        {scores.map((s) => (
          <li
            key={s.player_id}
            className={[
              'flex justify-between items-center px-2 py-1.5 rounded-lg',
              s.player_id === currentPlayer
                ? 'bg-yellow-400/15 border border-yellow-400/30'
                : 'border border-transparent',
            ].join(' ')}
          >
            <span className="font-medium text-sm">
              {s.player_name}
              {s.player_id === playerId && (
                <span className="text-gray-500 text-xs ml-1">(you)</span>
              )}
              {s.player_id === currentPlayer && (
                <span className="ml-1 text-yellow-400">●</span>
              )}
            </span>
            <span className="font-bold tabular-nums">{s.score}</span>
          </li>
        ))}
      </ul>
    </div>
  )
}
