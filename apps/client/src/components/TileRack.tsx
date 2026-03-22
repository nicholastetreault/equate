import { useState } from 'react'
import { useGameStore } from '../store/gameStore'
import { tileLabel } from '../types'

export function TileRack() {
  const { rack, selectedRackIndex, equalsSelected, selectRackTile, selectEquals } = useGameStore()
  const [draggingIndex, setDraggingIndex] = useState<number | 'equals' | null>(null)

  return (
    <div className="flex items-center gap-2 p-3 bg-gray-800 rounded-xl border border-gray-700">
      <span className="text-gray-500 text-xs mr-1 shrink-0">RACK</span>

      {rack.map((tile, i) => {
        const isSelected = selectedRackIndex === i
        const isDragging = draggingIndex === i
        return (
          <div
            key={i}
            draggable
            onClick={() => selectRackTile(isSelected ? null : i)}
            onDragStart={(e) => {
              setDraggingIndex(i)
              e.dataTransfer.setData('rackIndex', String(i))
              e.dataTransfer.effectAllowed = 'move'
              const ghost = e.currentTarget.cloneNode(true) as HTMLElement
              ghost.style.cssText = 'position:fixed;top:-1000px;left:-1000px;opacity:1;'
              document.body.appendChild(ghost)
              e.dataTransfer.setDragImage(ghost, 20, 20)
              setTimeout(() => document.body.removeChild(ghost), 0)
            }}
            onDragEnd={() => setDraggingIndex(null)}
            className={[
              'w-10 h-10 flex flex-col items-center justify-center rounded cursor-grab select-none transition-transform',
              'bg-yellow-200 text-gray-900 font-bold',
              isDragging
                ? 'opacity-0'
                : isSelected
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
        draggable
        onClick={selectEquals}
        onDragStart={(e) => {
          setDraggingIndex('equals')
          e.dataTransfer.setData('rackIndex', 'equals')
          e.dataTransfer.effectAllowed = 'move'
          const ghost = e.currentTarget.cloneNode(true) as HTMLElement
          ghost.style.cssText = 'position:fixed;top:-1000px;left:-1000px;opacity:1;'
          document.body.appendChild(ghost)
          e.dataTransfer.setDragImage(ghost, 20, 20)
          setTimeout(() => document.body.removeChild(ghost), 0)
        }}
        onDragEnd={() => setDraggingIndex(null)}
        title="= (always available)"
        className={[
          'w-10 h-10 flex flex-col items-center justify-center rounded cursor-grab select-none transition-transform',
          'bg-yellow-200 text-gray-900 font-bold',
          draggingIndex === 'equals'
            ? 'opacity-0'
            : equalsSelected
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
