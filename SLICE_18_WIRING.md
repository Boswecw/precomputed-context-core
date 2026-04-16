# Slice 18 wiring

Run this from repo root after unzipping the bundle.

## 1) Export the new modules from `src/lib.rs`

```bash
python3 - <<'PY'
from pathlib import Path
p = Path('src/lib.rs')
text = p.read_text()
for line in ['pub mod trust_envelope;', 'pub mod import_authorization;']:
    if line not in text:
        text = text.rstrip() + '\n' + line + '\n'
p.write_text(text)
print('updated', p)
PY
```

## 2) Run the new slice verifier

```bash
bash scripts/verify_slice_18.sh
```

## 3) Optional manual smoke commands

```bash
cargo run --bin proof_trust_envelope
cargo run --bin proof_import_authorize
```
