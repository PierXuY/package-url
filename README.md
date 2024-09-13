<p align="center">
  <a href="./README.md">简体中文</a> |
  <a href="./README_EN.md">English</a>
</p>



项目基于Tauri框架，最初用于打包另一个[ChatGPT-Assistant](https://github.com/PierXuY/ChatGPT-Assistant)项目。    
用于打包URL为桌面应用


**有以下几种使用方式：**


### 简单体验
- 下载[Releases](https://github.com/PierXuY/package-url/releases/tag/app-v0.0.3)中的程序并安装
- 安装完成后，定义config文件中的config.json文件，改为自己的url信息
- 打开应用即可

### 本地打包
可以重新设定默认url，替换应用图标等
- 复制项目源码
- 安装rust环境
- 安装tauri cli：`cargo install tauri-cli`
- 安装IDE插件rust-analyzer，会自动安装所需包
- 更换 根目录 和  /src路径 下的app-icon.png图片（必须为方形），名称需一致
- 运行`cargo tauri icon`来更换图标
- 更改\src-tauri\tauri.config.json中的 唯一标识符"identifier" 和 "package"信息
- 定义config文件夹中的config.json文件，改为自己的url信息
- 运行`cargo tauri build`打包应用（`cargo tauri dev`可用于调试）

### GitHub Action 跨平台打包
- fork项目
- 更换 根目录 和  /src路径 下的app-icon.png图片（必须为方形），名称需一致
- 更改\src-tauri\tauri.config.json中的 唯一标识符"identifier" 和 "package"信息
- 定义config文件夹中的config.json文件，改为自己的url信息
- 点击fork项目上方的Actions，选择名为publish的workflow，点击右方的Run workflow
- 等待打包完成，在项目的Releases中查看（在草稿箱中，未正式发布）

### 打包本地程序（仅Windows）
- url字段设置为本地端口，例如"url": "http://127.0.0.1:8666/"
- windows_cmd字段设置为本地程序绝对路径，用于启动项目
- emit_message字段用于标识前端项目是否发送"ready"消息，用于控制主窗口的显示，可设置为false   
  发送消息示例：
  ```js
  if ("__TAURI__" in window) {
	  window.__TAURI__.event.emit("ready");
  }
  ```
注：目前的tauri版本（tauri-cli 1.6.1）默认不支持本地IP的访问，需要手动修改tauri源码，参考[issue](https://github.com/tauri-apps/tauri/issues/7009#issuecomment-1715396761)