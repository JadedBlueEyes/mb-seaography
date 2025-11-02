# GraphQL API Documentation

## Overview

The MusicBrainz GraphQL server is now running at `http://localhost:8001` with a working database connection.

## Available Queries

### Health Check Queries

#### `health`
Simple health check that returns "OK"

**Example:**
```graphql
{
  health
}
```

**Response:**
```json
{
  "data": {
    "health": "OK"
  }
}
```

#### `database_connected`
Checks if the database connection is active

**Example:**
```graphql
{
  database_connected
}
```

**Response:**
```json
{
  "data": {
    "database_connected": true
  }
}
```

### Artist Queries

#### `artist(id: Int!)`
Get a single artist by ID

**Arguments:**
- `id` (Int!, required): The artist ID

**Example:**
```graphql
{
  artist(id: 4) {
    id
    name
    sort_name
    gid
    comment
    type
    area
    gender
    begin_date_year
    end_date_year
    ended
  }
}
```

**Response:**
```json
{
  "data": {
    "artist": {
      "id": 4,
      "name": "Massive Attack",
      "sort_name": "Massive Attack",
      "gid": "10adbe5e-a2c0-4bf3-8249-2b4cbf6e6ca8",
      "comment": "",
      "type": null,
      "area": null,
      "gender": null,
      "begin_date_year": null,
      "end_date_year": null,
      "ended": false
    }
  }
}
```

#### `artists(limit: Int, offset: Int)`
Get a list of artists with pagination

**Arguments:**
- `limit` (Int, optional): Maximum number of results (default: 10)
- `offset` (Int, optional): Number of results to skip (default: 0)

**Example:**
```graphql
{
  artists(limit: 3, offset: 0) {
    id
    name
    sort_name
  }
}
```

**Response:**
```json
{
  "data": {
    "artists": [
      {
        "id": 1,
        "name": "Various Artists",
        "sort_name": "Various Artists"
      },
      {
        "id": 4,
        "name": "Massive Attack",
        "sort_name": "Massive Attack"
      },
      {
        "id": 6,
        "name": "Apartment 26",
        "sort_name": "Apartment 26"
      }
    ]
  }
}
```

#### `artistCount`
Get the total number of artists in the database

**Example:**
```graphql
{
  artistCount
}
```

**Response:**
```json
{
  "data": {
    "artistCount": 273925
  }
}
```

### Area Queries

#### `area(id: Int!)`
Get a single area (geographic location) by ID

**Arguments:**
- `id` (Int!, required): The area ID

**Example:**
```graphql
{
  area(id: 1) {
    id
    name
    gid
    type
  }
}
```

**Response:**
```json
{
  "data": {
    "area": {
      "id": 1,
      "name": "Afghanistan",
      "gid": "aa95182f-df0a-3ad6-8bfb-4b63482cd276",
      "type": 1
    }
  }
}
```

#### `areas(limit: Int, offset: Int)`
Get a list of areas with pagination

**Arguments:**
- `limit` (Int, optional): Maximum number of results (default: 10)
- `offset` (Int, optional): Number of results to skip (default: 0)

**Example:**
```graphql
{
  areas(limit: 5, offset: 0) {
    id
    name
    gid
  }
}
```

**Response:**
```json
{
  "data": {
    "areas": [
      {
        "id": 1,
        "name": "Afghanistan",
        "gid": "aa95182f-df0a-3ad6-8bfb-4b63482cd276"
      },
      {
        "id": 2,
        "name": "Albania",
        "gid": "1c69b790-b46b-3e92-b6b4-93b4364badbc"
      },
      {
        "id": 3,
        "name": "Algeria",
        "gid": "28242750-534a-326b-8ed6-1b03dfb88cd0"
      },
      {
        "id": 4,
        "name": "American Samoa",
        "gid": "e228a3c1-53c0-3ec9-842b-ec1b2138e387"
      },
      {
        "id": 5,
        "name": "Andorra",
        "gid": "e01da61e-99a8-3c76-a27d-774c3f4982f0"
      }
    ]
  }
}
```

## Testing the API

### Using cURL

```bash
curl -X POST http://localhost:8001 \
  -H "Content-Type: application/json" \
  -d '{"query":"{ artists(limit: 3) { id name sort_name } }"}'
```

### Using GraphQL Playground

Visit `http://localhost:8001` in your browser to access the interactive GraphQL Playground where you can explore the schema and test queries.

## Type Schemas

### Artist Type

```graphql
type Artist {
  id: Int!
  gid: String!
  name: String!
  sort_name: String!
  comment: String!
  type: Int
  area: Int
  gender: Int
  begin_date_year: Int
  end_date_year: Int
  ended: Boolean!
}
```

### Area Type

```graphql
type Area {
  id: Int!
  gid: String!
  name: String!
  type: Int
}
```

## Notes

- All field names use `snake_case` (e.g., `sort_name`, not `sortName`)
- The `gid` field is a UUID represented as a string
- Pagination is available on list queries using `limit` and `offset` parameters
- The database contains 273,925+ artists and thousands of areas
- The server uses SeaORM for database access and async-graphql for the GraphQL layer

## Future Enhancements

To expand this API, you can:

1. Add more entity types (releases, recordings, labels, etc.)
2. Implement relationship queries between entities
3. Add search/filter capabilities
4. Implement mutations for data modification
5. Add more complex queries with joins
6. Re-enable full seaography integration for automatic schema generation

## Troubleshooting

If queries fail:

1. Check that the database is running and accessible
2. Verify the `DATABASE_URL` environment variable is set correctly
3. Check server logs for detailed error messages
4. Ensure the server is running on port 8001