项目基于Tauri框架，最初用于打包另一个[ChatGPT-Assistant](https://github.com/PierXuY/ChatGPT-Assistant)项目。    


对项目进行简易配置即可打包任意URL，跨平台适配。   

### 简单体验
- 下载Releases中的程序并安装（目前仅有windows版）
- 安装完成后，定义config文件中的conf.json文件，改为自己的url信息
- 打开应用即可

### 重新打包
- 复制项目源码
- 安装rust和node.js环境
- 更换根目录和/src路径下的app-icon.png图片，名称需一致
- 运行cargo tauri icon来更换图标
- 更改\src-tauri\tauri.conf.json中的唯一标识符"identifier"和"package"信息
- 定义config文件中的conf.json文件，改为自己的url信息
- 运行cargo tauri build打包应用（cargo tauri dev可用于调试）
