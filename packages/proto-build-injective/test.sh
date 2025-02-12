#!/bin/bash

# Example array with potential camel case field names
camel_case_fields=("auctionRound" "auctionClosingTime" "highestBidder" "highestBidAmount" "auctionRound" "highestBidAmount")

# Print the original array
echo "Original array:"
echo "${camel_case_fields[@]}"

# Remove duplicate entries
unique_array=($(printf "%s\n" "${camel_case_fields[@]}" | sort -u))

# Print found camel case fields
echo "Found camel case fields:"
printf "%s\n" "${unique_array[@]}"