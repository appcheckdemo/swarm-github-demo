# Evidence plan

Use a clean checkout, identical viewport (1280x900), device scale, fonts, and server URL. Save screenshots outside the repository and record command, revision, timestamp, viewport, and observations. Compare rendering/index.html in a reference browser and target renderer at desktop and 390x844; inspect wrapping, controls, layout, stacking, and clipping.

Required captures:

1. **Controlled comparison:** desktop and narrow rendering screenshots; report visible differences, not invented pass results.
2. **Three public-page captures:** choose stable public URLs, record URL and retrieval date, and capture both systems. Respect robots, terms, privacy, and network availability; blocked pages stay blocked.
3. **Recovery:** capture malformed, exception, missing-resource, and failure final states with console/network logs. Run infinite-loop in a subprocess with a hard timeout.
4. **Task interaction:** serve `fixtures` locally, verify JSON-loaded items, then capture add, toggle, remove, and all filters.

Keep originals and a manifest. Change capability statuses only when an artifact is reproducible and linked; record negative outcomes honestly.
