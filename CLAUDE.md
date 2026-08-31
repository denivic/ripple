## graphify

This project has a knowledge graph at graphify-out/ with god nodes, community structure, and cross-file relationships.

Rules:
- For codebase questions, first run `graphify query "<question>"` when graphify-out/graph.json exists. Use `graphify path "<A>" "<B>"` for relationships and `graphify explain "<concept>"` for focused concepts. These return a scoped subgraph, usually much smaller than GRAPH_REPORT.md or raw grep output.
- If graphify-out/wiki/index.md exists, use it for broad navigation instead of raw source browsing.
- Read graphify-out/GRAPH_REPORT.md only for broad architecture review or when query/path/explain do not surface enough context.
- After modifying code, run `graphify update .` to keep the graph current (AST-only, no API cost).

### Mandatory use — no exceptions

When graphify-out/graph.json exists in this project, graphify is not optional or best-effort — it is the required first step for any task that touches understanding the codebase:
- Before answering ANY question about architecture, structure, relationships, or "how does X work" — run `graphify query`/`path`/`explain` first, even if you feel confident you already know the answer from context or memory.
- Before starting ANY code change beyond a trivial one-line edit, use graphify to orient (find the relevant nodes, callers, and dependencies) rather than jumping straight to Grep/Glob/Read exploration.
- Do not substitute raw grep, ad-hoc file browsing, or a subagent search for a graphify query when a graphify query can answer it — reach for those only after graphify's output is insufficient.
- After any code modification, running `graphify update .` is required before considering the task done — not just "nice to keep current."
- If graphify-out/graph.json is missing or clearly stale for the area you're working in, say so explicitly and offer to run `graphify update .` before proceeding, rather than silently falling back to manual exploration.
