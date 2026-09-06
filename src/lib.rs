// This file was modified in a fork of jaskang/wukong-minimap.
//
// Upstream: https://github.com/jaskang/wukong-minimap (Apache-2.0)
// Fork:     https://github.com/Ouye/wukong-minimap
//
// Changes: log the build version and both repository URLs at startup, so a
// user-submitted log file identifies which build produced it.

use std::{ffi, panic, thread, time};

use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::tracing;
use hudhook::windows::Win32::{
    Foundation::{BOOL, HINSTANCE},
    System::SystemServices::DLL_PROCESS_ATTACH,
};
use utils::setup_tracing;

mod config;
mod font;
mod maploader;
mod render;
mod trail;
mod utils;
mod wukong;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(
    hmodule: HINSTANCE,
    reason: u32,
    _: *mut ffi::c_void,
) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        // 初始化日志系统，输出到文件和控制台
        setup_tracing();

        tracing::info!("DllMain: DLL_PROCESS_ATTACH");
        // 版本与出处，用于识别用户反馈的日志来自哪个构建
        tracing::info!(
            "wukong-minimap {} | fork: github.com/CNDDVP/wukong-minimap | upstream: github.com/jaskang/wukong-minimap (Apache-2.0)",
            env!("CARGO_PKG_VERSION")
        );

        // 设置panic钩子
        panic::set_hook(Box::new(|panic_info| {
            // 将panic信息记录到日志中
            if let Some(location) = panic_info.location() {
                tracing::error!(
                    "程序panic在 {}:{}:\n{}",
                    location.file(),
                    location.line(),
                    panic_info
                );
            } else {
                tracing::error!("程序panic: {}", panic_info);
            }
        }));

        thread::spawn(move || {
            tracing::info!("Background hook thread started, waiting 10s...");
            // 延迟 10 秒启动
            thread::sleep(time::Duration::from_secs(10));
            tracing::info!("Initializing MiniMap...");
            let minimap = render::MiniMap::new();
            tracing::info!("MiniMap created, applying Hudhook DX12 hooks...");
            if let Err(e) = ::hudhook::Hudhook::builder()
                .with::<ImguiDx12Hooks>(minimap)
                .with_hmodule(hmodule)
                .build()
                .apply()
            {
                tracing::error!("Couldn't apply hooks: {e:?}");
                ::hudhook::eject();
            } else {
                tracing::info!("Hudhook DX12 hooks applied successfully!");
            }
        });
    }

    BOOL::from(true)
}
