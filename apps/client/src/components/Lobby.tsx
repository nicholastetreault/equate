import { useState } from 'react'

import { gameSocket } from '../ws/client'
import { useGameStore } from '../store/gameStore'

type Mode = 'idle' | 'create' | 'join'

export function Lobby() {
  const [mode, setMode] = useState<Mode>('idle')
  const [name, setName] = useState('')
  const [roomCode, setRoomCode] = useState('')
  const [error, setError] = useState<string | null>(null)

  const { phase, setPlayerName, setSession, setWaiting } = useGameStore()
  const isWaiting = phase === 'waiting'

  const createRoom = async () => {
    setError(null)
    try {
      const res = await fetch('/api/rooms', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ player_name: name }),
      })
      if (!res.ok) throw new Error('Failed to create room')
      const data = (await res.json()) as { room_code: string; player_id: string }
      setPlayerName(name)
      setSession(data.room_code, data.player_id)
      setWaiting()
      gameSocket.connect(data.room_code, data.player_id)
    } catch (e) {
      setError(String(e))
    }
  }

  const joinRoom = async () => {
    setError(null)
    try {
      const res = await fetch(`/api/rooms/${roomCode}/join`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ player_name: name }),
      })
      if (res.status === 404) throw new Error('Room not found')
      if (res.status === 409) throw new Error('Room is full or game already started')
      if (!res.ok) throw new Error('Failed to join room')
      const data = (await res.json()) as { room_code: string; player_id: string }
      setPlayerName(name)
      setSession(data.room_code, data.player_id)
      gameSocket.connect(data.room_code, data.player_id)
    } catch (e) {
      setError(String(e))
    }
  }

  const { roomCode: currentRoom } = useGameStore()

  return (
    <div className="flex flex-col items-center justify-center min-h-screen gap-8 px-4">
      <div className="text-center">
        <h1 className="text-7xl font-black tracking-widest text-yellow-400">EQUATE</h1>
        <p className="text-gray-400 mt-2">Math Scrabble — Online Multiplayer</p>
      </div>

      {isWaiting && (
        <div className="flex flex-col items-center gap-3 text-center">
          <div className="text-2xl font-mono font-bold text-yellow-400 tracking-widest border border-yellow-400 px-6 py-3 rounded-lg">
            {currentRoom}
          </div>
          <p className="text-gray-400">Share this room code with your opponent</p>
          <p className="text-gray-500 text-sm animate-pulse">Waiting for opponent to join...</p>
        </div>
      )}

      {!isWaiting && mode === 'idle' && (
        <div className="flex gap-4">
          <button
            onClick={() => setMode('create')}
            className="px-8 py-3 bg-yellow-400 text-gray-900 font-bold rounded-lg hover:bg-yellow-300 transition-colors"
          >
            Create Room
          </button>
          <button
            onClick={() => setMode('join')}
            className="px-8 py-3 border-2 border-yellow-400 text-yellow-400 font-bold rounded-lg hover:bg-yellow-400/10 transition-colors"
          >
            Join Room
          </button>
        </div>
      )}

      {!isWaiting && mode === 'create' && (
        <div className="flex flex-col gap-4 w-72">
          <input
            type="text"
            placeholder="Your name"
            value={name}
            onChange={(e) => setName(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && name.trim() && createRoom()}
            className="px-4 py-2 bg-gray-800 rounded-lg border border-gray-600 focus:border-yellow-400 outline-none"
          />
          {error && <p className="text-red-400 text-sm">{error}</p>}
          <button
            onClick={createRoom}
            disabled={!name.trim()}
            className="px-6 py-3 bg-yellow-400 text-gray-900 font-bold rounded-lg disabled:opacity-50 hover:bg-yellow-300 transition-colors"
          >
            Create
          </button>
          <button onClick={() => setMode('idle')} className="text-gray-400 hover:text-white text-sm">
            ← Back
          </button>
        </div>
      )}

      {!isWaiting && mode === 'join' && (
        <div className="flex flex-col gap-4 w-72">
          <input
            type="text"
            placeholder="Room code"
            value={roomCode}
            onChange={(e) => setRoomCode(e.target.value.toUpperCase())}
            className="px-4 py-2 bg-gray-800 rounded-lg border border-gray-600 focus:border-yellow-400 outline-none font-mono tracking-widest uppercase"
            maxLength={6}
          />
          <input
            type="text"
            placeholder="Your name"
            value={name}
            onChange={(e) => setName(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && name.trim() && roomCode.length >= 4 && joinRoom()}
            className="px-4 py-2 bg-gray-800 rounded-lg border border-gray-600 focus:border-yellow-400 outline-none"
          />
          {error && <p className="text-red-400 text-sm">{error}</p>}
          <button
            onClick={joinRoom}
            disabled={!name.trim() || roomCode.length < 4}
            className="px-6 py-3 bg-yellow-400 text-gray-900 font-bold rounded-lg disabled:opacity-50 hover:bg-yellow-300 transition-colors"
          >
            Join
          </button>
          <button onClick={() => setMode('idle')} className="text-gray-400 hover:text-white text-sm">
            ← Back
          </button>
        </div>
      )}
    </div>
  )
}
