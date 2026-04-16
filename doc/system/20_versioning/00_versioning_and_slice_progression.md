## 20. Versioning and Slice Progression

This repo advances through governed proof slices rather than freeform feature drift.

### Versioning posture

- contracts are introduced in bounded slices
- proof binaries are added only when a contract family is ready for deterministic validation
- verification scripts must prove stable repeated emission where required
- failures must remain fail-closed and must not publish success artifacts on rejected paths

### Current slice position

The repo has reached capstone proof posture through Slice 36, ending at terminal consumer import validation and a program capstone report.
