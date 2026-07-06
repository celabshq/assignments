> [!NOTE]
> 💡 Take home assignment for https://join.com/companies/celabseu/16394366-security-cryptography-engineer

# Secure Asset Storage Protocol (SASP)

## 📌 Context & Overview

In a secure communication platform, Messaging Layer Security (MLS) handles group synchronization, membership, and low-bandwidth metadata. However, large file attachments (images, documents, and other long-term assets) are not directly routed through the main MLS application channel due to throughput and serialization overhead and availability. Instead, encrypted are offloaded to an external, untrusted Storage Server.

Your task is to implement SASP (Secure Asset Storage Protocol): a custom, minimal, asynchronous network protocol where clients utilize an OpenMLS Exported Secret, from the group they send the asset to, to authenticate connections and securely upload encrypted file payloads to an untrusted storage server.

```
                  [ OpenMLS Control Plane ]
                     /                 \
       (Group Sync) /                   \ (Group Sync)
                   v                     v
              [ Client A ]          [ Client B ]
                   |                     |
     (Derive keys  |                     | (Derive keys
      via Exporter)|                     |  via Exporter)
                   v                     v
               [ Custom SASP over TCP Stream ]
                   \                     /
                    v                   v
                [ Untrusted Storage Server ]
```

## 🎯 Objective

Implement a minimal, functional Rust client and server that demonstrates:

1. **Cryptographic Key Derivation:** Utilizing the OpenMLS Exporter API to get a base secret, using `libcrux` sub-crates to derive storage keys and handle transport security.
2. **Custom Protocol Framing:** Building everything necessary to handle asset uploads.
3. **Defensive Parsing:** Handling untrusted network buffers safely without risking memory exhaustion or panics.

## 🧠 Required System Design Discussion

Please include a section in your `README.md` or prepare a set of talking points addressing the long-term lifecycle security properties of this architecture. Specifically, we want to hear your thoughts on:

1. **New Joiner Access:** When a new user joins the MLS group, (how) should they get access to past assets uploaded prior to their arrival?
2. **Removed User Revocation:** When a user is removed from the MLS group, how do we guarantee they lose access to *future* assets and, if necessary, *existing* assets stored on the untrusted server?
3. **Overall Security Commitments:** What are the Forward Secrecy (FS) and Post-Compromise Security (PCS) bounds of your out-of-band asset layer compared to the core MLS protocol?

## 📦 Provided Skeleton Code

To respect your time and keep this assignment focused strictly on protocol layout and networking, the starter repository provides:

- A pre-configured `Cargo.toml` featuring `openmls`, `openmls_libcrux_crypto`, the individual `libcrux-*` sub-crates, `tokio`, and `serde`.
- A test helper that initializes a dummy, 2-member `MlsGroup` using the `OpenMlsLibcruxCrypto` provider.
- A basic network loop stub.

## 📊 Deliverables & What We Look For

Please provide a link to a Git repository containing your implementation along with a concise `README.md` explaining your architectural choices and your responses to the design discussion points above.

We will evaluate your submission based on:

- **Correct Usage of Cryptography:** Correct utilization of cryptographic primitives to build the protocol.
- **Idiomatic Rust & Async Safety:** Clean utilization of Tokio tasks, appropriate usage of `Arc`/`Mutex` or channels.
- **Input Validation & Parser Robustness:** How defensively your byte parser handles malicious or malformed frames over the wire.
- **Error Architecture:** Use of typed errors instead of `unwrap()` calls.

### 🕒 Time Commitment

This assignment is intended to take approximately **4 hours**, and not more than 8 hours. We do not expect production-grade UI, persistent databases, or complex configurations. Focus on clean data structures, robust code, and clear protocol states.

## 🚀 Getting Started - Skeleton

```sh
cargo build          # compiles the skeleton (stubs use `todo!()`)
cargo test           # baseline passes: two MLS members derive an equal exporter secret
```

Run **two client instances** to form a group (start the founder first, then the joiner,
in separate terminals). Each is one MLS member; they exchange KeyPackage/Welcome directly
and then derive the same exporter secret:

```sh
cargo run --bin client founder   # creates the group, listens for the joiner
cargo run --bin client joiner    # connects, joins, derives the same secret
```

Both set up the group, then hit the SASP `todo!`. The
untrusted storage server (the target of your SASP protocol) is run separately:

```sh
cargo run --bin server            # binds 127.0.0.1:9000 and accepts connections
```

### Repository layout

| Path | Status | What it is |
| --- | --- | --- |
| `src/mls.rs` | ✅ provided | Single-member MLS `Client` (create/join a group, export secrets). You shouldn't need to touch it. |
| `src/wire.rs` | ✅ provided | Length-prefixed async transport for the client-to-client MLS handshake. |
| `src/bin/client.rs` | 🟡 provided + **TODO** | P2P MLS handshake + secret export provided; SASP upload/download is yours. |
| `src/bin/server.rs` | 🟡 provided + **TODO** | Untrusted storage server; accept loop provided, frame handling/storage is yours. |
| `src/protocol.rs` | 🛠️ **TODO** | SASP framing, defensive parser, and key derivation. The core of the task. |
| `src/error.rs` | 🛠️ **TODO** | Typed error architecture (seeded with a few variants). |
| `tests/group_secret.rs` | ✅ provided | Green baseline test exercising the MLS `Client` handshake. |
