#!/bin/bash
# Hacker News cross-posting
# Usage: ./hn-post.sh <title> <url>

TITLE="$1"
URL="$2"

if [ -z "$TITLE" ] || [ -z "$URL" ]; then
    echo "Usage: $0 <title> <url>"
    echo "Example: $0 'My Article' 'https://example.com/article'"
    exit 1
fi

title_encoded() {
    echo "$1" | sed 's/ /%20/g; s/\//%2F/g; s/:/%3A/g; s/?/%3F/g'
}

TITLE_ENC=$(title_encoded "$TITLE")
URL_ENC=$(title_encoded "$URL")

echo "Hacker News submit page:"
echo "https://news.ycombinator.com/submit?title=${TITLE_ENC}&url=${URL_ENC}"
