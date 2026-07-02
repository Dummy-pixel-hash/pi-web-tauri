# PI WEB Desktop

A native Windows desktop wrapper for [PI WEB](https://github.com/jmfederico/pi-web) using [Tauri v2](https://tauri.app/).

## 🚀 Quick Start

### Prerequisites

1. **Node.js 22+** - [Download](https://nodejs.org/)
2. **Rust** - [Install via rustup](https://rustup.rs/)
3. **PI WEB** - Clone from [jmfederico/pi-web](https://github.com/jmfederico/pi-web)

### Windows-Specific Setup

1. **WebView2 Runtime** - Pre-installed on Windows 10/11. For older systems: [Download](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)
2. **Visual Studio Build Tools** (Required for native dependencies):
   - Download [VS Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
   - Select **"Desktop development with C++"** workload
   - Include Windows 10/11 SDK

## 📦 Installation

```bash
# Clone PI WEB (if not already done)
git clone https://github.com/jmfederico/pi-web.git
cd pi-web

# Build PI WEB (required before Tauri build)
npm install
npm run build

# Clone this Tauri wrapper
git clone <this-repo> pi-web-tauri
cd pi-web-tauri

# Install Tauri dependencies
npm install
```

## 🛠️ Development

```bash
# Run in development mode (hot-reload enabled)
npm run tauri:dev

# Or build and run
npm run tauri:build
```

This will:
1. Build the PI WEB frontend
2. Start the PI WEB dev server
3. Launch the Tauri window with the app

## 🏗️ Building

### Option 1: GitHub Actions (No Windows machine needed!)

Push your code to GitHub and the workflow will automatically build the app:

1. Push to GitHub: `git push origin main`
2. Go to **Actions** tab in your GitHub repo
3. Click the workflow run to view progress
4. Download the installer from **Artifacts** after build completes

**Manual trigger**:
- Go to Actions → "Build PI WEB Desktop" → "Run workflow"

### Option 2: Local Build (Windows)

```bash
# Build for Windows (x64)
npm run tauri:build:windows

# Build for all platforms
npm run tauri:build
```

### Output locations

- **NSIS Installer**: `src-tauri/target/release/bundle/nsis/PI WEB_0.1.0_x64-setup.exe`
- **MSI Installer**: `src-tauri/target/release/bundle/msi/PI WEB_0.1.0_x64.msi`
- **Portable**: `src-tauri/target/release/bundle/app/`

## 🏛️ Architecture

```
pi-web-tauri/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs        # Entry point
│   │   └── lib.rs         # Native commands & server management
│   ├── tauri.conf.json    # Tauri configuration
│   ├── capabilities/      # Permission capabilities
│   └── Cargo.toml         # Rust dependencies
├── src/
│   └── pi-web-api.js      # JavaScript API wrapper
├── package.json           # NPM scripts
└── README.md
```

## 🔧 Native Commands

The Rust backend exposes these Tauri commands:

| Command | Description |
|---------|-------------|
| `start_server` | Start the PI WEB server on a random port |
| `stop_server` | Stop the PI WEB server |
| `get_server_port` | Get the current server port |
| `get_data_dir` | Get PI WEB data directory |
| `create_directory` | Create a directory |
| `list_directory` | List files in a directory |
| `read_file` | Read file contents |
| `write_file` | Write content to a file |
| `execute_command` | Execute a Windows command |
| `get_cwd` | Get current working directory |
| `set_cwd` | Set current working directory |
| `get_env_var` | Get environment variable |
| `set_env_var` | Set environment variable |
| `get_os_info` | Get OS information |
| `get_memory_info` | Get system memory info |
| `get_disk_usage` | Get disk usage for a path |
| `open_url` | Open URL in default browser |

## 📡 Using the API

```javascript
import { PiWebApi } from './pi-web-api.js';

// Start the PI WEB server
await PiWebApi.startServer();

// Get the server port
const port = await PiWebApi.getServerPort();
console.log(`Server running on port ${port}`);

// List files
const files = await PiWebApi.listDirectory('C:\\Users\\YourName\\Projects');
console.log(files);

// Execute a command
const output = await PiWebApi.executeCommand('dir C:\\', 'C:\\');
console.log(output);

// Get system info
const osInfo = await PiWebApi.getOsInfo();
console.log(osInfo);
```

## ⚙️ Configuration

Edit `src-tauri/tauri.conf.json` to customize:

```json
{
  "productName": "PI WEB",
  "identifier": "com.pi-web.desktop",
  "build": {
    "frontendDist": "../../pi-web-New-UI-/dist",
    "beforeBuildCommand": "cd ../pi-web-New-UI-/ && npm run build"
  },
  "app": {
    "windows": [
      {
        "title": "PI WEB",
        "width": 1200,
        "height": 800,
        "minWidth": 800,
        "minHeight": 600,
        "resizable": true,
        "fullscreen": false
      }
    ]
  },
  "bundle": {
    "targets": ["nsis", "msi"],
    "icon": [...]
  }
}
```

## 🔒 Security

- Uses Tauri's permission system (capabilities)
- CSP is set to `null` for development (enable for production)
- Server runs on `127.0.0.1` only (localhost)
- Random port selection to avoid conflicts

## 🤖 CI/CD

The project includes GitHub Actions workflows:

- **`build.yml`** - Builds for Windows (uses `windows-latest` runner)
- **`build-linux.yml`** - Builds for Linux (uses `ubuntu-latest` runner)

### Features
- Automatic builds on push to `main`
- Artifact upload for manual download
- Automatic release publishing (when a GitHub release is created)

## 🐛 Troubleshooting

### Build fails with "node not found"
- Ensure Node.js is in your PATH
- Run `node --version` to verify installation

### Build fails with C++ errors
- Install Visual Studio Build Tools with "Desktop development with C++"
- Restart terminal after installation

### WebView2 not found
- Update Windows to 10/11
- Or install WebView2 Runtime: [Download](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

### Port already in use
- The app uses random ports (8500-9500) to avoid conflicts
- Kill any existing PI WEB processes: `taskkill /F /IM node.exe`

### Build takes too long
- First build downloads and compiles dependencies (can take 10-30 minutes)
- Subsequent builds are faster

## 📝 License

MIT - See [PI WEB License](../LICENSE)

## 🤝 Contributing

Contributions are welcome! Open an issue or PR on the [PI WEB repository](https://github.com/jmfederico/pi-web).

## 📚 Resources

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [PI WEB Documentation](https://pi-web.dev/)
- [Rust Documentation](https://doc.rust-lang.org/book/)
