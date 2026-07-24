# Fox the Guy — Map Topology

> Not a street atlas. A **connection graph**: where you can go from where, what it costs, and which doors the Bank forgot to watch.
>
> Vertical rule: **up is the heist, down is home.** Most runs begin at the Hedgerow edge, climb into the Outshire / arcology rings, and vanish back into the Burrow before grey light.

---

## 1. The shape of the world

Think rings stacked on struts:

1. **Hedgerow** — outer wastes; Holdouts; run start.
2. **Outshire** — slums, service-towns, no time-toll; redistribution sinks.
3. **Arcology time-zones** — concentric rings, each gated by a checkpoint that charges time (hours → weeks as you move inward).
4. **Core institutions** — Bank ziggurat, Tower of Coins, Library, Powderworks, Vault (opaque).
5. **The Burrow** — hung *under* the arcology: Containers, Cells, scrubbed chrome bolted to support struts.

Foxes mostly skip checkpoints. Foxes use **side channels**.

```mermaid
flowchart TB
  subgraph Edge["Edge — free of tolls"]
    H[Hedgerow<br/>Holdouts / run start]
    O[Outshire<br/>slums · hospice · Cobble]
  end

  subgraph Rings["Arcology — time-zones"]
    Z3[Outer Ring<br/>toll: hours]
    Z2[Mid Ring<br/>toll: days]
    Z1[Inner Ring<br/>toll: weeks]
  end

  subgraph Core["Core — Bank institutions"]
    BANK[Bank Ziggurat<br/>mirrored clocks]
    TOWER[Tower of Coins]
    LIB[Saint Whither's Library]
    POW[Powderworks]
    VAULT[The Vault<br/>unseen]
  end

  subgraph Under["Under — the Burrow"]
    BURROW[Burrow scrap cathedral<br/>Containers · Cells · ozone]
  end

  H --- O
  O --- Z3
  Z3 --- Z2
  Z2 --- Z1
  Z1 --- BANK
  BANK --- TOWER
  BANK --- LIB
  BANK --- POW
  BANK -.->|Called In / never returns| VAULT
  O -.->|hangs below| BURROW
  Z3 -.->|struts / sewers| BURROW
  Z2 -.->|service guts| BURROW
  Z1 -.->|dangerous drop| BURROW
```

---

## 2. Radial rings (what checkpoints think is the map)

Official travel is ring-to-ring through **time-toll checkpoints**. Citizens pay. Foxes do not — or they spoof one handshake and pray the serial stays dead.

```mermaid
flowchart LR
  H[Hedgerow] -->|waste roads<br/>no toll| O[Outshire]
  O -->|Outer Checkpoint<br/>hours| ZR[Outer Ring]
  ZR -->|Mid Checkpoint<br/>days| MR[Mid Ring]
  MR -->|Inner Checkpoint<br/>weeks| IR[Inner Ring]
  IR -->|Core Gate<br/>or worse| CORE[Bank Core]

  ZR --- LR[Ledger Row<br/>rich walkers]
  MR --- TW[Tower fringe<br/>auditors · lifts]
  IR --- MZ[Mirrored ziggurat plaza]
  O --- CB[Cobble Street]
  O --- BR[Brick / mask-marks]
  O --- HP[Hospice fringe]
```

| Layer | Tone | Typical run verb |
|-------|------|------------------|
| Hedgerow | woods through highways, Holdout camps | brief, kit up, leave |
| Outshire | failing service grid, walk-ups, canal | give time, vanish |
| Outer Ring | licensed light, enforcer patrol density rising | take time, jam optics |
| Mid Ring | Tower-adjacent, ledgers, lifts | infiltrate, burn, steal payroll |
| Inner / Core | mirrored glass, Vault gravity | rarely; Big Plot weather |

---

## 3. Side channels (the real fox map)

These edges are why the Bank has never foreclosed on the concept of Guy. Solid lines are walkable; dashed lines are one-way, timed, or stupid.

