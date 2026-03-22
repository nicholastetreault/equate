import React from 'react'
import { useGameStore } from '../store/gameStore'
import { PremiumSquare, tileLabel } from '../types'

const BOARD_SIZE = 19
const CENTER_ROW = 9
const CENTER_COL = 9

function premiumLabel(p: PremiumSquare | null): string {
  switch (p) {
    case 'DoubleTile': return '2S'
    case 'TripleTile': return '3S'
    case 'DoubleEquation': return '2E'
    case 'TripleEquation': return '3E'
    default: return ''
  }
}

function premiumBg(p: PremiumSquare): string {
  switch (p) {
    case 'DoubleTile': return 'bg-sky-500'
    case 'TripleTile': return 'bg-green-600'
    case 'DoubleEquation': return 'bg-purple-700'
    case 'TripleEquation': return 'bg-pink-600'
  }
}

const GRADIENT_IMAGE =
  'radial-gradient(ellipse at 15% 20%, #bfdbfe 0%, transparent 55%), ' +
  'radial-gradient(ellipse at 85% 80%, #ddd6fe 0%, transparent 55%), ' +
  'radial-gradient(ellipse at 75% 10%, #c7d2fe 0%, transparent 45%), ' +
  'radial-gradient(ellipse at 25% 85%, #e0d7fd 0%, transparent 45%), ' +
  'radial-gradient(circle at 50% 50%, #eae8f7, #eae8f7)'

// Each blank cell gets a slice of the gradient positioned to match its location on the board.
const CELL_SIZE = 36   // w-9
const GAP = 3
const PADDING = 3
const BOARD_PX = BOARD_SIZE * CELL_SIZE + (BOARD_SIZE - 1) * GAP // 738px

function blankCellStyle(row: number, col: number): React.CSSProperties {
  return {
    backgroundImage: GRADIENT_IMAGE,
    backgroundSize: `${BOARD_PX}px ${BOARD_PX}px`,
    backgroundPosition: `${-(PADDING + col * (CELL_SIZE + GAP))}px ${-(PADDING + row * (CELL_SIZE + GAP))}px`,
  }
}

export function GameBoard() {
  const { board, pendingTiles, selectedRackIndex, equalsSelected, rack, addPendingTile, removePendingTile, selectRackTile } =
    useGameStore()

  if (!board) return null

  const handleCellClick = (row: number, col: number) => {
    const pendingHere = pendingTiles.find((t) => t.row === row && t.col === col)
    if (pendingHere) {
      removePendingTile(row, col)
      return
    }

    if (equalsSelected) {
      addPendingTile({ tile: { kind: { type: 'Equals' }, point_value: 0 }, row, col })
      selectRackTile(null)
      return
    }

    if (selectedRackIndex !== null && rack[selectedRackIndex]) {
      addPendingTile({ tile: rack[selectedRackIndex], row, col })
      selectRackTile(null)
    }
  }

  const handleDrop = (e: React.DragEvent, row: number, col: number) => {
    e.preventDefault()
    // Don't drop onto an occupied or already-pending cell
    if (board.cells[row][col].tile) return
    if (pendingTiles.find((t) => t.row === row && t.col === col)) return

    const raw = e.dataTransfer.getData('rackIndex')
    if (raw === 'equals') {
      addPendingTile({ tile: { kind: { type: 'Equals' }, point_value: 0 }, row, col })
    } else {
      const idx = parseInt(raw)
      if (!isNaN(idx) && rack[idx]) {
        addPendingTile({ tile: rack[idx], row, col })
        selectRackTile(null)
      }
    }
  }

  return (
    <div className="overflow-auto max-w-full">
      <div
        className="inline-grid gap-[3px] p-[3px] rounded border-4"
        style={{ gridTemplateColumns: `repeat(${BOARD_SIZE}, minmax(0, 1fr))`, backgroundColor: '#d9d1a1', borderColor: '#30763e' }}
      >
        {board.cells.map((rowCells, row) =>
          rowCells.map((cell, col) => {
            const pending = pendingTiles.find((t) => t.row === row && t.col === col)
            const displayTile = pending?.tile ?? cell.tile
            const isCenter = row === CENTER_ROW && col === CENTER_COL

            const isBlank = !displayTile && !cell.premium

            return (
              <div
                key={`${row}-${col}`}
                onClick={() => handleCellClick(row, col)}
                onDragOver={(e) => e.preventDefault()}
                onDrop={(e) => handleDrop(e, row, col)}
                style={isBlank ? blankCellStyle(row, col) : undefined}
                className={[
                  'w-9 h-9 flex flex-col items-center justify-center cursor-pointer select-none transition-all rounded-[1px] border border-gray-500',
                  displayTile
                    ? 'bg-yellow-100 text-gray-900'
                    : cell.premium ? premiumBg(cell.premium) : '',
                  pending ? 'ring-2 ring-inset ring-yellow-400' : '',
                  isCenter && !displayTile ? 'ring-2 ring-inset ring-white/70' : '',
                  'hover:brightness-110',
                ].join(' ')}
              >
                {displayTile ? (
                  <span className="text-[18px] font-bold leading-none">{tileLabel(displayTile)}</span>
                ) : (
                  <>
                    {cell.premium && (
                      <span className="text-white font-bold text-[12px] leading-none">{premiumLabel(cell.premium)}</span>
                    )}
                  </>
                )}
              </div>
            )
          })
        )}
      </div>
    </div>
  )
}
