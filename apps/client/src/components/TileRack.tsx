import { useGameStore } from '../store/gameStore'
import { Tile } from '../types'

function tileLabel(tile: Tile): string {
  switch (tile.kind.type) {
    case 'Number': return String(tile.kind.value)
    case 'Operator': return { Add: '+', Subtract: '−', Multiply: '×', Divide: '÷' }[tile.kind.op]
    case 'Equals': return '='
  }
}

export function TileRack() {
  const { rack, selectedRackIndex, selectRackTile, pendingTiles } = useGameStore()

  // Track which rack indices are already placed on the board
  // Simple O(n²) check — rack is at most 7 tiles
  const placedCount = pendingTiles.length

  return (
    <div className="flex items-center gap-2 p-3 bg-gray-800 rounded-xl border border-gray-700">
      <span className="text-gray-500 text-xs mr-1">RACK</span>
      {rack.map((tile, i) => {
        const isSelected = selectedRackIndex === i
        return (
          <div
            key={i}
            onClick={() => selectRackTile(isSelected ? null : i)}
            className={[
              'w-10 h-10 flex flex-col items-center justify-center rounded cursor-pointer select-none transition-transform',
              'bg-yellow-200 text-gray-900 font-bold',
              isSelected ? 'ring-2 ring-yellow-400 scale-110 shadow-lg shadow-yellow-400/30' : 'hover:scale-105',
            ].join(' ')}
          >
            <span className="text-lg leading-none">{tileLabel(tile)}</span>
            <span className="text-[8px] text-gray-500 leading-none">{tile.point_value}</span>
          </div>
        )
      })}
      {rack.length === 0 && (
        <span className="text-gray-600 text-sm italic">empty</span>
      )}
    </div>
  )
}
