#!/usr/bin/env bash
# Verify the cq2-router on-chain program matches this repo.
# Requires: solana CLI 3.x, docker, cargo install solana-verify --locked.
#
# Run from the repo root.

set -euo pipefail

PROGRAM_ID="cqXSJi3MdPgPfxKfe86nxrrq6speZP4LTEJ2WUXoSQF"
PROGRAMDATA="E2wqazUFXz2nmLT5uo2h4YG6UJzue8v5EBswXaYzmwHY"

if ! command -v solana-verify >/dev/null 2>&1; then
  echo "[verify] install solana-verify first: cargo install solana-verify --locked"
  exit 1
fi

echo "[verify] reproducibly building cq2-router via solana-verify (uses docker)"
solana-verify build --library-name cq2-router

LOCAL_HASH=$(solana-verify get-executable-hash target/deploy/cq2_router.so)
echo "[verify] local executable hash:    $LOCAL_HASH"

ONCHAIN_HASH=$(solana-verify get-program-hash "$PROGRAM_ID")
echo "[verify] on chain hash:            $ONCHAIN_HASH"

if [[ "$LOCAL_HASH" == "$ONCHAIN_HASH" ]]; then
  echo "[verify] MATCH. This repo reproduces the deployed program byte for byte."
else
  echo "[verify] MISMATCH. Possible causes: wrong git tag checked out, dependency drift, toolchain skew."
  exit 2
fi
