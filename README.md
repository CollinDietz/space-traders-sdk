# Space Traders SDK

A rust implementation of <https://spacetraders.io/>

- [Space Traders SDK](#space-traders-sdk)
  - [Account](#account)
  - [Get Account](#get-account)
  - [Register Agent](#register-agent)
  - [Agent](#agent)
    - [List Agents](#list-agents)
    - [Get Public Details For An Agent](#get-public-details-for-an-agent)
    - [Get Agent](#get-agent)
    - [Get Agent Events](#get-agent-events)
  - [Contracts](#contracts)
    - [List Contracts](#list-contracts)
    - [Get Contract](#get-contract)
    - [Accept Contract](#accept-contract)
    - [Fulfill contract](#fulfill-contract)
    - [Deliver Cargo To Contract](#deliver-cargo-to-contract)
    - [Negotiate Contract](#negotiate-contract)
  - [Faction](#faction)
    - [List Factions](#list-factions)
    - [Faction Details](#faction-details)
    - [Get My Factions](#get-my-factions)
  - [Fleet](#fleet)
    - [List Ships](#list-ships)
    - [Purchase Ships](#purchase-ships)
    - [Get Ship](#get-ship)
    - [Create Chart](#create-chart)
    - [Get Ship Cooldown](#get-ship-cooldown)
    - [Dock](#dock)
    - [Extract Resources](#extract-resources)
    - [Extract Resources with Survey](#extract-resources-with-survey)
    - [Jettison Cargo](#jettison-cargo)
    - [Jump Ship](#jump-ship)
    - [Scan Systems](#scan-systems)
    - [Scan Waypoints](#scan-waypoints)
    - [Scan Ships](#scan-ships)
    - [Scrap Ship](#scrap-ship)
    - [Get Scrap Ship](#get-scrap-ship)
    - [Navigate Ship](#navigate-ship)
    - [Warp Ship](#warp-ship)
    - [Orbit Ship](#orbit-ship)
    - [Purchase Cargo](#purchase-cargo)
    - [Ship Refine](#ship-refine)
    - [Refuel Ship](#refuel-ship)
    - [Repair Ship](#repair-ship)
    - [Get Repair Ship](#get-repair-ship)
    - [Sell Cargo](#sell-cargo)
    - [Siphon Resources](#siphon-resources)
    - [Create Survey](#create-survey)
    - [Transfer Cargo](#transfer-cargo)
    - [Get Ship Cargo](#get-ship-cargo)
    - [Get Ship Modules](#get-ship-modules)
    - [Install Ship Modules](#install-ship-modules)
    - [Remove Ship Modules](#remove-ship-modules)
    - [Get Mounts](#get-mounts)
    - [Install Mount](#install-mount)
    - [Remove Mount](#remove-mount)
    - [Get Ship Nav](#get-ship-nav)
    - [Patch Ship Nav](#patch-ship-nav)
  - [System](#system)
    - [List Systems](#list-systems)
    - [Get System](#get-system)
    - [List Waypoints In A System](#list-waypoints-in-a-system)
    - [Waypoint](#waypoint)
      - [Get Waypoint](#get-waypoint)
      - [Get Construction Site](#get-construction-site)
      - [Supply Construction Site](#supply-construction-site)
      - [Get Market](#get-market)
      - [Get Jump Gate](#get-jump-gate)
      - [Get Shipyard](#get-shipyard)
  - [Data](#data)
    - [Describes Trade Relationship](#describes-trade-relationship)
    - [Subscribe to Events](#subscribe-to-events)
  - [Global](#global)
    - [Server Status](#server-status)
    - [Error Code List](#error-code-list)

## Account

## Get Account

Endpoint: <https://spacetraders.io/openapi#tag/accounts/GET/my/account>

Unimplemented

Todo

- Implement: `agent.get_account()` and `Agent::get_account_data(client)`

## Register Agent

Endpoint: <https://spacetraders.io/openapi#tag/accounts/POST/register>

```rust
account.register_agent(request)
```

Todo

- Make `Account::register_new_agent()`
- Make function sig `(callsign, faction)`

## Agent

### List Agents

Endpoint: <https://spacetraders.io/openapi#tag/agents/GET/agents>

Unimplemented

Todo

- Implement `Agent::list_agents()`

### Get Public Details For An Agent

Endpoint: <https://spacetraders.io/openapi#tag/agents/GET/agents/{agentSymbol}>

Unimplemented

Todo

- Implement `Agent::get_public_data` and `agent.get_public_data`

### Get Agent

Endpoint: <https://spacetraders.io/openapi#tag/agents/GET/my/agent>

Unimplemented

Todo

- Implement `Agent::get_agent_data` and `agent.get_data()`

### Get Agent Events

Endpoint: <https://spacetraders.io/openapi#tag/agents/GET/my/agent/events>

Unimplemented

Todo

- Implement `Agent::get_agent_events()` and `agent.get_events()`

## Contracts

### List Contracts

Endpoint: <https://spacetraders.io/openapi#tag/contracts/GET/my/contracts>

Unimplemented

Todo

- Implement: `agent.list_contracts()` and `Agent::list_contracts()`

### Get Contract

Endpoint: <https://spacetraders.io/openapi#tag/contracts/GET/my/contracts/{contractId}>

Unimplemented

Todo

- Implement: `contract.get_data()` and `Contract::get_contract_data()`

### Accept Contract

Endpoint: <https://spacetraders.io/openapi#tag/contracts/POST/my/contracts/{contractId}/accept>

Unimplemented

Todo

- Implement: `Contract::accept_contract()` and `contract.accept()`

### Fulfill contract

Endpoint: <https://spacetraders.io/openapi#tag/contracts/POST/my/contracts/{contractId}/fulfill>

Unimplemented

Todo

- Implement: `Contract::fullfil_contract()` and `contract.fulfill()`

### Deliver Cargo To Contract

Endpoint: <https://spacetraders.io/openapi#tag/contracts/POST/my/contracts/{contractId}/deliver>

Unimplemented

Todo

- Implement: `Contract::deliver_to_contract()` and `contract.deliver()`

### Negotiate Contract

Endpoint: <https://spacetraders.io/openapi#tag/contracts/POST/my/ships/{shipSymbol}/negotiate/contract>

Unimplemented

Todo

- Implement: `Contract::negotiate_contract()` and `contract.negotiate()`

## Faction

### List Factions

Endpoint: <https://spacetraders.io/openapi#tag/factions/GET/factions>

Unimplemented

Todo

- Implement:

### Faction Details

Endpoint: <https://spacetraders.io/openapi#tag/factions/GET/factions/{factionSymbol}>

Unimplemented

Todo

- Implement:

### Get My Factions

Endpoint: <https://spacetraders.io/openapi#tag/factions/GET/my/factions>

Unimplemented

Todo

- Implement:

## Fleet

### List Ships

Endpoint: <https://spacetraders.io/openapi#tag/fleet/GET/my/ships>

Unimplemented

Todo

- Implement: `Agent::list_agent_ships()` and `agent.list_ships()`

### Purchase Ships

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships>

Unimplemented

Todo

- Implement: `Agent::purchase_ship_for_agent()` and `agent.purchase_ship()`

### Get Ship

Endpoint: <https://spacetraders.io/openapi#tag/fleet/GET/my/ships/{shipSymbol}>

Unimplemented

Todo

- Implement: `Ship::get_ship_data()` and `ship.get_data()`

### Create Chart

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/chart>

Unimplemented

Todo

- Implement: `Ship:create_chart()` and `ship.chart()`

### Get Ship Cooldown

Endpoint: <https://spacetraders.io/openapi#tag/fleet/GET/my/ships/{shipSymbol}/cooldown>

Unimplemented

Todo

- Implement: `Ship::get_ship_cooldown()` and `ship.get_cooldown()`

### Dock

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/dock>

Unimplemented

Todo

- Implement: `Ship::dock_ship()` and `ship.dock()`

### Extract Resources

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/extract>

Unimplemented

Todo

- Implement: `Ship::extract_with_ship()` and `ship.extract()`

### Extract Resources with Survey

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/extract/survey>

Unimplemented

Todo

- Implement:

### Jettison Cargo

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/jettison>

Unimplemented

Todo

- Implement:

### Jump Ship

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/jump>

Unimplemented

Todo

- Implement:

### Scan Systems

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/scan/systems>

Unimplemented

Todo

- Implement:

### Scan Waypoints

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/scan/waypoints>

Unimplemented

Todo

- Implement:

### Scan Ships

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/scan/ships>

Unimplemented

Todo

- Implement:

### Scrap Ship

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/scrap>

Unimplemented

Todo

- Implement:

### Get Scrap Ship

Endpoint: <https://spacetraders.io/openapi#tag/fleet/GET/my/ships/{shipSymbol}/scrap>

Unimplemented

Todo

- Implement:

### Navigate Ship

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/navigate>

Unimplemented

Todo

- Implement:

### Warp Ship

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/warp>

Unimplemented

Todo

- Implement:

### Orbit Ship

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/orbit>

Unimplemented

Todo

- Implement:

### Purchase Cargo

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/purchase>

Unimplemented

Todo

- Implement

### Ship Refine

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/refine>

Unimplemented

Todo

- Implement:

### Refuel Ship

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/refuel>

Unimplemented

Todo

- Implement:

### Repair Ship

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/repair>

Unimplemented

Todo

- Implement:

### Get Repair Ship

Endpoint: <https://spacetraders.io/openapi#tag/fleet/GET/my/ships/{shipSymbol}/repair>

Unimplemented

Todo

- Implement:

### Sell Cargo

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/sell>

Unimplemented

Todo

- Implement:

### Siphon Resources

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/siphon>

Unimplemented

Todo

- Implement:

### Create Survey

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/survey>

Unimplemented

Todo

- Implement

### Transfer Cargo

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/transfer>

Unimplemented

Todo

- Implement

### Get Ship Cargo

Endpoint: <https://spacetraders.io/openapi#tag/fleet/GET/my/ships/{shipSymbol}/cargo>

Unimplemented

Todo

- Implement:

### Get Ship Modules

Endpoint: <https://spacetraders.io/openapi#tag/fleet/GET/my/ships/{shipSymbol}/modules>

Unimplemented

Todo

- Implement:

### Install Ship Modules

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/modules/install>

Unimplemented

Todo

- Implement:

### Remove Ship Modules

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/modules/remove>

Unimplemented

Todo

- Implement:

### Get Mounts

Endpoint: <https://spacetraders.io/openapi#tag/fleet/GET/my/ships/{shipSymbol}/mounts>

Unimplemented

Todo

- Implement:

### Install Mount

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/mounts/install>

Unimplemented

Todo

- Implement:

### Remove Mount

Endpoint: <https://spacetraders.io/openapi#tag/fleet/POST/my/ships/{shipSymbol}/mounts/remove>

Unimplemented

Todo

- Implement:

### Get Ship Nav

Endpoint: <https://spacetraders.io/openapi#tag/fleet/GET/my/ships/{shipSymbol}/nav>

Unimplemented

Todo

- Implement:

### Patch Ship Nav

Endpoint: <https://spacetraders.io/openapi#tag/fleet/PATCH/my/ships/{shipSymbol}/nav>

Unimplemented

Todo

- Implement:

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

## Data

### Describes Trade Relationship

Endpoint: <https://spacetraders.io/openapi#tag/data/GET/market/supply-chain>

Unimplemented

Todo

- Implement:

### Subscribe to Events

Endpoint: <https://spacetraders.io/openapi#tag/data/GET/my/socket.io>

Unimplemented

Todo

- Implement:

## Global

### Server Status

Endpoint: <https://spacetraders.io/openapi#tag/global/GET/>

Todo

- Implement:

### Error Code List

Endpoint: <https://spacetraders.io/openapi#tag/global/GET/error-codes>

Todo

- Implement:
