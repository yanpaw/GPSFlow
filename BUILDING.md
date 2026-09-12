# Building GPSFlow from source

You only need this if you want to compile it yourself. Most people should
just download an installer from the
[Releases page](../../releases).

GPSFlow is packaged with [Tauri 2](https://tauri.app), so the app is roughly
6–12 MB installed — it uses the operating system's own web view rather than
shipping a whole browser.

---

## 1. One-time setup

### Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Windows: download and run [rustup-init.exe](https://rustup.rs) instead.

### System dependencies

- **Windows** — install [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
  (select "Desktop development with C++"). WebView2 is already on Windows 10/11.
- **macOS** — `xcode-select --install`
- **Linux (Debian/Ubuntu)**
  ```bash
  sudo apt update
  sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
    libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
  ```

### Tauri CLI

```bash
cargo install tauri-cli --version "^2.0"
```

---

## 2. Vendor the assets (recommended)

The app pulls MapLibre GL JS and Open Sans from the internet unless you download
them locally first. Do this once:

```bash
bash scripts/vendor-assets.sh
```

**Windows:** run this in **Git Bash** (right-click the project folder →
"Git Bash Here"), not Command Prompt or PowerShell. Note that Git Bash uses
forward slashes and `/c/` for the C: drive, e.g.
`cd /c/Users/you/Downloads/windlog-tauri`.

The script needs only `curl`, `grep` and `sed`, all of which ship with Git
Bash. No Python required.

**The basemap is the exception** — CARTO's vector tiles, sprites and label fonts
are fetched at runtime and can't be bundled. Offline, a session still shows the
track, stats, runs and timeline; only the map background will be blank.

---

## 3. Icons

Already generated from your Teemo drawing — `.ico` for Windows, `.icns` for
macOS (all eight resolutions), and PNGs for Linux. Nothing to do.

To swap in different artwork later, replace `src-tauri/icons/app-icon.png`
with a square 1024×1024 PNG and run:

```bash
cargo tauri icon src-tauri/icons/app-icon.png
```

---

## 4. Run and build

Development, with hot reload of the frontend:

```bash
cargo tauri dev
```

Production installers:

```bash
cargo tauri build
```

Output lands in `src-tauri/target/release/bundle/`:

| Platform | Artifact |
|---|---|
| Windows | `msi/GPSFlow_1.0.0_x64_en-US.msi`, `nsis/…-setup.exe` |
| macOS | `dmg/GPSFlow_1.0.0_aarch64.dmg`, `macos/GPSFlow.app` |
| Linux | `deb/…amd64.deb`, `appimage/…AppImage`, `rpm/…rpm` |

The first build compiles the whole Rust dependency tree and takes several
minutes. Later builds are much faster.

---

## How file opening works

Each platform hands a double-clicked file to the app differently, so both paths
are covered in `src-tauri/src/main.rs`:

- **Windows / Linux** — the path arrives as the first command-line argument,
  read at startup in `setup()`.
- **macOS** — the system sends a `RunEvent::Opened` event instead, both at
  launch and while the app is already open.

Either way the Rust side reads the file, keeps it in memory, and the frontend
collects it by calling `take_opened_file` once the UI is ready. Files opened
later are pushed to the window as a `gpx-opened` event.

The Rust side only accepts paths ending in `.gpx`, since the OS can pass
anything through a file association.

File associations register at **install** time, so test them with a built
installer rather than under `cargo tauri dev`.

---

## Security posture

- `capabilities/default.json` grants `core:default` only. No filesystem, shell
  or HTTP plugin is enabled — the app reads GPX through its own narrow Rust
  command rather than exposing general file access to the web view.
- The CSP in `tauri.conf.json` allows exactly what's needed: CARTO for tiles,
  Google Fonts and unpkg only as fallbacks. Once you've vendored assets you can
  tighten it further by removing the `https://unpkg.com`, `fonts.googleapis.com`
  and `fonts.gstatic.com` entries.

---

## Project layout

```
windlog-tauri/
├── dist/
│   ├── index.html          the viewer (identical to the browser version
│   │                       plus a small Tauri bridge at the bottom)
│   └── vendor/             populated by scripts/vendor-assets.sh
├── scripts/
│   └── vendor-assets.sh
└── src-tauri/
    ├── Cargo.toml
    ├── build.rs
    ├── tauri.conf.json     window, bundle and file-association config
    ├── capabilities/
    │   └── default.json
    ├── icons/
    └── src/
        └── main.rs
```

`dist/index.html` still opens directly in a browser — the Tauri bridge is
inert when `window.__TAURI__` is absent.
