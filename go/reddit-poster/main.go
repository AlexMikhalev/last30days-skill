package main

import (
	"fmt"
	"net/http"
	"net/url"
	"os"
	"time"
)

// RedditPoster posts to Reddit via browser automation
type RedditPoster struct {
	http       *http.Client
	username  string
	password  string
}

// NewRedditPoster creates a new Reddit poster
func NewRedditPoster() *RedditPoster {
	return &RedditPoster{
		http: &http.Client{
			Timeout: 30 * time.Second,
		},
	}
}

// LoadConfig loads credentials from env
func (p *RedditPoster) LoadConfig() {
	p.username = os.Getenv("REDDIT_USERNAME")
	p.password = os.Getenv("REDDIT_PASSWORD")
}

// GenerateSubmitURL generates a Reddit submit URL with pre-filled fields
func (p *RedditPoster) GenerateSubmitURL(subreddit, title, postURL string, isText bool, textContent string) string {
	v := url.Values{}
	v.Set("title", title)
	
	if isText && textContent != "" {
		v.Set("text", textContent)
	} else if postURL != "" {
		v.Set("url", postURL)
	}
	
	baseURL := fmt.Sprintf("https://www.reddit.com/r/%s/submit", subreddit)
	if query := v.Encode(); query != "" {
		return baseURL + "?" + query
	}
	return baseURL
}

// GenerateShareURL generates a share link for easy sharing
func (p *RedditPoster) GenerateShareURL(subreddit, title, postURL string) string {
	// Direct link that opens Reddit with the submit page
	v := url.Values{}
	v.Set("title", title)
	v.Set("url", postURL)
	
	return fmt.Sprintf("https://www.reddit.com/r/%s/submit?%s", subreddit, v.Encode())
}

// CheckLogin checks if we're logged in (makes a request to verify)
func (p *RedditPoster) CheckLogin() (bool, error) {
	req, _ := http.NewRequest("GET", "https://www.reddit.com/api/v1/me", nil)
	resp, err := p.http.Do(req)
	if err != nil {
		return false, err
	}
	defer resp.Body.Close()
	
	return resp.StatusCode == 200, nil
}

// GetSubredditRules gets subreddit rules (to check if link is allowed)
func (p *RedditPoster) GetSubredditRules(subreddit string) ([]string, error) {
	req, _ := http.NewRequest("GET", fmt.Sprintf("https://www.reddit.com/r/%s/about/rules.json", subreddit), nil)
	resp, err := p.http.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()
	
	// Return empty for now - need auth to read rules
	return []string{}, nil
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: reddit-poster <command> [args...]")
		fmt.Println()
		fmt.Println("Commands:")
		fmt.Println("  url <subreddit> <title> <url>      - Generate submit URL")
		fmt.Println("  check                           - Check login status")
		fmt.Println("  open <subreddit> <title> <url>  - Open submit page in browser")
		fmt.Println()
		fmt.Println("Environment:")
		fmt.Println("  REDDIT_USERNAME, REDDIT_PASSWORD")
		os.Exit(1)
	}

	poster := NewRedditPoster()
	poster.LoadConfig()
	
	command := os.Args[1]
	
	switch command {
	case "url":
		if len(os.Args) < 5 {
			fmt.Println("Usage: reddit-poster url <subreddit> <title> <url>")
			os.Exit(1)
		}
		subreddit := os.Args[2]
		title := os.Args[3]
		postURL := os.Args[4]
		
		submitURL := poster.GenerateSubmitURL(subreddit, title, postURL, false, "")
		fmt.Println("Submit URL:")
		fmt.Println(submitURL)
		
	case "open":
		if len(os.Args) < 5 {
			fmt.Println("Usage: reddit-poster open <subreddit> <title> <url>")
			os.Exit(1)
		}
		subreddit := os.Args[2]
		title := os.Args[3]
		postURL := os.Args[4]
		
		submitURL := poster.GenerateSubmitURL(subreddit, title, postURL, false, "")
		fmt.Println("Generated submit URL:")
		fmt.Println(submitURL)
		fmt.Println()
		fmt.Println("I'll open this in the browser for you...")
		
		// The actual opening will be done by the caller
		os.Stdout.Write([]byte(submitURL))
		
	case "check":
		loggedIn, err := poster.CheckLogin()
		if err != nil {
			fmt.Printf("Error checking login: %v\n", err)
			os.Exit(1)
		}
		if loggedIn {
			fmt.Println("✓ Logged in to Reddit")
		} else {
			fmt.Println("✗ Not logged in")
		}
		
	default:
		fmt.Printf("Unknown command: %s\n", command)
		os.Exit(1)
	}
}