#!/bin/bash

curl http://localhost:3000/ -X GET -v

echo ""
echo "-----------------------------"

curl http://localhost:3000/vehicle -X GET -v

echo ""
echo "-----------------------------"

curl http://localhost:3000/vehicle -X POST \
    -d '{"manufacturer": "Narayan Ltd", "model": "Fancy Styles", "year": 2024}' \
    -H "Content-Type: application/json" -v

echo ""
echo "-----------------------------"