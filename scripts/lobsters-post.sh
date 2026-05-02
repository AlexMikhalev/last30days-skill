#!/bin/bash
# Lobsters cross-posting
# Usage: ./lobsters-post.sh <title> <url> [tag(s)]

TITLE="$1"
URL="$2"
TAGS="${3:-}"

if [ -z "$TITLE" ] || [ -z "$URL" ]; then
    echo "Usage: $0 <title> <url> [tags]"
    echo "Example: $0 'My Article' 'https://example.com/article' 'rust,show'"
    exit 1
fi

# Simple URL encoding
title_encoded() {
    echo "$1" | sed 's/ /%20/g; s/\//%2F/g; s/:/%3A/g; s/?/%3F/g; s/=/%3D/g; s/#/%23/g; s/&/%26/g'
}

TITLE_ENC=$(title_encoded "$TITLE")
URL_ENC=$(title_encoded "$URL")

SUBMIT_URL="https://lobste.rs/s/new?title=${TITLE_ENC}&url=${URL_ENC}"

if [ -n "$TAGS" ]; then
    TAGS_ENC=$(title_encoded "$TAGS")
    SUBMIT_URL="${SUBMIT_URL}&tags=${TAGS_ENC}"
fi

echo "Lobsters submit page:"
echo "$SUBMIT_URL"
