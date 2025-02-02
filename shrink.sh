#!/bin/bash

URL=${1:-https://www.google.com}

curl -s -H "Content-Type: application/json" -d '{"url":"'$URL'"}' ${SERVER:-localhost:3000}
