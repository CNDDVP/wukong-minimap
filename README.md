# 黑神话·悟空 - 内置实时地图（适配 1.0.21+ 分支）

![alt text](./docs/banner.png)

> **这是 [jaskang/wukong-minimap](https://github.com/jaskang/wukong-minimap) 的适配分支。**
>
> 原作者 [@jaskang](https://github.com/jaskang) 完成了这个插件的全部核心工作——注入、渲染、
> 坐标换算，以及那几百个手工采集的点位。**请先去给原仓库点个 star。**
>
> 本分支维护：**持续适配 1.0.21+ 最新游戏版本**，
> 顺带包含了地图朝向模式、走过路线的记录、周边敌人及掉落物雷达等功能。
> 小地图的玩法与视觉设计一律保持原样。
>
> 每个被修改过的文件顶部都写明了改了什么，以及上游是谁。

- 下载地址：[releases](https://github.com/CNDDVP/wukong-minimap/releases)
- 原项目：[jaskang/wukong-minimap](https://github.com/jaskang/wukong-minimap) · [BiliBili 演示视频](https://www.bilibili.com/video/BV1Y1KueREho/) · [Nexusmods](https://www.nexusmods.com/blackmythwukong/mods/1172)

Switch language: [English](README.en.md)

## 本分支做了什么

**一、全面适配 1.0.21+（版本号 1.0.21.23831 / Steam Build 21393610）**
- 重新计算与校验了游戏最新二进制文件的关键虚表偏移：`GObjects` (0x1D47AF90)、`FName::AppendString` (0x0CB63140) 与 `ProcessEvent` (0x0CCF8400)。
- 修复了 64 位 MSVC 环境下 `DllMain` 标准系统调用规范与返回值导致的模块被系统早期卸载及后续线程访问越界崩溃（`0xc0000005`）隐患。
- 采用**运行时特征码动态扫描**方案，兼容后续更新版本。

**二、功能与体验优化**
- 新增走过路线的记录（按区域存盘，大小地图都会画出来）。
- 小地图上的周边目标显示（敌人红点、中立灰点，目标死亡后自动消失）。
- 掉落物与采集物雷达标记，并支持区分垂直高低差与警觉战斗状态。

## 更新日志

- v2.2.1（本分支）
  - 适配游戏版本 **1.0.21.23831**（Steam BuildID: 21393610）
  - 更新静态偏移 fallback：GObjects、AppendString、ProcessEvent
  - 规范 DllMain 过程调用协议与返回值，解决 DLL 提早卸载及 0xc0000005 崩溃
- v2.2（前序分支）
  - 修复跨层换图时卡顿 1~3 秒（浮屠界最明显）：底图改为后台解码
  - 红点区分高度：空心=下方、实心=同层、实心加外环=上方
  - 红点区分战斗状态：已经发现你的敌人会呼吸
  - 新增地上物品显示：掉落物、采集物等，`7`
- v2.1（前序分支）
  - 小地图新增周边目标显示：敌人红点 `8`、中立灰点 `Shift` + `8`
- v2.0（前序分支）
  - 适配 1.0.20 之后的游戏版本
  - 修复小地图底图花屏
  - 新增地图朝向模式：`Shift` + `0`
  - 新增走过路线的记录与显示：`9` / `Shift` + `9`
  - 设置（窗口大小、比例、朝向、路线开关）自动保存，重进游戏保留
  - 路线颜色可在配置文件里自定义
  - 提示文字改用系统中文字体
  - 大地图右侧新增按键说明面板
  - 地图改为按需加载，常驻内存 368MB → 16MB
  - 地图重采样至 2000×2000，安装包 96MB → 23MB
- v1.7（上游）
  - 调整 UI，添加大量点位
- v1.6（上游）
  - 修复 AMD 显卡渲染问题，添加点位

## 按键说明

按 `Tab` 打开大地图后，右侧会列出下面这些按键，不用回来翻文档。

- `+` 放大 小地图窗口
- `-` 缩小 小地图窗口
- `Shift` + `+` 放大 小地图比例
- `Shift` + `-` 缩小 小地图比例
- `0` 显示/隐藏 地图
- `Shift` + `0` 切换地图朝向模式 **（本分支新增）**
  - 默认：地图不动，箭头随人物转向
  - 切换后：箭头锁定朝上，地图随人物转向
- `9` 开关走过的路线 **（本分支新增）**——关闭后既不显示也不记录
- `Shift` + `9` 清除全部路线 **（本分支新增）**——不可撤销，需要在 3 秒内按第二次确认
- `7` 开关地上的掉落物 / 采集物 **（本分支新增）**——默认关闭
- `8` 开关周边敌人的红点 **（本分支新增）**
- `Shift` + `8` 开关中立/友方的灰点 **（本分支新增）**——默认关闭

小地图上这些记号用两条互不干扰的通道表示：

| | 含义 |
|---|---|
| **圆形** | 角色（敌人、NPC） |
| **菱形** | 地上的东西（掉落物、采集物等） |
| **空心** | 在你**下方**一层 |
| **实心** | 和你**同层** |
| **实心 + 外环** | 在你**上方**一层 |

颜色再区分具体类别：红=敌人，灰=中立，金=掉落物，绿=采集物，白=其它可交互物。
运行时发现的土地庙，如果内置点位表里没有，会用同样的传送点图标补上。

**已经发现你的敌人，红点会缩小再弹回，循环。**小地图是用余光看的，
而余光对颜色迟钝、对运动敏感 —— 所以战斗状态用动画而不是颜色来表示。

上面这些设置（窗口大小、比例、地图朝向、路线开关）会存进 dll 同目录的
`wukong_minimap_config.json`，下次进游戏自动恢复。删掉这个文件即可恢复默认。

配置文件里还有三项颜色，只能手改：

```json
"trail_color":    "#22E0FFCC",
"enemy_color":    "#C2352BCC",
"alert_color":    "#FF4438FF",
"neutral_color":  "#C8C8C8A0",
"drop_color":     "#FFC93CEE",
"collect_color":  "#5BD86BE0",
"interact_color": "#E0E0E0B4"
```

`#RRGGBB` 或 `#RRGGBBAA`（后两位是不透明度）。改完重进游戏生效；
写错格式会退回内置颜色，并在日志里说明是哪一项。

路线按地图区域分别记录，保存在 dll 同目录的 `wukong_minimap_trails.json`，
下次启动自动载入。想备份就复制这个文件；想手动清空，删掉它即可。
每 30 秒、以及切换区域时落盘，所以游戏崩溃最多丢失最近 30 秒的路线。

## 演示截图

![alt text](./docs/demo0.png)
![alt text](./docs/demo1.png)
![alt text](./docs/demo2.png)

## 安装说明

将 `wukong-minimap.zip` 直接解压至黑神话的安装文件夹下面的 `b1\Binaries\Win64` 中
（steam 的安装文件夹可以通过右键黑神话 -> 管理 -> 浏览本地文件找到）

![alt text](./docs/install0.png)

本插件包含以下文件：

- `version.dll` 内置原生转发加载器（通过代理系统 version 库原生自加载，不依赖第三方 Loader，彻底解决与 Stardock Groupy、Windows DWM 窗口管理器以及各类 Overlay 注入器的冲突）
- `wukong_minimap.dll` 插件功能核心文件
- `maps` 地图文件夹

> **注意**：如果游戏目录下曾经存在旧版的 `dwmapi.dll`，请务必将其删除，防止旧版代理拦截导致冲突。

## 卸载

删除 `version.dll` 和 `wukong_minimap.dll` 文件即可

## 从源码构建

需要 Windows + MSVC（含 C++ 工具集）+ CMake + Rust（`x86_64-pc-windows-msvc`）。

```powershell
# 构建、安装到游戏目录、打包
.\build.ps1 -Package -Install "D:\Games\Steam\steamapps\common\BlackMythWukong\b1\Binaries\Win64"
```

`check_offsets.ps1` 可以在不启动游戏的情况下检查当前 exe 的特征码是否仍然有效——
**下次游戏更新后如果插件失效，先跑这个**：它会告诉你是只需重新编译，还是需要重新
dump SDK。

## 遇到问题

插件会在 dll 同目录写 `wukong_minimap.log`。需要详细日志时，启动游戏前设
`RUST_LOG=debug`。反馈问题时请附上这个文件。

## 许可与致谢

本项目继承上游的 **Apache License 2.0**。原始版权归 [@jaskang](https://github.com/jaskang) 所有。

- [jaskang/wukong-minimap](https://github.com/jaskang/wukong-minimap) — 原项目（Apache-2.0）
- [hudhook](https://github.com/veeenu/hudhook) — 注入与渲染框架（MIT，Andrea Venuta），本仓库内含经修改的 vendored 副本
- [imgui](https://github.com/ocornut/imgui)
- [Dumper-7](https://github.com/Encryqed/Dumper-7) — 生成 UE SDK

`maps/` 下的地图素材源自上游仓库，本分支仅做了重采样。
