# Flutter Frontend

## Pages

| Page | Description |
|------|-------------|
| Dashboard | System overview, stats, mesh visualization, activity feed |
| Auth | Login/register with E2EE, MFA/TOTP verification |
| Mesh | Peer topology, signal bars, scan controls |
| Messages | E2E encrypted chat with channel/topic tabs |
| Storage | Usage gauge, quota breakdown, recent files |
| AI Engine | Model list, inference stats, local chat |
| Compute | CPU/RAM/GPU gauges, task progress, node capabilities |
| Sensing | RF heatmap, beacon list, indoor positioning |
| Settings | Security toggles, network config, system info |

## Packages

| Package | Purpose |
|---------|---------|
| `wios_ui` | Design system, theme, reusable widgets |
| `wios_domain` | BLoCs, repository interfaces, use cases |
| `wios_data` | Repository implementations, data sources |
| `wios_common` | Shared models, constants, utilities |

## State Management
Uses BLoC pattern with `flutter_bloc`. Each feature has its own BLoC with events and states.
