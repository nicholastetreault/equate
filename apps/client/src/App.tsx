import { useEffect } from 'react'

import { gameSocket } from './ws/client'
import { useGameStore } from './store/gameStore'
import { Lobby } from './components/Lobby'
import { Game } from './components/Game'
import { ServerMessage } from './types'

export default function App() {
  const { phase, setWaiting, setGameStarted, setMoveAccepted, setCurrentPlayer, setGameOver } =
    useGameStore()

  useEffect(() => {
    const unsub = gameSocket.onMessage((msg: ServerMessage) => {
      switch (msg.type) {
        case 'waiting_for_opponent':
          setWaiting()
          break
        case 'game_started':
          setGameStarted(msg.board, msg.your_rack, msg.players, msg.current_player)
          break
        case 'move_accepted':
          setMoveAccepted(msg.board, msg.scores, msg.next_player, msg.your_new_rack)
          break
        case 'turn_changed':
          setCurrentPlayer(msg.current_player)
          break
        case 'game_over':
          setGameOver(msg.scores, msg.winner)
          break
        case 'error':
          console.error('Server error:', msg.message)
          break
      }
    })
    return unsub
  }, [setWaiting, setGameStarted, setMoveAccepted, setCurrentPlayer, setGameOver])

  return (
    <div className="min-h-screen bg-gray-900 text-white">
      {phase === 'lobby' || phase === 'waiting' ? <Lobby /> : <Game />}
    </div>
  )
}
