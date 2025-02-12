#!/bin/bash

ALIAS=${1:-short}
URL=${2:-https://www.google.com}

curl -s -X PUT -H "Content-Type: application/json" -d '{"url":"'$URL'","alias":"'$ALIAS'"}' ${SERVER:-localhost:3000}
