# Documentation

This directory holds documentation that does not belong in rustdoc: the shape
of the system, the reasoning behind it, and the constraints a reader needs
before touching the code. API reference lives in doc comments next to the code,
where it cannot drift.

## Layout

```text
docs/
├── README.md      # this index
├── spec/          # architecture and per-module specifications
└── adr/           # architecture decision records, numbered and immutable
```

- **`spec/`** — one file per module or subsystem, describing its purpose,
  public surface, invariants, and operational constraints. Start with
  `spec/README.md` as the top-level architecture reference once the crate grows
  beyond a couple of modules.
- **`adr/`** — a dated record per significant decision. Use
  [`adr/0001-record-architecture-decisions.md`](adr/0001-record-architecture-decisions.md)
  as the template. An accepted ADR is not edited; it is superseded by a later
  one.

Complex modules also carry a module-level `README.md` inside `src/<module>/`
covering their design, public surface, and important constraints.

## Conventions

- Keep every Markdown file at 500 lines or fewer. When a topic outgrows that,
  split it into focused files and link them from the nearest `README.md`.
- Update documentation in the same commit as the behavior it describes.
- Prefer a concrete example over an abstract description.
- Link between documents rather than duplicating content; one fact lives in one
  place.
