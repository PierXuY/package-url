<p align="center">
  <a href="./README.md">简体中文</a> |
  <a href="./README_en.md">English</a>
</p>



项目基于Tauri框架，最初用于打包另一个[ChatGPT-Assistant](https://github.com/PierXuY/ChatGPT-Assistant)项目。    
对项目进行简易配置即可打包任意URL，跨平台适配。   


**有以下两种使用方式：**


### 简单体验
- 下载Releases中的程序并安装
- 安装完成后，定义config文件中的conf.json文件，改为自己的url信息
- 打开应用即可

### 重新打包
可以重新设定默认url，替换应用图标等
- 复制项目源码
- 安装rust环境
- 安装tauri cli：`cargo install tauri-cli`
- 安装IDE插件rust-analyzer，会自动安装所需包
- 更换根目录和/src路径下的app-icon.png图片，名称需一致
- 运行`cargo tauri icon`来更换图标
- 更改\src-tauri\tauri.conf.json中的唯一标识符"identifier"和"package"信息
- 定义config文件中的conf.json文件，改为自己的url信息
- 运行`cargo tauri build`打包应用（`cargo tauri dev`可用于调试）
