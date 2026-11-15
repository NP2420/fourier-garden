# Fourier Garden

Audio-reactive visuals for Windows. Fourier Garden listens to whatever your PC is playing,
analyses it in Rust, and draws it in a small, movable, always-on-top window. A desktop
wallpaper mode is planned.

**Status:** early development. Phase 0 (scaffold) is done; audio capture is next.

## Development

Requirements: Git, Node.js LTS, pnpm, Rust (stable, MSVC) and the Visual Studio C++ Build Tools.
WebView2 ships with Windows 11.

```powershell
pnpm install
pnpm tauri dev      # run the app
pnpm lint           # ESLint
pnpm typecheck      # TypeScript
pnpm test           # Vitest
cargo test --workspace
```

## Privacy

Audio is analysed in memory and discarded. Nothing is recorded, logged or sent anywhere.

## Licence

Licensed under either of MIT or Apache-2.0, at your option. See `LICENSE-MIT` and
`LICENSE-APACHE`. Third-party material is listed in `THIRD_PARTY.md`.
