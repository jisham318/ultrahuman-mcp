# Ultrahuman MCP Server

A Rust MCP server for the Ultrahuman Daily API. It exposes Ultrahuman health metrics to MCP clients over stdio using the official `rmcp` SDK.

## Features

- Fetch complete daily metrics for an authorized Ultrahuman user.
- Fetch daily metrics for a default email configured in `.env`.
- Return structured MCP tool output as `{ email, date, metrics }`.
- Validate dates locally before calling the API.

## Tools

### `get_daily_metrics`

Fetch complete daily metrics for an authorized user.

Arguments:

- `email`: authorized Ultrahuman user email
- `date`: date in `YYYY-MM-DD` format

### `get_default_daily_metrics`

Fetch complete daily metrics for `ULTRAHUMAN_DEFAULT_EMAIL`.

Arguments:

- `date`: date in `YYYY-MM-DD` format

## Setup

Create a local `.env` file:

```dotenv
ULTRAHUMAN_API_TOKEN=your_token_here
ULTRAHUMAN_DEFAULT_EMAIL=you@example.com
```

Supported credential variables:

- `ULTRAHUMAN_AUTHORIZATION`
- `ULTRAHUMAN_API_TOKEN`
- `ULTRAHUMAN_AUTH_KEY`

Optional variables:

- `ULTRAHUMAN_DEFAULT_EMAIL`
- `ULTRAHUMAN_BASE_URL`, defaults to `https://partner.ultrahuman.com/api/v1`

`.env` is ignored by git.

## Run Locally

```nu
cargo run
```

## Build

```nu
cargo build --release
```

The release binary is written to:

```text
target/release/ultrahuman-mcp
```

## MCP Client Configuration

Use stdio transport and run the binary from this repository directory so `.env` can be loaded.

Example Cursor or Claude Desktop server entry:

```json
{
  "command": "/Users/isham/repos/ultrahuman-mcp/target/release/ultrahuman-mcp",
  "cwd": "/Users/isham/repos/ultrahuman-mcp"
}
```

## API

This server calls the Ultrahuman Daily API endpoint:

```text
GET https://partner.ultrahuman.com/api/v1/metrics?email=<email>&date=<YYYY-MM-DD>
```

The API requires an `Authorization` header containing your Ultrahuman token.
