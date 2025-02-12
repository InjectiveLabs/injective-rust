#!/bin/bash

# Directory containing .proto files
PROTO_DIR="../../dependencies/injective-core/proto" # Update this to your proto directory

# Regular expression for camel case (multi-word)
CAMEL_CASE_PATTERN='^[a-z][a-zA-Z0-9]*([A-Z][a-zA-Z0-9]+)+$'

# Array to hold camel case fields
camel_case_fields=()

# Function to check if a field is camel case and not a single word
check_camel_case() {
    local field_name="$1"
    if [[ "$field_name" =~ $CAMEL_CASE_PATTERN ]] && [[ "${#field_name}" -gt 1 ]]; then
        echo "$field_name"
    fi
}

# Function to process each line in a file
process_file() {
    local proto_file="$1"
    echo "Processing file: $proto_file"

    # Process every line to find potential camel case field names
    while IFS= read -r line; do
        # Remove comments
        cleaned_line=$(echo "$line" | sed 's/#.*//')
        
        # Extract potential field names by splitting on whitespace
        for word in $cleaned_line; do
            # Check if the word matches the camel case pattern
            camel_case_field=$(check_camel_case "$word")
            if [[ -n "$camel_case_field" && "$word" != "optional" && "$word" != "required" && "$word" != "repeated" && "$word" != "map" && "$word" != "=" && "$word" != "[" && "$word" != "]" ]]; then
                echo "File: $proto_file - Camel Case Field: $camel_case_field"
                camel_case_fields+=("$camel_case_field")
            fi
        done
    done < "$proto_file"
}

# Find all .proto files in the directory and process each one
for proto_file in $(find "$PROTO_DIR" -type f -name "*.proto"); do
    process_file "$proto_file"
done

echo "Original array:"
echo "${camel_case_fields[@]}"

# Remove duplicate entries
unique_array=($(printf "%s\n" "${camel_case_fields[@]}" | sort -u))

# Print found camel case fields
echo "Found camel case fields:"
printf "%s\n" "${unique_array[@]}"
