#!/bin/bash

if [[ -z "$AOC_SESSION_ID" ]]; then
    echo "Please define a 'AOC_SESSION_ID' environment variable."
    exit 1
fi


if [[ $# -ne 3 ]]; then
    echo "Please input exactly two arguments. Correct usage: bash query_input.bash <year> <day> <input_output_path>."
    exit 1
fi

YEAR=$1
DAY=$2
OUTPUT_FILE=$3

AOC_INPUT_URL="https://adventofcode.com/$YEAR/day/$DAY/input"
echo $AOC_INPUT_URL
curl "$AOC_INPUT_URL" --cookie "session=$AOC_SESSION_ID" > $OUTPUT_FILE
