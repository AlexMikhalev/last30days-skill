package main

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"time"
)

// Config holds WhatsApp client settings
type Config struct {
	GatewayPort string // default 18789
}

// WhatsAppMessage represents a WhatsApp message
type WhatsAppMessage struct {
	ID       string `json:"id"`
	From     string `json:"from"`
	Body     string `json:"body"`
	SentAt   int64  `json:"sentAt"`
	IsIncoming bool `json:"isIncoming"`
}

// Client wraps OpenClaw CLI operations
type Client struct {
	config Config
}

// NewClient creates a new WhatsApp client
func NewClient() *Client {
	return &Client{
		config: Config{
			GatewayPort: "18789",
		},
	}
}

// Send sends a WhatsApp message
func (c *Client) Send(to, message string) error {
	cmd := exec.Command("openclaw", "message", "send", 
		"--target", to, 
		"--message", message,
	)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	return cmd.Run()
}

// Read retrieves recent messages
func (c *Client) Read(target string, limit int) ([]WhatsAppMessage, error) {
	args := []string{"message", "read", "--json"}
	if target != "" {
		args = append(args, "-t", target)
	}
	if limit > 0 {
		args = append(args, "--limit", fmt.Sprintf("%d", limit))
	}

	cmd := exec.Command("openclaw", args...)
	output, err := cmd.Output()
	if err != nil {
		return nil, fmt.Errorf("failed to read messages: %w", err)
	}

	var messages []WhatsAppMessage
	if err := json.Unmarshal(output, &messages); err != nil {
		// Try parsing as single message
		var msg WhatsAppMessage
		if err := json.Unmarshal(output, &msg); err == nil {
			messages = []WhatsAppMessage{msg}
		} else {
			return nil, fmt.Errorf("failed to parse messages: %w", err)
		}
	}

	return messages, nil
}

// PollMessages checks for new messages (simple polling)
func (c *Client) PollMessages(target string, interval time.Duration, handler func(WhatsAppMessage)) {
	ticker := time.NewTicker(interval)
	defer ticker.Stop()

	lastID := ""
	
	for {
		messages, err := c.Read(target, 10)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error reading messages: %v\n", err)
			time.Sleep(interval)
			continue
		}

		// Process new messages (in reverse order, newest first)
		for i := len(messages) - 1; i >= 0; i-- {
			msg := messages[i]
			if msg.ID != lastID && msg.IsIncoming {
				handler(msg)
				lastID = msg.ID
			}
		}

		<-ticker.C
	}
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: whatsapp-client <send|read|poll> [args...]")
		fmt.Println("  send <to> <message>  - Send a message")
		fmt.Println("  read [target] [n]   - Read recent messages")
		fmt.Println("  poll [target]        - Poll for new messages")
		os.Exit(1)
	}

	client := NewClient()
	command := os.Args[1]

	switch command {
	case "send":
		if len(os.Args) < 4 {
			fmt.Println("Usage: whatsapp-client send <to> <message>")
			os.Exit(1)
		}
		to := os.Args[2]
		message := os.Args[3]
		if err := client.Send(to, message); err != nil {
			fmt.Fprintf(os.Stderr, "Failed to send: %v\n", err)
			os.Exit(1)
		}
		fmt.Println("Message sent successfully")

	case "read":
		target := ""
		limit := 10
		if len(os.Args) > 2 {
			target = os.Args[2]
		}
		if len(os.Args) > 3 {
			fmt.Sscanf(os.Args[3], "%d", &limit)
		}
		messages, err := client.Read(target, limit)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Failed to read: %v\n", err)
			os.Exit(1)
		}
		jsonBytes, _ := json.MarshalIndent(messages, "", "  ")
		fmt.Println(string(jsonBytes))

	case "poll":
		target := ""
		if len(os.Args) > 2 {
			target = os.Args[2]
		}
		fmt.Println("Polling for messages... (Ctrl+C to exit)")
		client.PollMessages(target, 10*time.Second, func(msg WhatsAppMessage) {
			fmt.Printf("📱 From %s: %s\n", msg.From, msg.Body)
		})

	default:
		fmt.Printf("Unknown command: %s\n", command)
		os.Exit(1)
	}
}