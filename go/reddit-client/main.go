package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
	"time"
)

// Config holds Reddit API settings
type Config struct {
	ClientID     string
	ClientSecret string
	Username     string
	Password     string
	UserAgent    string
	Subreddit    string
}

// RedditPost represents a Reddit post
type RedditPost struct {
	ID        string `json:"id"`
	Title     string `json:"title"`
	Selftext  string `json:"selftext,omitempty"`
	URL       string `json:"url,omitempty"`
	Subreddit string `json:"subreddit"`
	Author    string `json:"author"`
	Created   float64 `json:"created"`
	Score     int `json:"score"`
	NumComments int `json:"num_comments"`
	Permalink string `json:"permalink"`
	Link       string `json:"link"`
}

// RedditMessage represents a Reddit notification/message
type RedditMessage struct {
	ID            string `json:"id"`
	Type         string `json:"type"`
	Subject       string `json:"subject"`
	Body          string `json:"body"`
	Author        string `json:"author"`
	Created       float64 `json:"created"`
	Subreddit     string `json:"subreddit,omitempty"`
	Link          string `json:"link,omitempty"`
	New           bool   `json:"new"`
}

// RedditResponse generic reddit API response
type RedditResponse struct {
	JSON struct {
		Data struct {
			Children []struct {
				Data RedditPost `json:"data"`
			} `json:"children"`
		} `json:"data"`
	} `json:"data"`
}

// MessageResponse for inbox messages
type MessageResponse struct {
	JSON struct {
		Data struct {
			Children []struct {
				Data RedditMessage `json:"data"`
			} `json:"children"`
		} `json:"data"`
	} `json:"data"`
}

// Client wraps Reddit API operations
type Client struct {
	config    Config
	http     *http.Client
	token    string
	tokenEx  time.Time
}

// NewClient creates a new Reddit client
func NewClient() *Client {
	userAgent := "GoRedditClient/1.0"
	return &Client{
		config: Config{
			UserAgent: userAgent,
		},
		http: &http.Client{Timeout: 30 * time.Second},
	}
}

// loadConfig loads config from env vars
func (c *Client) loadConfig() {
	c.config.ClientID = os.Getenv("REDDIT_CLIENT_ID")
	c.config.ClientSecret = os.Getenv("REDDIT_CLIENT_SECRET")
	c.config.Username = os.Getenv("REDDIT_USERNAME")
	c.config.Password = os.Getenv("REDDIT_PASSWORD")
	c.config.Subreddit = os.Getenv("REDDIT_SUBREDDIT")
}

// authenticate authenticates with Reddit
func (c *Client) authenticate() error {
	c.loadConfig()
	if c.config.ClientID == "" || c.config.ClientSecret == "" {
		return fmt.Errorf("REDDIT_CLIENT_ID and REDDIT_CLIENT_SECRET required")
	}

	// Use refresh token if available, otherwise password grant
	refreshToken := os.Getenv("REDDIT_REFRESH_TOKEN")
	
	var tokenURL string
	var payload string
	
	if refreshToken != "" {
		tokenURL = "https://www.reddit.com/api/v1/access_token"
		payload = fmt.Sprintf("grant_type=refresh_token&refresh_token=%s", refreshToken)
	} else {
		tokenURL = "https://www.reddit.com/api/v1/access_token"
		payload = fmt.Sprintf("grant_type=password&username=%s&password=%s", 
			c.config.Username, c.config.Password)
	}

	req, _ := http.NewRequest("POST", tokenURL, strings.NewReader(payload))
	req.Header.Set("User-Agent", c.config.UserAgent)
	req.Header.Set("Content-Type", "application/x-www-form-urlform-urlencoded")
	req.SetBasicAuth(c.config.ClientID, c.config.ClientSecret)

	resp, err := c.http.Do(req)
	if err != nil {
		return fmt.Errorf("authentication failed: %w", err)
	}
	defer resp.Body.Close()

	var result struct {
		AccessToken string `json:"access_token"`
		TokenType   string `json:"token_type"`
		ExpiresIn  int    `json:"expires_in"`
		Scope      string `json:"scope"`
	}

	body, _ := io.ReadAll(resp.Body)
	if err := json.Unmarshal(body, &result); err != nil {
		return fmt.Errorf("failed to parse auth response: %w", err)
	}

	if result.AccessToken == "" {
		return fmt.Errorf("no access token received: %s", string(body))
	}

	c.token = result.AccessToken
	c.tokenEx = time.Now().Add(time.Duration(result.ExpiresIn-60) * time.Second)

	return nil
}

