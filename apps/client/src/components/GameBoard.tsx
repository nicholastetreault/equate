import { useGameStore } from '../store/gameStore'
import { PremiumSquare, tileLabel } from '../types'

const BOARD_SIZE = 15
const CENTER_ROW = 7
const CENTER_COL = 7

function premiumLabel(p: PremiumSquare | null): string {
  switch (p) {
    case 'DoubleTile': return '2S'
    case 'TripleTile': return '3S'
    case 'DoubleEquation': return '2E'
    case 'TripleEquation': return '3E'
    default: return ''
  }
}

function premiumBg(p: PremiumSquare | null): string {
  switch (p) {
    case 'DoubleTile': return 'bg-sky-500'         // teal/blue  — 2S
    case 'TripleTile': return 'bg-green-600'        // green      — 3S
    case 'DoubleEquation': return 'bg-purple-700'   // dark purple — 2E
    case 'TripleEquation': return 'bg-pink-600'     // pink/magenta — 3E
    default: return 'bg-indigo-200'                 // plain lavender square
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

  return (
    <div className="overflow-auto max-w-full">
      <div
        className="inline-grid gap-px bg-gray-500 p-px rounded"
        style={{ gridTemplateColumns: `repeat(${BOARD_SIZE}, minmax(0, 1fr))` }}
      >
        {board.cells.map((rowCells, row) =>
          rowCells.map((cell, col) => {
            const pending = pendingTiles.find((t) => t.row === row && t.col === col)
            const displayTile = pending?.tile ?? cell.tile
            const isCenter = row === CENTER_ROW && col === CENTER_COL

            return (
              <div
                key={`${row}-${col}`}
                onClick={() => handleCellClick(row, col)}
                className={[
                  'w-9 h-9 flex flex-col items-center justify-center cursor-pointer select-none transition-all',
                  displayTile
                    ? 'bg-yellow-100 text-gray-900'
                    : premiumBg(cell.premium),
                  pending ? 'ring-2 ring-inset ring-yellow-400' : '',
                  isCenter && !displayTile ? 'ring-2 ring-inset ring-white/70' : '',
                  'hover:brightness-110',
                ].join(' ')}
              >
                {displayTile ? (
                  <span className="text-sm font-bold leading-none">{tileLabel(displayTile)}</span>
                ) : (
                  <>
                    {cell.premium && (
                      <span className="text-white font-bold text-[9px] leading-none">{premiumLabel(cell.premium)}</span>
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
