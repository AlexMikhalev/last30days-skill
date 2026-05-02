#!/bin/bash
# Reddit cross-posting via browser automation
# Usage: ./reddit-post.sh <title> <url> [subreddit]

SUBREDDIT="${3:-programming}"
TITLE="$1"
URL="$2"

if [ -z "$TITLE" ] || [ -z "$URL" ]; then
    echo "Usage: $0 <title> <url> [subreddit]"
    echo "Example: $0 'My Post' 'https://example.com' programming"
    exit 1
fi

echo "Posting to r/$SUBREDDIT: $TITLE"
echo "URL: $URL"

# Use Camoufox to post to Reddit
# This creates a temporary script that will be executed
cat > /tmp/reddit-post.js << 'EOF'
const { chromium } = require('playwright');

(async () => {
    const browser = await chromium.launch({ headless: false });
    const page = await browser.newPage();
    
    // Navigate to Reddit submit page
    await page.goto('https://www.reddit.com/r/' + process.env.SUBREDDIT + '/submit');
    
    // Wait for login or submit form
    await page.waitForTimeout(2000);
    
    // Fill in the form
    const titleInput = await page.$('input[name="title"]');
    const urlInput = await page.$('input[name="url"]');
    
    if (titleInput) {
        await titleInput.fill(process.env.TITLE);
    }
    if (urlInput) {
        await urlInput.fill(process.env.URL);
    }
    
    console.log('Please complete the post manually');
    await browser.waitForTimeout(5000);
    await browser.close();
})();
EOF

echo "Opening browser for manual post..."
echo "Title: $TITLE"
echo "URL: $URL"
echo "Subreddit: r/$SUBREDDIT"

# Instead, let's use a simpler approach with direct URL
# Reddit's submit URL with pre-filled params
SUBMIT_URL="https://www.reddit.com/r/${SUBREDDIT}/submit?title=$(echo "$TITLE" | jq -rsr '@uri')&url=$(echo "$URL" | jq -rsr '@uri')"

echo "Navigate to: $SUBMIT_URL"

# Open in browser
camofox_create_tab "$SUBMIT_URL" 2>/dev/null || echo "$SUBMIT_URL"