// ensureToken ensures we have a valid token
func (c *Client) ensureToken() error {
	if c.token == "" || time.Now().After(c.tokenEx) {
		if err := c.authenticate(); err != nil {
			return err
		}
	}
	return nil
}

// get makes an authenticated GET request
func (c *Client) get(endpoint string) ([]byte, error) {
	if err := c.ensureToken(); err != nil {
		return nil, err
	}

	url := "https://oauth.reddit.com" + endpoint
	req, _ := http.NewRequest("GET", url, nil)
	req.Header.Set("User-Agent", c.config.UserAgent)
	req.Header.Set("Authorization", "Bearer "+c.token)
	req.Header.Set("Accept", "application/json")

	resp, err := c.http.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	return io.ReadAll(resp.Body)
}

// post makes an authenticated POST request
func (c *Client) post(endpoint string, data map[string]string) ([]byte, error) {
	if err := c.ensureToken(); err != nil {
		return nil, err
	}

	payload := ""
	for k, v := range data {
		if payload != "" {
			payload += "&"
		}
		payload += k + "=" + v
	}

	url := "https://oauth.reddit.com" + endpoint
	req, _ := http.NewRequest("POST", url, strings.NewReader(payload))
	req.Header.Set("User-Agent", c.config.UserAgent)
	req.Header.Set("Authorization", "Bearer "+c.token)
	req.Header.Set("Content-Type", "application/x-www-form-urlencoded")
	req.Header.Set("Accept", "application/json")

	resp, err := c.http.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	return io.ReadAll(resp.Body)
}

// GetNewPosts gets new posts from a subreddit
func (c *Client) GetNewPosts(subreddit string, limit int) ([]RedditPost, error) {
	endpoint := fmt.Sprintf("/r/%s/new.json?limit=%d", subreddit, limit)
	body, err := c.get(endpoint)
	if err != nil {
		return nil, err
	}

	var resp RedditResponse
	if err := json.Unmarshal(body, &resp); err != nil {
		return nil, fmt.Errorf("failed to parse response: %w", err)
	}

	posts := make([]RedditPost, len(resp.JSON.Data.Children))
	for i, child := range resp.JSON.Data.Children {
		posts[i] = child.Data
		posts[i].Link = "https://reddit.com" + child.Data.Permalink
	}

	return posts, nil
}

// PostText creates a text post
func (c *Client) PostText(subreddit, title, text string) error {
	data := map[string]string{
		"sr":        subreddit,
		"kind":      "self",
		"title":     title,
		"text":      text,
		"api_type":  "json",
	}

	body, err := c.post("/api/submit", data)
	if err != nil {
		return err
	}

	var result struct {
		JSON struct {
			Errors [][]interface{} `json:"errors"`
		} `json:"json"`
	}
	json.Unmarshal(body, &result)

	if len(result.JSON.Errors) > 0 && len(result.JSON.Errors[0]) > 0 {
		return fmt.Errorf("post failed: %v", result.JSON.Errors)
	}

	return nil
}

// PostLink creates a link post
func (c *Client) PostLink(subreddit, title, url string) error {
	data := map[string]string{
		"sr":       subreddit,
		"kind":     "link",
		"title":    title,
		"url":      url,
		"api_type": "json",
	}

	_, err := c.post("/api/submit", data)
	return err
}

// GetInbox gets unread messages/notifications
func (c *Client) GetInbox(limit int) ([]RedditMessage, error) {
	endpoint := fmt.Sprintf("/message/inbox.json?limit=%d", limit)
	body, err := c.get(endpoint)
	if err != nil {
		return nil, err
	}

	var resp MessageResponse
	if err := json.Unmarshal(body, &resp); err != nil {
		return nil, fmt.Errorf("failed to parse response: %w", err)
	}

	messages := make([]RedditMessage, 0, len(resp.JSON.Data.Children))
	for _, child := range resp.JSON.Data.Children {
		if child.Data.New {
			messages = append(messages, child.Data)
		}
	}

	return messages, nil
}

