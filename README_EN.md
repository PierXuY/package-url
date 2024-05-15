The project is based on the Tauri framework, originally used to package another [ChatGPT-Assistant](https://github.com/PierXuY/ChatGPT-Assistant) project. 
A simple configuration of the project can package any URL, cross-platform adaptability. 


**There are several ways to use it:**


### Simple Experience
- Download and install the program in [Releases](https://github.com/PierXuY/package-url/releases/tag/app-v0.0.3)
- After installation, define the conf.json file in the config folder and change it to your own URL information
- Open the application

### Local Packaging
You can re-set the default URL, replace the application icon, etc.
- Copy the project source code
- Install rust environment
- Install tauri cli: `cargo install tauri-cli`
- Install the IDE plugin rust-analyzer, which will automatically install the required packages
- Replace the app-icon.png images(must be square) in the root directory and /src path, the name should be consistent
- Run `cargo tauri icon` to replace the icon
- Change the unique identifier "identifier" and "package" information in \src-tauri\tauri.conf.json
- Define the conf.json file in the config folder and change it to your own URL information
- Run `cargo tauri build` to package the application (`cargo tauri dev` can be used for debugging)

### GitHub Action Cross-platform Packaging
- Fork the project
- Replace the app-icon.png images(must be square) in the root directory and /src path, the name should be consistent
- Change the unique identifier "identifier" and "package" information in \src-tauri\tauri.conf.json
- Define the conf.json file in the config folder and change it to your own URL information
- Click on Actions above the forked project, select the workflow named publish, and click Run workflow on the right
- Wait for the package to complete, check in the Releases of the project (in the draft box, not officially released)
