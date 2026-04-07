package hub

import (
	"encoding/json"
	"log"
	"sync"

	"github.com/gorilla/websocket"
	"shihuang-guard/internal/model"
)

// Client wraps a single WebSocket connection.
type Client struct {
	conn *websocket.Conn
	send chan []byte
}

// Hub maintains active WebSocket clients and broadcasts messages.
type Hub struct {
	clients    map[*Client]bool
	broadcast  chan []byte
	register   chan *Client
	unregister chan *Client
	mu         sync.RWMutex
}

// New creates a ready‑to‑run Hub.
func New() *Hub {
	return &Hub{
		clients:    make(map[*Client]bool),
		broadcast:  make(chan []byte, 512),
		register:   make(chan *Client),
		unregister: make(chan *Client),
	}
}

// Run is the hub's main event loop — start it as a goroutine.
func (h *Hub) Run() {
	for {
		select {
		case c := <-h.register:
			h.mu.Lock()
			h.clients[c] = true
			h.mu.Unlock()
			log.Printf("[WS] client connected  (total: %d)", h.ClientCount())

		case c := <-h.unregister:
			h.mu.Lock()
			if _, ok := h.clients[c]; ok {
				delete(h.clients, c)
				close(c.send)
			}
			h.mu.Unlock()
			log.Printf("[WS] client disconnected (total: %d)", h.ClientCount())

		case msg := <-h.broadcast:
			h.mu.RLock()
			for c := range h.clients {
				select {
				case c.send <- msg:
				default:
					close(c.send)
					delete(h.clients, c)
				}
			}
			h.mu.RUnlock()
		}
	}
}

// BroadcastAlert pushes an alert to all connected clients.
func (h *Hub) BroadcastAlert(alert *model.Alert) {
	data, _ := json.Marshal(model.WSMessage{Type: "alert", Payload: alert})
	h.broadcast <- data
}

// BroadcastStats pushes dashboard stats to all connected clients.
func (h *Hub) BroadcastStats(stats *model.DashboardStats) {
	data, _ := json.Marshal(model.WSMessage{Type: "stats", Payload: stats})
	h.broadcast <- data
}

// AddClient registers a new WebSocket connection and starts its pumps.
func (h *Hub) AddClient(conn *websocket.Conn) {
	c := &Client{conn: conn, send: make(chan []byte, 256)}
	h.register <- c
	go c.writePump()
	go c.readPump(h)
}

// ClientCount returns the number of active connections.
func (h *Hub) ClientCount() int {
	h.mu.RLock()
	defer h.mu.RUnlock()
	return len(h.clients)
}

// ---- pumps -----------------------------------------------------------------

func (c *Client) writePump() {
	defer c.conn.Close()
	for msg := range c.send {
		if err := c.conn.WriteMessage(websocket.TextMessage, msg); err != nil {
			return
		}
	}
}

func (c *Client) readPump(h *Hub) {
	defer func() {
		h.unregister <- c
		c.conn.Close()
	}()
	for {
		if _, _, err := c.conn.ReadMessage(); err != nil {
			break
		}
	}
}
