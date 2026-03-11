import { useGameStore } from '../store/gameStore'
import { tileLabel } from '../types'

export function TileRack() {
  const { rack, selectedRackIndex, equalsSelected, selectRackTile, selectEquals } = useGameStore()

  return (
    <div className="flex items-center gap-2 p-3 bg-gray-800 rounded-xl border border-gray-700">
      <span className="text-gray-500 text-xs mr-1 shrink-0">RACK</span>

      {rack.map((tile, i) => {
        const isSelected = selectedRackIndex === i
        return (
          <div
            key={i}
            onClick={() => selectRackTile(isSelected ? null : i)}
            className={[
              'w-10 h-10 flex flex-col items-center justify-center rounded cursor-pointer select-none transition-transform',
              'bg-yellow-200 text-gray-900 font-bold',
              isSelected
                ? 'ring-2 ring-yellow-400 scale-110 shadow-lg shadow-yellow-400/30'
                : 'hover:scale-105',
            ].join(' ')}
          >
            <span className="text-base leading-none">{tileLabel(tile)}</span>
            <span className="text-[8px] text-gray-500 leading-none">{tile.point_value}</span>
          </div>
        )
      })}

      {rack.length === 0 && (
        <span className="text-gray-600 text-sm italic">empty</span>
      )}

      {/* Equals tile — always available, shown separately */}
      <div className="w-px h-8 bg-gray-600 mx-1" />
      <div
        onClick={selectEquals}
        title="= (always available)"
        className={[
          'w-10 h-10 flex flex-col items-center justify-center rounded cursor-pointer select-none transition-transform',
          'bg-yellow-200 text-gray-900 font-bold',
          equalsSelected
            ? 'ring-2 ring-yellow-400 scale-110 shadow-lg shadow-yellow-400/30'
            : 'hover:scale-105',
        ].join(' ')}
      >
        <span className="text-base leading-none">=</span>
        <span className="text-[8px] text-gray-500 leading-none">0</span>
      </div>
    </div>
  )
}
