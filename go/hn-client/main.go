package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"time"
)

// HNItem represents a Hacker News item
type HNItem struct {
	ID        int    `json:"id"`
	Type      string `json:"type"`
	By        string `json:"by"`
	Time      int    `json:"time"`
	Title     string `json:"title"`
	URL       string `json:"url"`
	Score     int    `json:"score"`
	Descendants int  `json:"descendants"`
	Text      string `json:"text"`
	Parent    int    `json:"parent"`
	Kids      []int  `json:"kids"`
}

type Client struct {
	http *http.Client
}

func NewClient() *Client {
	return &Client{
		http: &http.Client{Timeout: 30 * time.Second},
	}
}

// GetTopStories returns top story IDs
func (c *Client) GetTopStories(limit int) ([]int, error) {
	url := fmt.Sprintf("https://hacker-news.firebaseio.com/v0/topstories.json")
	resp, err := c.http.Get(url)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	var ids []int
	if err := json.NewDecoder(resp.Body).Decode(&ids); err != nil {
		return nil, err
	}
	if limit > 0 && len(ids) > limit {
		ids = ids[:limit]
	}
	return ids, nil
}

// GetItem returns a single item
func (c *Client) GetItem(id int) (*HNItem, error) {
	url := fmt.Sprintf("https://hacker-news.firebaseio.com/v0/item/%d.json", id)
	resp, err := c.http.Get(url)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	var item HNItem
	if err := json.NewDecoder(resp.Body).Decode(&item); err != nil {
		return nil, err
	}
	return &item, nil
}

// GetTopStoriesWithDetails returns top stories with full details
func (c *Client) GetTopStoriesWithDetails(limit int) ([]HNItem, error) {
	ids, err := c.GetTopStories(limit)
	if err != nil {
		return nil, err
	}

	stories := make([]HNItem, 0, len(ids))
	for _, id := range ids {
		item, err := c.GetItem(id)
		if err != nil {
			continue
		}
		if item.Title != "" {
			stories = append(stories, *item)
		}
	}
	return stories, nil
}

// GetNewStories returns newest stories
func (c *Client) GetNewStories(limit int) ([]int, error) {
	url := "https://hacker-news.firebaseio.com/v0/newstories.json"
	resp, err := c.http.Get(url)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	var ids []int
	if err := json.NewDecoder(resp.Body).Decode(&ids); err != nil {
		return nil, err
	}
	if limit > 0 && len(ids) > limit {
		ids = ids[:limit]
	}
	return ids, nil
}

// GetUser returns user info
type HNUser struct {
	ID        string `json:"id"`
	Created  int   `json:"created"`
	Karma    int   `json:"karma"`
	About   string `json:"about"`
	Submissions []int `json:"submissions"`
}

func (c *Client) GetUser(username string) (*HNUser, error) {
	url := fmt.Sprintf("https://hacker-news.firebaseio.com/v0/user/%s.json", username)
	resp, err := c.http.Get(url)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	var user HNUser
	if err := json.NewDecoder(resp.Body).Decode(&user); err != nil {
		return nil, err
	}
	return &user, nil
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: hn-client <command> [args...]")
		fmt.Println("  top [n]       - Get top stories (default 10)")
		fmt.Println("  new [n]       - Get new stories")
		fmt.Println("  item <id>      - Get item details")
		fmt.Println("  user <name>   - Get user info")
		os.Exit(1)
	}

	client := NewClient()
	command := os.Args[1]

	switch command {
	case "top":
		limit := 10
		if len(os.Args) > 2 {
			fmt.Sscanf(os.Args[2], "%d", &limit)
		}
		stories, err := client.GetTopStoriesWithDetails(limit)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
		for i, s := range stories {
			fmt.Printf("[%d] %s\n", i+1, s.Title)
			fmt.Printf("    by %s | %d points | %d comments\n", s.By, s.Score, s.Descendants)
			if s.URL != "" {
				fmt.Printf("    %s\n", s.URL)
			}
			fmt.Println()
		}

	case "new":
		limit := 10
		if len(os.Args) > 2 {
			fmt.Sscanf(os.Args[2], "%d", &limit)
		}
		ids, err := client.GetNewStories(limit)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
		for _, id := range ids {
			item, _ := client.GetItem(id)
			if item != nil && item.Title != "" {
				fmt.Printf("[%d] %s\n", id, item.Title)
				fmt.Printf("    by %s | %d points\n", item.By, item.Score)
				if item.URL != "" {
					fmt.Printf("    %s\n", item.URL)
				}
				fmt.Println()
			}
		}

	case "item":
		if len(os.Args) < 3 {
			fmt.Println("Usage: hn-client item <id>")
			os.Exit(1)
		}
		var id int
		fmt.Sscanf(os.Args[2], "%d", &id)
		item, err := client.GetItem(id)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
		fmt.Printf("Title: %s\n", item.Title)
		fmt.Printf("By: %s\n", item.By)
		fmt.Printf("Score: %d\n", item.Score)
		fmt.Printf("URL: %s\n", item.URL)
		if item.Text != "" {
			fmt.Printf("Text: %s\n", item.Text)
		}

	case "user":
		if len(os.Args) < 3 {
			fmt.Println("Usage: hn-client user <username>")
			os.Exit(1)
		}
		user, err := client.GetUser(os.Args[2])
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
		fmt.Printf("User: %s\n", user.ID)
		fmt.Printf("Karma: %d\n", user.Karma)
		fmt.Printf("About: %s\n", user.About)

	default:
		fmt.Printf("Unknown command: %s\n", command)
		os.Exit(1)
	}
}