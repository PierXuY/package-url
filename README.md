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
- 安装rust环境，并使用包管理工具cargo安装所需包（安装IDE插件rust-analyzer会自动安装）
- 更换根目录和/src路径下的app-icon.png图片，名称需一致
- 运行cargo tauri icon来更换图标
- 更改\src-tauri\tauri.conf.json中的唯一标识符"identifier"和"package"信息
- 定义config文件中的conf.json文件，改为自己的url信息
- 运行cargo tauri build打包应用（cargo tauri dev可用于调试）
