#!/bin/bash

YEAR=2024

if [ -f .env ]; then
  # Use the .env file if it exists, it should look like this:
  # SESSION_COOKIE=cookie
  # USER_AGENT = ...
  set -a
  source .env
  set +a
else
  echo ".env file not found. Exiting."
  exit 1
fi
#echo "Session cookie $SESSION_COOKIE"
#echo "User agent : $USER_AGENT"
# Check if DAY is provided as argument
if [ -z "$1" ]; then
  echo "No day provided. Would you like to use the current day? (Y/n)"
  read -r response
  # Default to Y if no response
  if [[ -z "$response" || "$response" =~ ^[Yy]$ ]]; then
    DAY=$(date +%d) # Use current day of the month
    DAY=$((10#$DAY))
    echo "Using current day: $DAY"
  else
    echo "Exiting, no day selected."
    exit 1
  fi
else
  # Use provided DAY and remove leading zeros if present
  DAY=$((10#$1))
fi

PADDED_DAY=$(printf "%02d" "$DAY")
CRATE_NAME="day$PADDED_DAY"

cargo generate aoc-template --name $CRATE_NAME

# Download input
INPUT_URL="https://adventofcode.com/$YEAR/day/$DAY/input"
echo $INPUT_URL
INPUT_FILE="./$CRATE_NAME/input.txt"
curl --fail --cookie "session=$SESSION_COOKIE" "$INPUT_URL" --user-agent "$USER_AGENT" -o "$INPUT_FILE"
## Remove Trailing newline from input file
sed -i '${/^$/d}' "$INPUT_FILE"
