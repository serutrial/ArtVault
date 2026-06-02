# ArtVault — Local AI Gallery Desktop App

A fully offline desktop app for browsing, organizing, and tagging your AI-generated images and models. Built with Tauri (Rust + WebView) — no internet required, no storage limits.

---

## What You Need (One-Time Setup)

Install these in order:

### 1. Microsoft C++ Build Tools
Required by Rust on Windows.
- Download: https://visualstudio.microsoft.com/visual-cpp-build-tools/
- Run the installer → select **"Desktop development with C++"** → Install
- Restart your PC after

### 2. Rust
- Download: https://rustup.rs/
- Run `rustup-init.exe` → press Enter for all defaults
- After install, open a **new** terminal and verify:
  ```
  rustc --version
  ```

### 3. Node.js (v18 or newer)
- Download: https://nodejs.org/ (choose LTS)
- Install with defaults
- Verify:
  ```
  node --version
  npm --version
  ```

### 4. WebView2 (usually already on Windows 11)
- If on Windows 10, download: https://developer.microsoft.com/en-us/microsoft-edge/webview2/
- Most Windows 10/11 PCs already have this via Edge updates

---

## Building the App

Open a terminal (Command Prompt or PowerShell) in the `ArtVault` folder:

```bash
# Step 1 — Install JS dependencies (only needed once)
npm install

# Step 2 — Build the .exe installer
npm run build
```

The build takes **5–15 minutes** the first time (Rust compiles everything).
Subsequent builds are much faster.

### Output
When done, find your installer at:
```
ArtVault\src-tauri\target\release\bundle\msi\ArtVault_1.0.0_x64_en-US.msi
```
Or the portable exe at:
```
ArtVault\src-tauri\target\release\artvault.exe
```

Double-click the `.msi` to install, or just run the `.exe` directly.

---

## Running in Dev Mode (optional, for testing)

```bash
npm run dev
```

This opens the app in a live-reload window. Changes to `src/index.html` appear instantly.

---

## Where Your Data Is Stored

| What | Where |
|------|-------|
| Images & covers | `C:\Users\<you>\AppData\Roaming\com.artvault.app\images\` |
| Database (metadata JSON) | `C:\Users\<you>\AppData\Roaming\com.artvault.app\db.json` |

Your data persists across app updates. To back up everything, just copy that folder.

---

## Generating App Icons (Optional)

The build needs icon files in `src-tauri/icons/`. To generate them from a PNG:

```bash
# Install the Tauri CLI globally first
npm install -g @tauri-apps/cli

# Generate icons from any 1024x1024 PNG image
npx tauri icon path/to/your-icon.png
```

This auto-generates all required sizes. If you skip this, Tauri uses default placeholder icons.

---

## Project Structure

```
ArtVault/
├── src/
│   └── index.html          ← Full frontend (HTML + CSS + JS)
├── src-tauri/
│   ├── src/
│   │   └── main.rs         ← Rust backend (file I/O, DB read/write)
│   ├── Cargo.toml          ← Rust dependencies
│   ├── tauri.conf.json     ← App config (window size, name, icons)
│   └── build.rs            ← Tauri build script
├── package.json            ← Node scripts
└── README.md               ← This file
```

---

## Troubleshooting

**"error: linker `link.exe` not found"**
→ Install Microsoft C++ Build Tools (step 1 above) and restart your terminal.

**"Could not find WebView2"**
→ Download and install WebView2 from the Microsoft link above.

**"npm: command not found"**
→ Node.js isn't installed or the terminal needs to be reopened after install.

**Build fails with Rust errors**
→ Run `rustup update` to make sure Rust is current, then try again.

**App opens but images don't show**
→ This is normal on first launch for sample placeholder images (they have no file). Upload real images via the `+ Add Image` button.

---

## Features

- ✅ Fully offline — no internet needed after fonts cached
- ✅ No storage limits — images stored as real files on disk
- ✅ Auto-reads EXIF metadata from JPEG (A1111, SD.Next, ComfyUI, InvokeAI)
- ✅ Batch image upload
- ✅ Model library with type badges, ratings, tags
- ✅ Masonry image grid with lightbox
- ✅ Search, filter, sort
- ✅ Copy prompt, like, delete
- ✅ Data persists across launches (JSON on disk, not localStorage)
