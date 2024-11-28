#!/bin/bash

YEAR=2024

if [ -f .env ]; then
  # Use the .env file if it exists, it should look like this:
  # SESSION_COOKIE=cookie
  export $(grep -v '^#' .env | xargs)
else
  echo ".env file not found. Exiting."
  exit 1
fi

# Check if DAY is provided as argument
if [ -z "$1" ]; then
  # No argument provided, prompt user to use the current day
  echo "No day provided. Would you like to use the current day? (Y/n)"
  read -r response
  # Default to Y if no response
  if [[ -z "$response" || "$response" =~ ^[Yy]$ ]]; then
    DAY=$(date +%d) # Use current day of the month
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
INPUT_URL="https://adventofcode.com/$YEAR/day/$DAY/input"
echo $INPUT_URL
INPUT_FILE="./$CRATE_NAME/input.txt"
curl --fail --cookie "session=$SESSION_COOKIE" "$INPUT_URL" -o "$INPUT_FILE"
