# Space Traders SDK

A rust implementation of <https://spacetraders.io/>

Idea

SDK

- List All Public Agents
- Get Public Details For Agent
-

Account

- get-account
- register-agent

Agent

- get-agent
- get-events
- list-contracts
- get-contract

Contract

- accept
- fulfill
- deliver

## System

### List Systems

Endpoint: <https://spacetraders.io/openapi#tag/systems/GET/systems>

Function:

```rust
System::list_systems(page, limit) -> Vec<System>
```

### Get System

Endpoint: <https://spacetraders.io/openapi#tag/systems/GET/systems/{systemSymbol}>

Function:

```rust
system.get_data() -> SystemData

System::get_system_data(client, symbol) -> SystemData
```

### List Waypoints In A System

Endpoint: <https://spacetraders.io/openapi#tag/systems/GET/systems/{systemSymbol}/waypoints>

Function:

```rust
system.list_waypoints(waypoint_type, waypoint_trait) -> Vec<Waypoint>

System::list_system_waypoints(client, symbol, waypoint_type, waypoint_trait) -> Vec<Waypoint>
```

### Waypoint

#### Get Waypoint

Endpoint: <https://spacetraders.io/openapi#tag/systems/GET/systems/{systemSymbol}/waypoints/{waypointSymbol}>

```rust
waypoint.get_data() -> WaypointData

Waypoint::get_waypoint_data(client, system_symbol, symbol) -> WaypointData
```

#### Get Construction Site

Endpoint: <https://spacetraders.io/openapi#tag/systems/GET/systems/{systemSymbol}/waypoints/{waypointSymbol}/construction>

Unimplemented

#### Supply Construction Site

Endpoint: <https://spacetraders.io/openapi#tag/systems/POST/systems/{systemSymbol}/waypoints/{waypointSymbol}/construction/supply>

Unimplemented

#### Get Market

Endpoint: <https://spacetraders.io/openapi#tag/systems/GET/systems/{systemSymbol}/waypoints/{waypointSymbol}/market>

```rust
waypoint.get_market() -> Market

Waypoint::get_waypoint_market(client, system_symbol, symbol) -> Market
```

#### Get Jump Gate

Endpoint: <https://spacetraders.io/openapi#tag/systems/GET/systems/{systemSymbol}/waypoints/{waypointSymbol}/jump-gate>

Unimplemented

#### Get Shipyard

Endpoint: <https://spacetraders.io/openapi#tag/systems/GET/systems/{systemSymbol}/waypoints/{waypointSymbol}/shipyard>

```rust
waypoint.get_shipyard() -> Shipyard

Waypoint::get_waypoint_shipyard(client, system_symbol, symbol) -> Shipyard

```
