# Inlay Hint Refresh and Resolve Event Analysis

## Overview
This document summarizes our investigation into inlay hint refresh and resolve events in the LSP protocol, specifically for rust-analyzer.

## Key Findings

### 1. Refresh Event Structure
We successfully captured a refresh event with the following structure:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "workspace/inlayHint/refresh"
}
```

**Important characteristics:**
- It's a **notification** (has an `id` but no `params`)
- The method is `workspace/inlayHint/refresh`
- It's triggered by document changes (`didChange` notifications)
- The client should respond to this notification

### 2. Triggering Refresh Events
We discovered that refresh events are triggered by:
1. **Document changes** (`textDocument/didChange` notifications)
2. **Workspace configuration changes** (`workspace/didChangeConfiguration`)
3. **Manual refresh requests** (`workspace/inlayHint/refresh`)

### 3. Resolve Event Structure
Resolve events follow this pattern:
```json
{
  "jsonrpc": "2.0",
  "id": 5,
  "method": "inlayHint/resolve",
  "params": {
    "position": {...},
    "label": [...],
    "kind": 1,
    "data": {...}
  }
}
```

**Important characteristics:**
- It's a **request** (has both `id` and `params`)
- The method is `inlayHint/resolve`
- It requires the original inlay hint data to resolve additional properties

### 4. Message Flow Patterns

#### Refresh Flow:
1. Document change occurs
2. Server sends `workspace/inlayHint/refresh` notification
3. Client should re-request inlay hints for affected documents
4. Server responds with updated inlay hints

#### Resolve Flow:
1. Client receives inlay hints with `data` field
2. Client sends `inlayHint/resolve` request with the data
3. Server responds with resolved inlay hint (additional properties like tooltip)

## Implementation Strategy

### For LSP Proxy:
1. **Intercept refresh notifications**: When we see `workspace/inlayHint/refresh`, we should:
   - Log the refresh event
   - Optionally trigger our own refresh logic
   - Pass through the notification unchanged

2. **Intercept resolve requests**: When we see `inlayHint/resolve`, we should:
   - Log the resolve request
   - Pass through the request unchanged
   - Intercept the resolve response to apply our transformations

3. **Intercept resolve responses**: When we see resolve responses, we should:
   - Apply our whippyunits type transformations
   - Preserve all metadata and additional properties

### Message Types to Handle:
- `workspace/inlayHint/refresh` (notification)
- `inlayHint/resolve` (request)
- `inlayHint/resolve` responses (response with additional properties)

## Test Results Summary

### Successful Captures:
- ✅ Refresh event triggered by `didChange`
- ✅ Refresh event structure analysis
- ✅ Notification message patterns
- ✅ Live event capture

### Key Insights:
1. **Refresh events are notifications, not requests** - they have an `id` but no `params`
2. **Document changes are the primary trigger** for refresh events
3. **Resolve events require the original inlay hint data** to work properly
4. **The LSP proxy should pass through refresh notifications** and intercept responses

## Next Steps

1. Update the LSP proxy to recognize and handle refresh notifications
2. Add support for intercepting resolve requests and responses
3. Ensure our type transformations work with resolved inlay hints
4. Test the complete refresh/resolve flow in a real editor environment

## Files Created
- `inlay_hint_refresh_test.rs` - Comprehensive test for refresh/resolve events
- `inlay_hint_refresh_messages.json` - Captured message examples
- `live_refresh_messages.json` - Live event captures
