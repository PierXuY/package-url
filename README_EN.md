This project is initially used to package another project, [ChatGPT-Assistant](https://github.com/PierXuY/ChatGPT-Assistant), for desktop application packaging of a URL.

**Usage Options:**

### Quick Start
- Download the program from the [Releases](https://github.com/PierXuY/package-url/releases/tag/app-v0.0.3) page and install it.
- After installation, define the URL information in the `config.json` file located in the `config` folder.
- Open the application.

### Local Packaging
You can reconfigure the default URL, replace the application icon, etc.
- Clone the project source code.
- Install Rust environment.
- Install Tauri CLI: `cargo install tauri-cli`.
- Install the IDE plugin `rust-analyzer`, which will automatically install the required packages.
- Replace the `app-icon.png` image in both the root directory and the `/src` path (must be square), ensuring the names are consistent.
- Run `cargo tauri icon` to change the icon.
- Modify the `"identifier"` and `"package"` fields in the `\src-tauri\tauri.config.json` file.
- Define your URL information in the `config.json` file in the `config` folder.
- Run `cargo tauri build` to package the application (use `cargo tauri dev` for debugging).

### GitHub Action Cross-Platform Packaging
- Fork the project.
- Replace the `app-icon.png` image in both the root directory and the `/src` path (must be square), ensuring the names are consistent.
- Modify the `"identifier"` and `"package"` fields in the `\src-tauri\tauri.config.json` file.
- Define your URL information in the `config.json` file in the `config` folder.
- Click on "Actions" at the top of your forked project, select the workflow named `publish`, and click "Run workflow" on the right.
- Wait for the packaging to complete and check the project's Releases section (in the draft, not yet officially published).

### Packaging Local Programs (Windows Only)

- Set the `url` field to a local port, for example: `"url": "http://127.0.0.1:8666/"`.
- Set the `windows_cmd` field to the absolute path of the local program used to start the project.
- The `emit_message` field is used to indicate whether the front-end project sends a "ready" message to control the display of the main window; it can be set to `false`.    
  Example message sending:
  ```js
  if ("__TAURI__" in window) {
      window.__TAURI__.event.emit("ready");
  }
  ```
  
**Note:** As of the current Tauri version (tauri-cli 1.6.1), local IP access is not supported by default. You will need to manually modify the Tauri source code. Refer to this [issue](https://github.com/tauri-apps/tauri/issues/7009#issuecomment-1715396761) for more information.