```mermaid
flowchart TB
  subgraph Surface["Surface / legal"]
    CP[Checkpoints]
    LIFT[Tower lifts]
    ROAD[Arcology corridors]
  end

  subgraph Roof["Above"]
    ROOF[Rooftop relay<br/>chimneys · weathervanes]
    GUTTER[Gutter scramble<br/>scavenger routes]
  end

  subgraph Soft["Soft underbelly"]
    SEWER[Sewer arteries]
    SERVICE[Service ducts / crawlspaces]
    CANAL[Canal under Cobble]
    HOLD[Foreclosure holding<br/>service lift]
    SUB[Tower sub-basement<br/>side door: 'foreclosure']
  end

  subgraph Home["Burrow"]
    STRUT[Support struts]
    CONT[Container stacks]
    CELL[Cells / Cousins]
    SAFE[Hedgerow safehouse<br/>Bramble]
  end

  ROAD -.->|foxes skip| CP
  ROAD --> GUTTER
  GUTTER --> ROOF
  ROOF -->|pirate drop / crow cough| CELL

  ROAD --> SEWER
  SEWER -->|descent grate| STRUT
  SEWER --> CANAL
  CANAL -->|wet climb| CONT

  ROAD --> SERVICE
  SERVICE --> STRUT
  LIFT --> HOLD
  HOLD -->|thirty-second jam window| SERVICE
  HOLD -->|walk him out| ROAD

  SUB -->|burn ledger / run| SERVICE
  SUB -.->|if handshake lives| VAULTX[Vault ping — bad]

  STRUT --> CONT
  CONT --> CELL
  CELL -->|grey light walk| SAFE
  SAFE -->|run start| HEDGE[Hedgerow]
```

### Named side channels

| Channel | Connects | Notes |
|---------|----------|-------|
| **Sewer grate descents** | Outer/Mid Ring → strut lattice → Burrow | Primary vanish. Smell of ozone over shit means you're close. |
| **Service ducts** | Tower fringe, holding blocks, Cobble basements → Burrow | Tight; good for jammer packs and one fox. |
| **Canal under Cobble** | Cobble Street printers ↔ Outshire hospice fringe ↔ Container stacks | Kitling: if the clock ticks, put it in the canal and run. |
| **Rooftop relay** | Chimneys, weathervanes, dead-channel transmitters | Crow coughs = ours; weathervane crow = not. |
| **Gutter scramble** | Outer Ring façades → Outshire walk-ups | Arcology gutter-scavenger trade territory. |
| **Holding service lift** | Foreclosure holding ↔ Mid Ring service floor | Marrow: boredom + static. Jam optics ~30s. |
| **Tower side door** | Tower of Coins sub-basement ↔ service guts | Password *foreclosure*. Rook may be lying. |
| **Powder cart route** | Powderworks gate → Outshire basements → Burrow (if untagged) | Bramble: midnight plus seven; jam if it beeps. |
| **Strut walk** | Container to Container under the vaulted dark | Home. Never licensed. |

---

## 4. The Burrow (Heaven under the machine-city)

Built of straight-world junk hauled down piece by piece. Not a basement — a **hanging city** on the underside of the arcology.

```mermaid
flowchart LR
  subgraph Lattice["Strut lattice"]
    S1[North strut]
    S2[Cobble strut]
    S3[Tower-shadow strut]
    S4[Hedgerow drop]
  end

  subgraph Stacks["Container stacks"]
    C1[Kit Cell<br/>Hector quartermaster]
    C2[Cobble Cell<br/>Inkwells]
    C3[Memory Cell<br/>Widow / songs]
    C4[Signal loft<br/>Kitling relay]
  end

  S1 --- C1
  S2 --- C2
  S3 --- C3
  S1 --- C4
  C1 --- C2
  C2 --- C3
  C4 --- C1
  S4 -->|waste ladder| SAFE2[Hedgerow safehouse]
  C1 --> SAFE2
```

| Node | Function |
|------|----------|
| **Kit Cell** | Harness, jammer packs, riot lances; leakage intake |
| **Cobble Cell** | Pamphlets, scrubbed platen, sickroom |
| **Memory Cell** | Old Songs, mask-marks, ceremonial briefs |
| **Signal loft** | Pirate relays, crow boards, clock drops |
| **Hedgerow safehouse** | Run start / Bramble routes; powder & scrubbed kit |

---

## 5. Districts & landmarks (hooks for missions)

