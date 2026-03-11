import { useGameStore } from '../store/gameStore'
import { PremiumSquare, Tile } from '../types'

const BOARD_SIZE = 19

function tileLabel(tile: Tile): string {
  switch (tile.kind.type) {
    case 'Number': return String(tile.kind.value)
    case 'Operator': return { Add: '+', Subtract: '−', Multiply: '×', Divide: '÷' }[tile.kind.op]
    case 'Equals': return '='
  }
}

function premiumLabel(p: PremiumSquare | null): string {
  switch (p) {
    case 'DoubleTile': return 'DT'
    case 'TripleTile': return 'TT'
    case 'DoubleEquation': return 'DE'
    case 'TripleEquation': return 'TE'
    default: return ''
  }
}

function premiumBg(p: PremiumSquare | null): string {
  switch (p) {
    case 'DoubleTile': return 'bg-blue-700'
    case 'TripleTile': return 'bg-blue-900'
    case 'DoubleEquation': return 'bg-red-700'
    case 'TripleEquation': return 'bg-red-900'
    default: return 'bg-gray-700'
  }
}

export function GameBoard() {
  const { board, pendingTiles, selectedRackIndex, rack, addPendingTile, removePendingTile, selectRackTile } =
    useGameStore()

  if (!board) return null

  const handleCellClick = (row: number, col: number) => {
    const pendingHere = pendingTiles.find((t) => t.row === row && t.col === col)
    if (pendingHere) {
      removePendingTile(row, col)
      return
    }
    if (selectedRackIndex !== null && rack[selectedRackIndex]) {
      addPendingTile({ tile: rack[selectedRackIndex], row, col })
      selectRackTile(null)
    }
  }

  return (
    <div className="overflow-auto max-w-full">
      <div
        className="inline-grid gap-px bg-gray-600 p-px rounded"
        style={{ gridTemplateColumns: `repeat(${BOARD_SIZE}, minmax(0, 1fr))` }}
      >
        {board.cells.map((rowCells, row) =>
          rowCells.map((cell, col) => {
            const pending = pendingTiles.find((t) => t.row === row && t.col === col)
            const displayTile = pending?.tile ?? cell.tile
            const isCenter = row === 9 && col === 9

            return (
              <div
                key={`${row}-${col}`}
                onClick={() => handleCellClick(row, col)}
                className={[
                  'w-8 h-8 flex items-center justify-center text-xs font-bold cursor-pointer select-none',
                  displayTile
                    ? 'bg-yellow-200 text-gray-900'
                    : premiumBg(cell.premium),
                  pending ? 'ring-2 ring-inset ring-yellow-400' : '',
                  isCenter && !displayTile ? 'ring-2 ring-inset ring-white/60' : '',
                  'hover:brightness-110',
                ].join(' ')}
              >
                {displayTile ? (
                  <span>{tileLabel(displayTile)}</span>
                ) : (
                  <span className="text-white/30 text-[8px]">{premiumLabel(cell.premium)}</span>
                )}
              </div>
            )
          })
        )}
      </div>
    </div>
  )
}
