#!/usr/bin/env bash
set -euo pipefail

PROOF_FILE="${1:-}"
if [ -z "$PROOF_FILE" ]; then
  echo "Usage: $0 <proof_file.json>" >&2
  exit 1
fi

node -e "const p = require('./' + process.argv[1]); console.log(JSON.stringify({
  predicate_kind: p.predicate_kind,
  threshold: p.threshold,
  public_inputs_hash: p.public_inputs_hash,
  proof_hash: p.proof_hash,
  ok: Array.isArray(p.bytes) && p.bytes.length >= 96,
}, null, 2));" "$PROOF_FILE"
# rev-01124
