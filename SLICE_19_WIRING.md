# Slice 19 wiring

Run from repo root.

## 1) Unzip the bundle

```bash
unzip -o ~/Downloads/precomputed_context_core_slice_19_bundle.zip -d .
```

## 2) Export the new modules from `src/lib.rs`

```bash
python3 - <<'PY'
from pathlib import Path
p = Path('src/lib.rs')
text = p.read_text()
for line in ['pub mod import_policy;', 'pub mod authorization_evidence;']:
    if line not in text:
        text = text.rstrip() + '\n' + line + '\n'
p.write_text(text)
print('updated', p)
PY
```

## 3) Run the new verifier

```bash
bash scripts/verify_slice_19.sh
```
