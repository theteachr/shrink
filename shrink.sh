#!/bin/bash

URL=${1:-https://www.google.com}

curl -s -H "Content-Type: application/json" -d '{"uri":"'$URL'"}' http://localhost:3000
