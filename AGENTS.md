# AGENTS.md

Bevy plugin for loading and rendering SVG files (2D + 3D). Targets Bevy 0.18.

## Commands

```bash
cargo check                    # type-check
cargo clippy                   # lint (very strict — see below)
cargo build                    # compile
cargo test --no-run            # build tests (there are currently no #[test] functions)
```

## Architecture

```
src/
  lib.rs          → SvgPlugin, prelude
  loader.rs       → SvgAssetLoader (.svg/.svgz) via Bevy AssetLoader
  svg.rs          → Svg asset (the core struct), parsing (usvg), tessellation, PathDescriptor
  origin.rs       → Origin enum (TopLeft/Center/…), OriginState, apply_origin system
  plugin.rs       → SvgRenderPlugin<C> — watches AssetEvent<Svg>, links mesh to entities
  resources.rs    → FillTessellator / StrokeTessellator resources (wraps lyon)
  render/
    mod.rs        → SvgComponent trait (polymorphic 2d/3d), on_insert hook
    plugin.rs     → top-level SvgPlugin (inserts tess resources, adds 2d/3d sub-plugins)
    tessellation.rs → generate_buffer() — converts path descriptors to vertex buffers
    vertex_buffer.rs → Vertex, VertexBuffers, Convert→Mesh, lyon vertex constructors
    svg2d/mod.rs  → Svg2d component (wraps Handle<Svg>), requires Mesh2d/Origin/Transform
    svg2d/plugin.rs → Material2dPlugin<Svg>, loads svg_2d.wgsl
    svg2d/svg_2d.wgsl
    svg3d/mod.rs  → Svg3d component, requires Mesh3d/Origin/Transform
    svg3d/plugin.rs → MaterialPlugin<Svg>, loads svg_3d.wgsl
    svg3d/svg_3d.wgsl
```

## Key patterns

- **Feature gates**: `2d`/`3d` Cargo features control whether `svg2d/` and `svg3d/` modules are compiled. Both are default-on. Conditional compilation uses `#[cfg(feature = "2d")]` / `#[cfg(feature = "3d")]`.
- **Polymorphic rendering**: `SvgComponent` trait abstracts over 2D/3D, with associated types `MeshComponent` (Mesh2d vs Mesh3d) and `MaterialComponent`. `SvgRenderPlugin<C>` is generic over this trait.
- **Asset loading flow**: `SvgAssetLoader::load` — reads bytes → `Svg::from_bytes` (usvg parse) → `svg.tessellate()` (lyon) → stores mesh as labeled sub-asset → returns `Svg` asset. The mesh handle lives on `Svg.mesh`.
- **Mesh linking**: `svg_mesh_linker` system (in `Last` schedule) watches `AssetEvent<Svg>` and copies `svg.mesh` into the entity's `Mesh2d`/`Mesh3d` when the loaded SVG changes.
- **Origin system**: `apply_origin` runs in `Last` schedule. Modifies `GlobalTransform` directly (not the Transform hierarchy) so origin changes don't cascade to children. Uses `OriginState` to track previous origin and reverse old offset.
- **Y-axis flip**: Bevy uses a top-left Y-down coordinate system for 2D. Tessellation flips Y via `Transform::from_scale(Vec3::new(1.0, -1.0, 1.0))`.
- **`Convert<T>` trait**: Locally-defined `Into` surrogate to work around Rust orphan rules (see `src/lib.rs`).
- **`Svg` IS the material**: struct implements both `Material2d` and `Material` directly — it serves as both asset data and shader material.

## Gotchas

- **Extremely strict clippy**: `lib.rs` enables `clippy::all`, `clippy::restriction`, `clippy::pedantic`, `clippy::nursery`, `clippy::cargo`. Expect many warnings. Do not relax lints without good reason.
- **No tests exist**. Any added logic should include tests.
- **`from_bytes` loads system fonts** via `fontdb.load_system_fonts()`. This may fail in sandboxed/headless environments without fontconfig.
- **Cargo.lock is in .gitignore** (library convention). Don't commit it.
- **Examples are excluded from the published crate** (Cargo.toml `exclude` field). The repo has no `examples/` directory.
- **Fixed UUID shader handles**: shaders use `uuid_handle!()` — do not change these unless the shader also changes.
- **Vertex color space difference**: Fill vertices use `to_linear()`, stroke vertices use `to_srgba()`. This looks like a latent bug — stroke colors may appear wrong.
- **MSRV is 1.89** (required by Bevy 0.18).
- **`EventReader` → `MessageReader`**: Bevy 0.17 renamed this type. In 0.18, `EventReader` is fully removed. Import from `bevy::prelude::MessageReader`.
- **`LoadContext::path()` returns `AssetPath`**, not `&Path`. Use `.path().path()` to get the actual `&Path`.
- **`AssetLoader` now requires `TypePath`**: derive it on `SvgAssetLoader`.
