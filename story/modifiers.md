# Fox Modifiers

Stats are rolled on a **1–10** scale. Generation starts from the fox baseline in `fox-generation.json`, then applies sparse modifiers from **trade**, **reason**, **piece**, and **personal_touch**, then a little noise.

Positive modifiers mean the fox is better at that thing tonight. Negative ones are tradeoffs — former work, bad luck, or kit that cuts both ways.

| Modifier | What it does |
|----------|----------------|
| **strength** | Raw force. Forcing doors, wrestling kit, holding a lance steady, dragging a cousin out of a holding block. |
| **agility** | Whole-body quickness. Climbing struts, slipping ledges, twisting through Container gaps, not falling when the walkway does. |
| **endurance** | How long they last. Long crawls under the arcology, powder-mill shifts, cough nights, still standing when the jammer dies. |
| **movement** | Getting there. Crossing Cells, courier runs, debt-runner pace, being the closest fox when a band blinks red. |
| **dexterity** | Hands and fine work. Rewiring masks, cutting fuses, scrubbing jammer packs, picking pockets, mixing ink without spilling. |
| **perception** | Noticing. Optics bleed, Vault pings, crow signals, mirrored glass, a band reader screaming before the enforcers arrive. |
| **cunning** | Street craft. Spoofing trust-links, lying to ledgers, sabotage, knowing which checkpoint handshake will ghost. |
| **knowledge** | Learned systems. Bank procedure, pirate channels, medical kit, maps of the Burrow, what a scrubbed faceplate used to be. |
| **presence** | How they land on other people. Briefing a Cell, holding a soup line, looking like Guy when the mask is up — or looking like inventory when it isn't. |
| **nerve** | Holding the line when it gets bad. Powder work, first night under the mask, walking into Tower glass, dying on schedule. |

## How modifiers stack

1. Start at **baseline**.
2. Apply the fox's **trade** modifiers.
3. Apply **reason**, **piece**, and **personal_touch** modifiers (each may be empty).
4. Apply per-stat noise in `[-1, +1]`.
5. Clamp every stat to `1…10`.

Missing fields in a modifiers object count as `0`.
