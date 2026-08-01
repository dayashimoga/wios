# WIOS API Reference

## REST API (v1)

Base URL: `http://localhost:8080/api/v1`

### Health Check
```
GET /api/v1/health
```
Response: `{ "status": "healthy", "timestamp": "..." }`

### System Info
```
GET /api/v1/info
```
Response: `{ "name": "WIOS", "version": "0.1.0", "platform": "...", "uptime_secs": 0 }`

### Nodes
```
GET /api/v1/nodes
```
Response: `{ "nodes": [...], "total": 0 }`

### Network Stats
```
GET /api/v1/network/stats
```

### Storage Stats
```
GET /api/v1/storage/stats
```

### AI Models
```
GET /api/v1/ai/models
```

### Compute Tasks
```
GET /api/v1/compute/tasks
```

## gRPC API
Protocol buffer definitions will be added in Phase 8.
