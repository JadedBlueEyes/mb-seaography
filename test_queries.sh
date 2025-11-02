#!/bin/bash

# Test script for MusicBrainz GraphQL API
# Run this to verify all queries are working

BASE_URL="http://localhost:8001"

echo "Testing MusicBrainz GraphQL API at $BASE_URL"
echo "=============================================="
echo ""

# Test 1: Health check
echo "Test 1: Health check"
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{"query":"{ health }"}' | jq '.'
echo ""

# Test 2: Database connection
echo "Test 2: Database connection"
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{"query":"{ database_connected }"}' | jq '.'
echo ""

# Test 3: Artist count
echo "Test 3: Artist count"
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{"query":"{ artistCount }"}' | jq '.'
echo ""

# Test 4: Get single artist
echo "Test 4: Get single artist (Massive Attack, id: 4)"
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{"query":"{ artist(id: 4) { id name sort_name gid comment } }"}' | jq '.'
echo ""

# Test 5: Get multiple artists
echo "Test 5: Get multiple artists (limit: 3)"
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{"query":"{ artists(limit: 3) { id name sort_name } }"}' | jq '.'
echo ""

# Test 6: Artists with pagination
echo "Test 6: Artists with pagination (limit: 2, offset: 10)"
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{"query":"{ artists(limit: 2, offset: 10) { id name } }"}' | jq '.'
echo ""

# Test 7: Get single area
echo "Test 7: Get single area (Afghanistan, id: 1)"
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{"query":"{ area(id: 1) { id name gid type } }"}' | jq '.'
echo ""

# Test 8: Get multiple areas
echo "Test 8: Get multiple areas (limit: 5)"
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{"query":"{ areas(limit: 5) { id name gid } }"}' | jq '.'
echo ""

# Test 9: Complex query with multiple fields
echo "Test 9: Complex artist query with all fields"
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{"query":"{ artist(id: 1) { id gid name sort_name comment type area gender begin_date_year end_date_year ended } }"}' | jq '.'
echo ""

# Test 10: Combined query
echo "Test 10: Combined query (health, count, and list)"
curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{"query":"{ health database_connected artistCount artists(limit: 2) { id name } }"}' | jq '.'
echo ""

echo "=============================================="
echo "All tests completed!"