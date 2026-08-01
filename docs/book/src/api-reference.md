# API Reference

## REST API (axum)

Base URL: `http://localhost:3000/api/v1`

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/system` | GET | Node info |
| `/peers` | GET | List mesh peers |
| `/messages` | POST | Send encrypted message |
| `/storage/stats` | GET | Storage usage |
| `/ai/infer` | POST | Run inference |
| `/compute/tasks` | POST | Submit task |

Full spec: `proto/openapi.yaml`

## gRPC API

Service: `wios.v1.WiosService`

| RPC | Request | Response |
|-----|---------|----------|
| HealthCheck | HealthCheckRequest | HealthCheckResponse |
| GetNodeInfo | NodeInfoRequest | NodeInfoResponse |
| ListPeers | ListPeersRequest | ListPeersResponse |
| SendMessage | SendMessageRequest | SendMessageResponse |
| GetStorageStats | StorageStatsRequest | StorageStatsResponse |
| RunInference | InferenceRequest | InferenceResponse |
| SubmitTask | SubmitTaskRequest | SubmitTaskResponse |

Full spec: `proto/wios/v1/wios.proto`
