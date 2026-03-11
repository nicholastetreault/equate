import { ClientMessage, ServerMessage } from '../types'

type MessageHandler = (msg: ServerMessage) => void

class GameSocket {
  private ws: WebSocket | null = null
  private handlers: MessageHandler[] = []

  connect(roomCode: string, playerId: string) {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:'
    const url = `${protocol}//${window.location.host}/ws/${roomCode}/${playerId}`
    this.ws = new WebSocket(url)

    this.ws.onmessage = (event) => {
      try {
        const msg = JSON.parse(event.data as string) as ServerMessage
        this.handlers.forEach((h) => h(msg))
      } catch (e) {
        console.error('Failed to parse server message', e)
      }
    }

    this.ws.onerror = (e) => console.error('WebSocket error', e)
    this.ws.onclose = () => console.log('WebSocket closed')
  }

  send(msg: ClientMessage) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(msg))
    }
  }

  /** Register a message handler. Returns an unsubscribe function. */
  onMessage(handler: MessageHandler): () => void {
    this.handlers.push(handler)
    return () => {
      this.handlers = this.handlers.filter((h) => h !== handler)
    }
  }

  disconnect() {
    this.ws?.close()
    this.ws = null
  }
}

export const gameSocket = new GameSocket()
