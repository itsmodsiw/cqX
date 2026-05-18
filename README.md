# cq2-router

[![Solana](https://img.shields.io/badge/Solana-mainnet-9945FF?logo=solana&logoColor=white)](https://explorer.solana.com/address/cqXSJi3MdPgPfxKfe86nxrrq6speZP4LTEJ2WUXoSQF)
[![Program](https://img.shields.io/badge/program-cqXSJi3M-blueviolet)](https://solscan.io/account/cqXSJi3MdPgPfxKfe86nxrrq6speZP4LTEJ2WUXoSQF)
[![Anchor](https://img.shields.io/badge/anchor-0.29.0-512BD4)](https://www.anchor-lang.com/)
[![Rust](https://img.shields.io/badge/rust-1.90-CE412B?logo=rust&logoColor=white)](https://rust-lang.org)
[![security.txt](https://img.shields.io/badge/security.txt-enabled-success?logo=security&logoColor=white)](https://github.com/neodyme-labs/solana-security-txt)
[![Verifiable Build](https://img.shields.io/badge/verifiable_build-solana_foundation-success)](https://github.com/solana-foundation/solana-verifiable-build)
[![License](https://img.shields.io/badge/license-Apache_2.0-blue)](LICENSE)

On chain swap dispatcher used by [cq2](https://carbium.io). One Borsh `SwapArgs` struct routes through any supported DEX under a single program id.

## Program details

| Field | Value |
|---|---|
| **Mainnet program id** | `cqXSJi3MdPgPfxKfe86nxrrq6speZP4LTEJ2WUXoSQF` |
| **ProgramData** | `E2wqazUFXz2nmLT5uo2h4YG6UJzue8v5EBswXaYzmwHY` |
| **Upgrade authority** | `cq2Lcb7UtZZb43hwGXdxrYJfGcvZfoGGriePRSFd51R` |
| **Version** | 0.3.1 |
| **Anchor** | 0.29.0 |
| **Rust toolchain** | 1.90.0 |
| **sha256 of deployed bytes** | `29e9895a03727df265194f777360db4098d87344306cacd762f3e16901176ffc` |

## Supported swap variants

The `SwapType` enum dispatches to every DEX cq2 routes through. Ordinals are stable across releases.

| Ord | Variant | DEX |
|---:|---|---|
| 0 | `Raydium` | Raydium AMM |
| 1 | `RaydiumV2` | Raydium AMM v2 |
| 2 | `RaydiumCp` | Raydium CPMM |
| 3 | `RaydiumClmm` | Raydium CLMM |
| 4 | `RaydiumClmmV2` | Raydium CLMM v2 |
| 5 | `RaydiumStable` | Raydium Stable |
| 6 | `Meteora` | Meteora DAMM v1 |
| 7 | `MeteoraDlmm` | Meteora DLMM |
| 8 | `MeteoraDlmmV2` | Meteora DLMM v2 |
| 9 | `MeteoraDammV2` | Meteora DAMM v2 |
| 10 | `WhirlpoolS2` | Orca Whirlpool |
| 11 | `OrcaV1S` | Orca v1 |
| 12 | `OrcaV2S` | Orca v2 |
| 13 | `PumpAmmBuyExactQuoteIn` | Pump AMM buy |
| 14 | `PumpAmmSell` | Pump AMM sell |
| 15 | `HumidiFi` | HumidiFi |
| 16 | `BisonFi` | BisonFi (gated off chain) |
| 17 | `GoonfiV2` | GoonFi v2 |
| 18 | `AlphaQ` | AlphaQ |

## Integrator fee

The `swap` instruction accepts a trailing `fee_bps: u16` (Borsh). When non zero:

* Capped at `MAX_FEE_BPS = 500` (5%).
* Last entry in `remaining_accounts` is the integrator treasury.
* SPL output: `user_dst_ata` at slot `n-2`, `treasury_ata` at slot `n-1`, transfers `(gross_out * fee_bps) / 10_000`.
* SOL output: `treasury_wsol_ata` at slot `n-1` receives the wSOL fee before the temp wSOL is closed.

See [`src/lib.rs`](programs/cq2-router/src/lib.rs) `swap` for the full enforcement path.

## security.txt

Embedded in the binary per [neodyme-labs/solana-security-txt](https://github.com/neodyme-labs/solana-security-txt). Solscan and explorers surface this as a badge.

```
name              cq2-router
project_url       https://carbium.io
contacts          email:security@carbium.io, link:https://carbium.io/security
policy            https://carbium.io/security-policy
preferred_languages  en
source_code       https://github.com/itsmodsiw/cq2-router
auditors          internal review only, no third party audit yet, PRs welcome
```

## Verifiable build

The deployed program is reproducible from this repo at the tagged commit using [solana-verifiable-build](https://github.com/solana-foundation/solana-verifiable-build).

```bash
# Install once
cargo install solana-verify --locked

# Reproduce the deployed bytes from this repo
solana-verify build

# Compare the hash with what is on chain
solana-verify get-executable-hash target/deploy/cq2_router.so
solana-verify get-program-hash cqXSJi3MdPgPfxKfe86nxrrq6speZP4LTEJ2WUXoSQF
```

Both hashes should match `29e9895a03727df265194f777360db4098d87344306cacd762f3e16901176ffc`.

## What is NOT in this repo

This repo contains only the on chain dispatcher. The off chain router, adapters, schedulers, probes, pre compute service, and quote engine that drive cq2 routing decisions are private. The on chain program is intentionally minimal: it accepts a route plan, CPIs to the requested DEXes, and enforces slippage plus optional integrator fee. Nothing more.

## Build locally

```bash
cd programs/cq2-router
cargo build-sbf
```

Output at `target/sbpf-solana-solana/release/cq2_router.so`.

## License

Apache 2.0. See [LICENSE](LICENSE).

## Acknowledgements

* [neodyme-labs/solana-security-txt](https://github.com/neodyme-labs/solana-security-txt) for the embedded badge.
* [solana-foundation/solana-verifiable-build](https://github.com/solana-foundation/solana-verifiable-build) for the reproducible build verifier.
* [anchor](https://www.anchor-lang.com/) for the program framework.
