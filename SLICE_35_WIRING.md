# Slice 35 wiring

Run from repo root.

## 1) Unzip the bundle

```bash
unzip -o ~/Downloads/precomputed_context_core_slice_35_bundle.zip -d .
```

## 2) Export the new module from `src/lib.rs`

```bash
python3 - <<'PY'
from pathlib import Path
p = Path('src/lib.rs')
text = p.read_text()
line = 'pub mod sealed_release_bundle;'
if line not in text:
    text = text.rstrip() + '\n' + line + '\n'
p.write_text(text)
print('updated', p)
PY
```

## 3) Run the verifier

```bash
bash scripts/verify_slice_35.sh
```
