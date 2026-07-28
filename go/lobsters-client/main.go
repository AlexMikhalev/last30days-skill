package main

import (
	"encoding/xml"
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
	"time"
)

// RSS structures
type RSS struct {
	Channel Channel `xml:"channel"`
}

type Channel struct {
	Title   string `xml:"title"`
	Link    string `xml:"link"`
	Items   []Item `xml:"item"`
}

type Item struct {
	Title     string `xml:"title"`
	Link      string `xml:"link"`
	Guid      string `xml:"guid"`
	Author    string `xml:"author"`
	PubDate   string `xml:"pubDate"`
	Category string `xml:"category"`
}

type Client struct {
	http *http.Client
}

func NewClient() *Client {
	return &Client{
		http: &http.Client{Timeout: 30 * time.Second},
	}
}

// GetRecent fetches recent stories via RSS
func (c *Client) GetRecent(limit int) ([]Item, error) {
	url := "https://lobste.rs/rss"
	resp, err := c.http.Get(url)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	var rss RSS
	if err := xml.NewDecoder(resp.Body).Decode(&rss); err != nil {
		return nil, err
	}
	items := rss.Channel.Items
	if limit > 0 && len(items) > limit {
		items = items[:limit]
	}
	return items, nil
}

// GetByTag fetches stories by tag via RSS
func (c *Client) GetByTag(tag string, limit int) ([]Item, error) {
	url := fmt.Sprintf("https://lobste.rs/t/%s.rss", tag)
	resp, err := c.http.Get(url)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	var rss RSS
	if err := xml.NewDecoder(resp.Body).Decode(&rss); err != nil {
		return nil, err
	}
	items := rss.Channel.Items
	if limit > 0 && len(items) > limit {
		items = items[:limit]
	}
	return items, nil
}

// GetUser fetches user info (page scrape)
type UserInfo struct {
	Username string
	About   string
	Karma   int
}

func (c *Client) GetUser(username string) (*UserInfo, error) {
	url := fmt.Sprintf("https://lobste.rs/~%s", username)
	resp, err := c.http.Get(url)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	// Simple scrape for karma
	body, _ := io.ReadAll(resp.Body)
	s := string(body)

	var user UserInfo
	user.Username = username

	// Extract karma
	if idx := strings.Index(s, "Karma:</"); idx > 0 {
		end := strings.Index(s[idx:], "<")
		if end > 0 {
			karmaStr := s[idx+6 : idx+end]
			fmt.Sscanf(karmaStr, "%d", &user.Karma)
		}
	}

	return &user, nil
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: lobsters-client <command> [args...]")
		fmt.Println("  recent [n]    - Get recent stories (default 10)")
		fmt.Println("  tag <tag> [n] - Get stories by tag")
		fmt.Println("  user <name>   - Get user info")
		os.Exit(1)
	}

	client := NewClient()
	command := os.Args[1]

	switch command {
	case "recent":
		limit := 10
		if len(os.Args) > 2 {
			fmt.Sscanf(os.Args[2], "%d", &limit)
		}
		items, err := client.GetRecent(limit)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
		for _, item := range items {
			fmt.Printf("%s\n", item.Title)
			fmt.Printf("  by %s | %s\n", item.Author, item.Link)
			if item.Category != "" {
				fmt.Printf("  tag: %s\n", item.Category)
			}
			fmt.Println()
		}

	case "tag":
		if len(os.Args) < 3 {
			fmt.Println("Usage: lobsters-client tag <tag> [limit]")
			os.Exit(1)
		}
		tag := os.Args[2]
		limit := 10
		if len(os.Args) > 3 {
			fmt.Sscanf(os.Args[3], "%d", &limit)
		}
		items, err := client.GetByTag(tag, limit)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
		for _, item := range items {
			fmt.Printf("%s\n", item.Title)
			fmt.Printf("  by %s\n", item.Author)
			fmt.Println()
		}

	case "user":
		if len(os.Args) < 3 {
			fmt.Println("Usage: lobsters-client user <username>")
			os.Exit(1)
		}
		user, err := client.GetUser(os.Args[2])
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
		fmt.Printf("User: %s\n", user.Username)
		fmt.Printf("Karma: %d\n", user.Karma)

	default:
		fmt.Printf("Unknown command: %s\n", command)
		os.Exit(1)
	}
}