// MarkRead marks a message as read
func (c *Client) MarkRead(msgID string) error {
	data := map[string]string{
		"id": msgID,
	}
	_, err := c.post("/api/read_all_messages", data)
	return err
}

// Upvote upvotes a post
func (c *Client) Upvote(subreddit, postID string) error {
	data := map[string]string{
		"id":  postID,
		"dir": "1",
	}
	_, err := c.post("/api/vote", data)
	return err
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: reddit-client <command> [args...]")
		fmt.Println("  Commands:")
		fmt.Println("    post-text <subreddit> <title> <text>  - Create text post")
		fmt.Println("    post-link <subreddit> <title> <url>    - Create link post")
		fmt.Println("    inbox [limit]                           - Check notifications")
		fmt.Println("    new <subreddit> [limit]                - Get new posts")
		fmt.Println("    upvote <subreddit> <post-id>          - Upvote a post")
		fmt.Println()
		fmt.Println("  Environment variables:")
		fmt.Println("    REDDIT_CLIENT_ID, REDDIT_CLIENT_SECRET")
		fmt.Println("    REDDIT_USERNAME, REDDIT_PASSWORD")
		fmt.Println("    REDDIT_SUBREDDIT")
		os.Exit(1)
	}

	client := NewClient()
	command := os.Args[1]

	// Skip auth for help commands
	if command == "help" || command == "--help" || command == "-h" {
		fmt.Println("Reddit Go Client")
		fmt.Println()
		main()
		return
	}

	switch command {
	case "post-text":
		if len(os.Args) < 5 {
			fmt.Println("Usage: reddit-client post-text <subreddit> <title> <text>")
			os.Exit(1)
		}
		subreddit := os.Args[2]
		title := os.Args[3]
		text := strings.Join(os.Args[4:], " ")
		if err := client.PostText(subreddit, title, text); err != nil {
			fmt.Fprintf(os.Stderr, "Failed to post: %v\n", err)
			os.Exit(1)
		}
		fmt.Println("Posted successfully")

	case "post-link":
		if len(os.Args) < 5 {
			fmt.Println("Usage: reddit-client post-link <subreddit> <title> <url>")
			os.Exit(1)
		}
		subreddit := os.Args[2]
		title := os.Args[3]
		url := os.Args[4]
		if err := client.PostLink(subreddit, title, url); err != nil {
			fmt.Fprintf(os.Stderr, "Failed to post: %v\n", err)
			os.Exit(1)
		}
		fmt.Println("Posted successfully")

	case "inbox":
		limit := 10
		if len(os.Args) > 2 {
			fmt.Sscanf(os.Args[2], "%d", &limit)
		}
		messages, err := client.GetInbox(limit)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Failed to get inbox: %v\n", err)
			os.Exit(1)
		}
		if len(messages) == 0 {
			fmt.Println("No new messages")
			return
		}
		for _, msg := range messages {
			fmt.Printf("[%s] From %s: %s\n", msg.Type, msg.Author, msg.Subject)
			fmt.Printf("  Body: %s\n", msg.Body)
			// Mark as read
			client.MarkRead(msg.ID)
		}

	case "new":
		subreddit := os.Getenv("REDDIT_SUBREDDIT")
		limit := 10
		if len(os.Args) > 2 {
			subreddit = os.Args[2]
		}
		if len(os.Args) > 3 {
			fmt.Sscanf(os.Args[3], "%d", &limit)
		}
		posts, err := client.GetNewPosts(subreddit, limit)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Failed to get posts: %v\n", err)
			os.Exit(1)
		}
		for _, post := range posts {
			fmt.Printf("[%d] %s\n", post.Score, post.Title)
			fmt.Printf("    by %s - %d comments\n", post.Author, post.NumComments)
			fmt.Printf("    %s\n\n", post.Link)
		}

	case "upvote":
		if len(os.Args) < 4 {
			fmt.Println("Usage: reddit-client upvote <subreddit> <post-id>")
			os.Exit(1)
		}
		subreddit := os.Args[2]
		postID := os.Args[3]
		if err := client.Upvote(subreddit, postID); err != nil {
			fmt.Fprintf(os.Stderr, "Failed to upvote: %v\n", err)
			os.Exit(1)
		}
		fmt.Println("Upvoted successfully")

	default:
		fmt.Printf("Unknown command: %s\n", command)
		os.Exit(1)
	}
}