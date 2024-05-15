<p align="center">
  <a href="./README.md">简体中文</a> |
  <a href="./README_EN.md">English</a>
</p>



项目基于Tauri框架，最初用于打包另一个[ChatGPT-Assistant](https://github.com/PierXuY/ChatGPT-Assistant)项目。    
对项目进行简易配置即可打包任意URL，跨平台适配。   


**有以下几种使用方式：**


### 简单体验
- 下载[Releases](https://github.com/PierXuY/package-url/releases/tag/app-v0.0.3)中的程序并安装
- 安装完成后，定义config文件中的conf.json文件，改为自己的url信息
- 打开应用即可

### 重新打包（本地）
可以重新设定默认url，替换应用图标等
- 复制项目源码
- 安装rust环境
- 安装tauri cli：`cargo install tauri-cli`
- 安装IDE插件rust-analyzer，会自动安装所需包
- 更换`根目录`和`/src路径`下的app-icon.png图片，名称需一致
- 运行`cargo tauri icon`来更换图标
- 更改`\src-tauri\tauri.conf.json`中的 唯一标识符"identifier" 和 "package"信息
- 定义config文件夹中的`conf.json文件`，改为自己的url信息
- 运行`cargo tauri build`打包应用（`cargo tauri dev`可用于调试）

### 重新打包（GitHub Action）
- fork项目
- 更换`根目录`和`/src路径`下的app-icon.png图片，名称需一致
- 运行`cargo tauri icon`来更换图标
- 更改`\src-tauri\tauri.conf.json`中的 唯一标识符"identifier" 和 "package"信息
- 定义config文件夹中的`conf.json文件`，改为自己的url信息
- 点击fork项目上方的Actions，选择名为publish的workflow，点击右方的Run workflow
- 等待打包完成，在项目的Releases中查看（可能在草稿箱）
