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


curl -X POST -v "http://localhost:3000/vehicle2?manufacturer=Chandra&model=BP%20Sweaters&year=2024&first_name=Bandana&last_name=Devi"

echo ""
echo "-----------------------------"