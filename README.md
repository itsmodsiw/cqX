<p align="center">
  <img src="assets/cqx.png" alt="cqX" width="120" />
</p>

# cqX

[![Solana mainnet](https://img.shields.io/badge/Solana-mainnet-1f2937?logo=solana&logoColor=white)](https://solscan.io/account/cqXSJi3MdPgPfxKfe86nxrrq6speZP4LTEJ2WUXoSQF)
[![Program cqXSJi3M](https://img.shields.io/badge/program-cqXSJi3M%E2%80%A6-1f2937?logo=solana&logoColor=white)](https://solscan.io/account/cqXSJi3MdPgPfxKfe86nxrrq6speZP4LTEJ2WUXoSQF)
[![Anchor 0.29.0](https://img.shields.io/badge/anchor-0.29.0-1f2937)](https://www.anchor-lang.com/)
[![Rust 1.90](https://img.shields.io/badge/rust-1.90-1f2937?logo=rust&logoColor=white)](https://rust-lang.org)
[![Verifiable build](https://img.shields.io/badge/verifiable_build-yes-1f2937)](https://github.com/solana-foundation/solana-verifiable-build)
[![security.txt](https://img.shields.io/badge/security.txt-enabled-1f2937)](https://github.com/neodyme-labs/solana-security-txt)
[![License Apache-2.0](https://img.shields.io/badge/license-Apache_2.0-1f2937)](LICENSE)
[![12 adapters landed](https://img.shields.io/badge/adapters_landed-12-1f2937)](#supported-swap-variants)

On chain swap dispatcher used by [cq2](https://blog.carbium.io). One Borsh `SwapArgs` struct routes through any supported DEX under a single program id.

## Program details

| Field | Value |
|---|---|
| **Mainnet program id** | `cqXSJi3MdPgPfxKfe86nxrrq6speZP4LTEJ2WUXoSQF` |
| **ProgramData** | `E2wqazUFXz2nmLT5uo2h4YG6UJzue8v5EBswXaYzmwHY` |
| **Upgrade authority** | `cq2Lcb7UtZZb43hwGXdxrYJfGcvZfoGGriePRSFd51R` |
| **Version** | 0.2.0 |
| **Anchor** | 0.29.0 |
| **Rust toolchain** | 1.90.0 |
| **BPF path** | `target/deploy/cqx.so` |
| **BPF size** | 316168 bytes (308 kB) |
| **sha256 of deployed bytes** | `b1fc22f197f0d5dc04b41e8ac73454b5f57049306a7e1d439e814721ba9892cf` |

## Supported swap variants

The `SwapType` enum dispatches to every DEX cq2 routes through. Ordinals are stable across releases. The landed tx column links to a mainnet sig that exercises the variant.

| Ord | Variant | DEX | Landed tx |
|---:|---|---|---|
| 0 | Raydium | Raydium AMM | [`G55sfdaE…`](https://solscan.io/tx/G55sfdaEd1MYaV2onTYYuMcS8QNQTUNVP4YA3jgeTtYKejREFLjRZPSGQCVbBc3mixw542tUKjcqBdoRiXS36a2) |
| 1 | RaydiumV2 | Raydium AMM v2 | |
| 2 | RaydiumCp | Raydium CPMM | |
| 3 | RaydiumClmm | Raydium CLMM | [`3T1pwQKN…`](https://solscan.io/tx/3T1pwQKNmcGxZMZ61adqA2CGSF8D4kAztx2r4E9tyszKkgSDWc842HNw87uhvEnhxACFSSBvVmr1QNgKSjktzVK8) |
| 4 | RaydiumClmmV2 | Raydium CLMM v2 | |
| 5 | RaydiumStable | Raydium Stable | |
| 6 | Meteora | Meteora DAMM v1 | |
| 7 | MeteoraDlmm | Meteora DLMM | |
| 8 | MeteoraDlmmV2 | Meteora DLMM v2 | |
| 9 | MeteoraDammV2 | Meteora DAMM v2 | [`3FDaJidr…`](https://solscan.io/tx/3FDaJidrzcUCnjfoEiYQVTkMa7MMvY9E6CQSVXu2YpVY3yy9ViBCfSYGL7YU2wBUqqgTyjcQHKQw6XXVWE5f8B54) |
| 10 | WhirlpoolS2 | Orca Whirlpool | [`G55sfdaE…`](https://solscan.io/tx/G55sfdaEd1MYaV2onTYYuMcS8QNQTUNVP4YA3jgeTtYKejREFLjRZPSGQCVbBc3mixw542tUKjcqBdoRiXS36a2) |
| 11 | OrcaV1S | Orca v1 | |
| 12 | OrcaV2S | Orca v2 | [`G55sfdaE…`](https://solscan.io/tx/G55sfdaEd1MYaV2onTYYuMcS8QNQTUNVP4YA3jgeTtYKejREFLjRZPSGQCVbBc3mixw542tUKjcqBdoRiXS36a2) |
| 13 | PumpAmmBuyExactQuoteIn | Pump AMM (buy) | |
| 14 | PumpAmmSell | Pump AMM (sell) | |
| 15 | HumidiFi | HumidiFi | [`5zFGxyHt…`](https://solscan.io/tx/5zFGxyHtJTyWdqvufsQMFC3bMC2amAJC78FC6AZhMHrv9ogEWjEN6RLP9bz97uCmiHBX9YBZ63sBLNnr4KWF2WX7) |
| 16 | _DeadOrd16 | reserved | _n/a_ |
| 17 | GoonfiV2 | GoonFi v2 | [`5Km794yX…`](https://solscan.io/tx/5Km794yXHWUxnCGnRnvUTS5PsNdLSBGkikarAnkLXQuhCdJg7qNPf5bcZQnaQgQixHi1WJWEkxbGrJuNpBE2LqhP) |
| 18 | AlphaQ | AlphaQ | [`66uLaQZQ…`](https://solscan.io/tx/66uLaQZQniMSLACGihQGrGAJb1uZJsdv7UktMHwA9kgypNPwT4RK6QKcgkorDzLMP1dimuZZG6zAkpQ2erFx2vRG) |
| 19 | SanctumLstSwap | Sanctum LST · SPL Stake Pool · SanctumSpl · SanctumSplMulti | [`2MYU7UkV…`](https://solscan.io/tx/2MYU7UkVkoTKPztKHS31gFwiDctFHJgCXu3YJUgYdehTkxCZLXtBTk1pLQHZdJcBGNjhNReFPJRF74pXEvdT21v1) |
| 20 | ManifestSwapV2 | Manifest | [`3m3odY8b…`](https://solscan.io/tx/3m3odY8bcqUS7dCha9DCJf7XjxfrtmeqZWpnhFkmYcmap5NhEWPjNrcjjZP4dyqhzS8sMKFNE7ZdB6MnMW1fqNcA) |
| 21 | SolfiV2 | SolFi v2 | |
| 22 | TesseraV | TesseraV | |
| 23 | ZeroFi | ZeroFi | |
| 24 | PancakeswapV3 | PancakeSwap v3 | [`w7SeJszo…`](https://solscan.io/tx/w7SeJszoxWdXiJNQ9SwSJTiKcAo13KvAQKzDEueXuisKQjKLsKK6XR3jr1yR811ccEj8KAPjrR7hiptM8FTjD8C) |
| 25 | Byreal | Byreal CLMM | [`2ZtbEvwd…`](https://solscan.io/tx/2ZtbEvwdPLUiYi1p1K3ejd7ErzH7vC9Eu8YvrBQJ8Zi7vsZho7vYPFafFJwcL9jL1Ke1Np3dMXGR123UFAqLgL4n) |

## Integrator fee

The `swap` instruction accepts a trailing `fee_bps: u16` (Borsh). When non zero:

* Capped at `MAX_FEE_BPS = 500` (5%).
* Last entry in `remaining_accounts` is the integrator treasury.
* SPL output: `user_dst_ata` at slot `n-2`, `treasury_ata` at slot `n-1`, transfers `(gross_out * fee_bps) / 10_000`.
* SOL output: `treasury_wsol_ata` at slot `n-1` receives the wSOL fee before the temp wSOL is closed.

See [`src/lib.rs`](programs/cqX/src/lib.rs) `swap` for the full enforcement path.

## security.txt

Embedded in the binary per [neodyme-labs/solana-security-txt](https://github.com/neodyme-labs/solana-security-txt). Solscan and explorers surface this as a badge.

```
name              cqX
project_url       https://blog.carbium.io
contacts          email:modsiw@carbium.io
policy            https://carbium.io
preferred_languages  en
source_code       https://github.com/itsmodsiw/cqX
source_revision   0.2.0
source_release    0.2.0
```

## Verifiable build

The deployed program is reproducible from this repo at the tagged commit using [solana-verifiable-build](https://github.com/solana-foundation/solana-verifiable-build).

```bash
# Install once
cargo install solana-verify --locked

# Reproduce the deployed bytes from this repo
solana-verify build --library-name cqX

# Compare the hash with what is on chain
solana-verify get-executable-hash target/deploy/cqx.so
solana-verify get-program-hash cqXSJi3MdPgPfxKfe86nxrrq6speZP4LTEJ2WUXoSQF
```

Both hashes should match `b1fc22f197f0d5dc04b41e8ac73454b5f57049306a7e1d439e814721ba9892cf`.

## What is NOT in this repo

This repo contains only the on chain dispatcher. The off chain router, adapters, schedulers, probes, pre compute service, and quote engine that drive cq2 routing decisions are private. The on chain program is intentionally minimal: it accepts a route plan, CPIs to the requested DEXes, and enforces slippage plus optional integrator fee. Nothing more.

## Build locally

```bash
cd programs/cqX
cargo build-sbf
```

Output at `target/deploy/cqx.so`.

## License

Apache 2.0. See [LICENSE](LICENSE).

## Acknowledgements

* [neodyme-labs/solana-security-txt](https://github.com/neodyme-labs/solana-security-txt) for the embedded badge.
* [solana-foundation/solana-verifiable-build](https://github.com/solana-foundation/solana-verifiable-build) for the reproducible build verifier.
* [anchor](https://www.anchor-lang.com/) for the program framework.
