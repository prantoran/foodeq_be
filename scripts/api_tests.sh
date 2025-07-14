#!/bin/bash

curl http://localhost:3000/ -X GET -v
# Expected output: "Hello, World!"

echo ""
echo "-----------------------------"

curl http://localhost:3000/vehicle -X GET -v
# Expected output: "Vehicle GET endpoint"

echo ""
echo "-----------------------------"

curl http://localhost:3000/vehicle -X POST -v
# Expected output: "Vehicle GET endpoint"

echo ""
echo "-----------------------------"