```mermaid
flowchart TB
  subgraph OutshireDistricts["Outshire"]
    Cobble[Cobble Street<br/>printers · apothecary · canal]
    Brick[Brick<br/>mask-marks · walk-ups]
    Hospice[Hospice fringe<br/>Pith's tally board]
    Basements[Basement powder caches]
  end

  subgraph OuterDistricts["Outer Ring — hours"]
    OClock[Clockwind Exchange<br/>unlicensed minutes · crow drops]
    Ledger[Ledger Row<br/>mirrored-glass walkers · targets]
    OLight[Licensed Light Market<br/>splice stalls · trust-linked shops]
    Patrol[Patrol Yards<br/>enforcer staging · mecha pens]
    WorksGate[Powder Gate<br/>carts at the lying clock]
    Gutter[Gutter Ward<br/>façade scrambles · sewer descents]
  end

  subgraph MidDistricts["Mid Ring — days"]
    Relay[Relay Roofs<br/>weathervanes · crow boards]
    TowerFringe[Tower Fringe<br/>lifts · bodyguards]
    Auditors[Auditors' Walk<br/>payroll couriers · ledgers]
    Holding[Foreclosure Holding<br/>blocks · service lift]
    Service[Service Guts<br/>ducts · sewer arteries]
    Liftworks[Liftworks<br/>freight lifts · controls]
  end

  subgraph InnerDistricts["Inner Ring — weeks"]
    Burn[Record Burn Annex<br/>blank records · ash vents]
    Coin[Coin Court<br/>Tower of Coins]
    Mirror[Mirrored Plaza<br/>executive arrivals]
    VaultNode[Vault Approaches<br/>opaque · maps fail]
    Works[Powderworks Yard<br/>magazines · monopoly]
    Library[Saint Whither's<br/>Library quarter]
    CoreGate[Core Gate<br/>CIBoT axis · endgame]
  end

  %% Outshire roads
  Cobble --- Brick
  Brick --- Hospice
  Cobble --- Basements

  %% Outer Circuit and radial roads
  OClock ---|Outer Circuit| Ledger
  Ledger ---|Outer Circuit| OLight
  OLight ---|Outer Circuit| Patrol
  Patrol ---|Outer Circuit| WorksGate
  WorksGate ---|Outer Circuit| Gutter
  Gutter ---|Outer Circuit| OClock
  Cobble -->|Outer Checkpoint / hours| Gutter

  %% Mid Circuit
  Relay ---|Mid Circuit| TowerFringe
  TowerFringe ---|Mid Circuit| Auditors
  Auditors ---|Mid Circuit| Holding
  Holding ---|Mid Circuit| Service
  Service ---|Mid Circuit| Liftworks
  Liftworks ---|Mid Circuit| Relay

  %% Outer-to-Mid checkpoint roads
  Ledger -->|Mid Gate / Ledger Avenue| TowerFringe
  OLight -->|Lightline| Auditors
  Patrol -->|Enforcement Way| Holding
  WorksGate -->|Powder Cart Road| Service
  Gutter -.->|sewer / gutter side channel| Liftworks
  OClock -.->|rooftop relay| Relay

  %% Inner Circuit
  Burn ---|Inner Circuit| Coin
  Coin ---|Inner Circuit| Mirror
  Mirror ---|Inner Circuit| VaultNode
  VaultNode ---|Inner Circuit| Works
  Works ---|Inner Circuit| Library
  Library ---|Inner Circuit| Burn

  %% Mid-to-Inner checkpoint roads
  Relay -->|Ash Avenue| Burn
  TowerFringe -->|Coin Processional / Inner Gate| Coin
  Auditors -->|Mirror Way| Mirror
  Holding -.->|Silent Road / restricted| VaultNode
  Service -->|Powder Road| Works
  Liftworks -->|Saint Whither Way| Library

  %% Core approaches
  Coin --> CoreGate
  Mirror --> CoreGate
  Works --> CoreGate
  Library --> CoreGate
  VaultNode -.->|one way / maybe| CoreGate
```

Ring-plan SVGs: [Outer](map_sizes/outer-ring-districts.svg) · [Mid](map_sizes/mid-ring-districts.svg) · [Inner](map_sizes/inner-ring-districts.svg)

---

## 6. A typical night (path grammar)

Most Redistributions rhyme:

```mermaid
sequenceDiagram
  participant H as Hedgerow safehouse
  participant B as Burrow Cell
  participant O as Outshire / Outer Ring
  participant T as Target (Ledger / Tower)
  participant S as Side channel
  participant Back as Burrow / Hedgerow

  H->>B: kit up (jammer, mask, touch)
  B->>O: climb strut / sewer up
  O->>T: skip checkpoint / spoof / roof
  T->>O: take time (palm glow)
  O->>O: give time (hospice / Brick)
  O->>S: vanish (sewer / duct / canal)
  S->>Back: descend before grey light
```

**Extraction** swaps the middle for Holding → jam optics → service lift → duct.  
**Powder odd job** swaps the middle for Works gate → cart → jam-if-tagged → Burrow.  
**Pamphlet courier** stays Cobble-heavy: Cell → Cobble platen → canal if burned.

---

## 7. Design notes

- **Checkpoints are for citizens.** Treat them as obstacles and comedy, not as the fox path.
- **Every surface district should have at least one down-edge** into the Burrow (sewer, duct, canal, strut hatch). If a district has none, invent one before shipping a mission there.
- **The Vault is a sink, not a level.** Edges into it are one-way story weather, not a loot room.
- **Cousin territories** sit on nodes, not on whole rings: Bramble (Hedgerow + powder routes), Pith (hospice fringe), Hector (Kit Cell), Inkwells (Cobble Cell), Kitling (signal loft / roofs), Marrow (holding), Tilde (Tower fringe), Rook (Tower side door — maybe), Widow (Memory Cell).

---

*Living document. Add nodes when missions need them; keep edges honest.*
