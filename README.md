# ecoblock-mesh

Provides mesh maintenance and connectivity primitives used by the Ecoblock gossip layer to keep peers connected and data replicated.

Purpose
-------
- Maintain an overlay mesh of connected peers with simple heuristics for fanout, churn and health checks.
- Provide connection management (dialing, backoff, reconnection) and lightweight peer scoring to prefer reliable neighbors.
- Expose helpers to integrate with `ecoblock-gossip` and validate peer-supplied messages using `ecoblock-core` and `ecoblock-crypto`.

What lives here
---------------
- `src/mesh` — mesh maintenance, peer selection and periodic maintenance tasks.
- `src/peer` — peer identity, addressing, and lightweight scoring/ranking utilities.
- `src/transport` — connection dialing, keepalive, and basic transport abstractions (TCP/UDP or loopback for tests).
- `src/sim` — (optional) small helpers for local simulation of mesh behavior used by integration tests.

Stability contract
------------------
The mesh crate defines in-proc wire formats for peer metadata and any canonical encodings used for peer IDs or signed handshake messages. These encodings must remain stable between releases or be accompanied by a documented migration strategy and compatibility tests. Changes that affect peer identification, handshake payloads or scoring semantics can break mesh stability and should include regression tests and a migration plan.

Quick example
-------------
Create a mesh node, connect to a peer and publish a block (API names are illustrative):

```rust
// Conceptual example — adapt to actual crate APIs
// let mut mesh = Mesh::new(config);
// mesh.start()?;
// mesh.connect(peer_addr)?;
// mesh.publish(block)?; // relay to selected peers
```

Running tests
-------------
Run the crate tests locally:

```bash
cd libs/ecoblock-mesh
cargo test
```

Note: some integration or simulation tests may require opening loopback ports or running multiple processes; unit tests should remain isolated.

Contributing
------------
- Keep peer identity and handshake encodings stable. If you change them, include a migration plan and test vectors.
- Add unit tests for any change that affects peer selection, scoring, or connection lifecycle.
- When changing mesh heuristics (fanout, TTL, backoff), add small simulation/integration tests that show behaviour across multiple nodes.

License
-------
MIT
