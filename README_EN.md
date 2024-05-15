This project is based on the Tauri framework and was originally used for packaging another [ChatGPT-Assistant](https://github.com/PierXuY/ChatGPT-Assistant) project.     
Simple configuration of the project can package any URL, with cross-platform compatibility.

**There are two ways to use it:**

### Quick Experience
- Download and install the program in Releases
- After installation, define the conf.json file in the config file and change it to your own URL information
- Open the application

### Repackaging
You can redefine the default URL, replace the application icon, etc.
- Clone the project source code
- Install the Rust environment
- Install tauri cli: `cargo install tauri-cli`
- Install the IDE plugin rust-analyzer, which will automatically install the necessary packages
- Replace the app-icon.png image in the root directory and /src path, the name must be the same
- Run `cargo tauri icon` to replace the icon
- Change the unique identifier "identifier" and "package" information in the `\src-tauri\tauri.conf.json`
- Define the conf.json file in the config file and change it to your own URL information
- Run `cargo tauri build` to package the application (`cargo tauri dev` can be used for debugging)
