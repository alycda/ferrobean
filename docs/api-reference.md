# API Reference

Ferrobean provides a RESTful API for beancount processing through Cloudflare Workers.

## Base URL

```
https://your-worker.your-subdomain.workers.dev/
```

## Endpoints

### GET /

Returns a simple greeting to verify the service is running.

**Request:**
```http
GET / HTTP/1.1
Host: your-worker.your-subdomain.workers.dev
```

**Response:**
```http
HTTP/1.1 200 OK
Content-Type: text/plain

Hello World!
```

## Data Types

### Transaction Flags

Ferrobean supports all standard beancount transaction flags:

| Flag | Character | Description |
|------|-----------|-------------|
| Conversion | `C` or `c` | Currency conversion transactions |
| Merging | `M` or `m` | Account merging operations |
| Okay | `*` | Posted/cleared transactions |
| Padding | `P` or `p` | Padding entries |
| Returns | `R` or `r` | Return transactions |
| Summarize | `S` or `s` | Summary entries |
| Transfer | `T` or `t` | Transfer transactions |
| Unrealized | `U` or `u` | Unrealized gains/losses |
| Warning | `!` | Transactions requiring attention |

### Error Types

The API uses structured error responses:

```rust
enum Helpers {
    BeancountError(String),  // Beancount-specific errors
    FavaError(String)        // Fava-related errors
}
```

## Request/Response Format

### Content Types

- **Request**: `application/json` (when applicable)
- **Response**: `application/json` or `text/plain`

### Error Responses

Error responses follow a consistent format:

```json
{
  "error": "BeancountError",
  "message": "conversion error: X is not a valid variant of Flags"
}
```

## Rate Limiting

Cloudflare Workers automatically handles rate limiting based on your plan:

- **Free Tier**: 100,000 requests per day
- **Paid Plans**: Higher limits available

## Authentication

Currently, Ferrobean operates without authentication. For production use, consider implementing:

- API key authentication
- JWT tokens
- Cloudflare Access integration

## Examples

### Basic Health Check

```bash
curl https://your-worker.your-subdomain.workers.dev/
```

### Using with JavaScript

```javascript
fetch('https://your-worker.your-subdomain.workers.dev/')
  .then(response => response.text())
  .then(data => console.log(data));
```

### Using with Python

```python
import requests

response = requests.get('https://your-worker.your-subdomain.workers.dev/')
print(response.text)
```

## Status Codes

| Code | Description |
|------|-------------|
| 200 | Success |
| 400 | Bad Request - Invalid input |
| 500 | Internal Server Error |

## Future API Endpoints

Planned endpoints for future releases:

- `POST /parse` - Parse beancount files
- `GET /validate` - Validate transaction flags
- `POST /convert` - Convert between formats