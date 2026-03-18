# OxXlObs Local Execution Doctrine

Repo-local operating lessons for Excel observation work:
1. Declare scenario ids before widening driver code.
2. Treat workbook fingerprint, Excel build metadata, and trigger recipe as part of the contract.
3. Record capture loss explicitly at the first point it is known.
4. Prefer one stable retained baseline run over many ad hoc observation dumps.
5. Declare which surfaces are observed directly, which are derived, and which are unavailable.
6. Keep the non-Rust bridge seam narrow, explicit, and replaceable.
7. Emit replay-ready evidence bundles as early as possible.
