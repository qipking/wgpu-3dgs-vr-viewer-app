<<<<<<< HEAD
// 如果不是调试断言，则在 Windows 上隐藏控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 导入必要的库和模块
=======
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

>>>>>>> 4fe8ff633a4952b25cff21a424f9472303fa7488
use std::sync::Arc;

use eframe::{egui_wgpu, wgpu};

<<<<<<< HEAD
// 针对非 WebAssembly 目标的主函数实现
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    // 初始化日志记录器，设置默认的日志级别为 info
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // 定义原生应用选项
    let native_options = eframe::NativeOptions {
        // 设置深度缓冲区位数
        depth_buffer: 32,
        // 设置视口属性：初始大小、最小尺寸和图标
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])           // 初始窗口大小为 400x300 像素
            .with_min_inner_size([300.0, 220.0])       // 最小窗口大小为 300x220 像素
            .with_icon(
                // 加载应用程序图标（从 assets/icon-256.png）
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),    // 如果加载失败则 panic
            ),
        // 设置 WebGPU 选项
        wgpu_options: wgpu_configuration(),
        ..Default::default()                          // 其余选项使用默认值
    };

    // 使用 eframe 运行原生应用程序
    eframe::run_native(
        "3D Gaussian Splatting Viewer",               // 应用程序名称
        native_options,                               // 原生选项配置
        Box::new(|cc| Ok(Box::new(wgpu_3dgs_viewer_app::App::new(cc)))), // 创建 App 实例的闭包
    )
}

// 针对 WebAssembly 目标的主函数实现
#[cfg(target_arch = "wasm32")]
fn main() {
    // 初始化 Web 端的日志记录器
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    // 设置 Web 端选项
    let web_options = eframe::WebOptions {
        depth_buffer: 32,                             // 深度缓冲区位数
        wgpu_options: wgpu_configuration(),           // WebGPU 配置
        ..Default::default()                         // 其余选项使用默认值
    };

    // 在 Web 环境中启动异步任务
    wasm_bindgen_futures::spawn_local(async {
        // 启动 eframe Web 运行器
        let start_result = eframe::WebRunner::new()
            .start(
                wgpu_3dgs_viewer_app::App::get_canvas(),      // 获取画布元素
                web_options,                                  // Web 选项配置
                Box::new(|cc| Ok(Box::new(wgpu_3dgs_viewer_app::App::new(cc)))), // 创建 App 实例的闭包
            )
            .await;

        // 移除加载文本和旋转动画
=======
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let native_options = eframe::NativeOptions {
        depth_buffer: 32,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        wgpu_options: wgpu_configuration(),
        ..Default::default()
    };

    eframe::run_native(
        "3D Gaussian Splatting Viewer",
        native_options,
        Box::new(|cc| Ok(Box::new(wgpu_3dgs_viewer_app::App::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions {
        depth_buffer: 32,
        wgpu_options: wgpu_configuration(),
        ..Default::default()
    };

    wasm_bindgen_futures::spawn_local(async {
        let start_result = eframe::WebRunner::new()
            .start(
                wgpu_3dgs_viewer_app::App::get_canvas(),
                web_options,
                Box::new(|cc| Ok(Box::new(wgpu_3dgs_viewer_app::App::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
>>>>>>> 4fe8ff633a4952b25cff21a424f9472303fa7488
        if let Some(loading_text) =
            wgpu_3dgs_viewer_app::App::get_document().get_element_by_id("loading_text")
        {
            match start_result {
                Ok(_) => {
<<<<<<< HEAD
                    // 如果启动成功，移除加载文本
                    loading_text.remove();
                }
                Err(e) => {
                    // 如果启动失败，更新加载文本以显示错误信息
=======
                    loading_text.remove();
                }
                Err(e) => {
>>>>>>> 4fe8ff633a4952b25cff21a424f9472303fa7488
                    loading_text.set_inner_html(
                        "\
                        <p> \
                            It is possible that your browser does not support WebGPU, \
                            check \
                            <a href=\
                                \"https://github.com/gpuweb/gpuweb/wiki/Implementation-Status\"\
                            >\
                                WebGPU Implementation Status\
                            </a>\
                        </p>\
                        <p>\
                            You may try to use the native app, download from \
                            <a href=\"https://github.com/LioQing/wgpu-3dgs-viewer-app/releases\">\
                                Releases Page\
                            </a>\
                        </p>\
                        ",
                    );
<<<<<<< HEAD
                    // 抛出启动失败的恐慌
=======
>>>>>>> 4fe8ff633a4952b25cff21a424f9472303fa7488
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

<<<<<<< HEAD
// 定义 WebGPU 配置函数
fn wgpu_configuration() -> egui_wgpu::WgpuConfiguration {
    egui_wgpu::WgpuConfiguration {
        // 配置 WGPU 设置：创建新实例
        wgpu_setup: egui_wgpu::WgpuSetup::CreateNew(egui_wgpu::WgpuSetupCreateNew {
            // 优先选择高性能 GPU
            power_preference: wgpu::PowerPreference::HighPerformance,
            // 设备描述符，用于创建 wgpu::Device
            device_descriptor: Arc::new(|adapter| wgpu::DeviceDescriptor {
                label: Some("Device"),                  // 设备标签
                required_limits: adapter.limits(),      // 使用适配器的限制
                ..Default::default()                   // 其他属性使用默认值
            }),
            ..Default::default()                       // 其余选项使用默认值
        }),
        ..Default::default()                          // 其余配置使用默认值
    }
}
=======
fn wgpu_configuration() -> egui_wgpu::WgpuConfiguration {
    egui_wgpu::WgpuConfiguration {
        wgpu_setup: egui_wgpu::WgpuSetup::CreateNew(egui_wgpu::WgpuSetupCreateNew {
            power_preference: wgpu::PowerPreference::HighPerformance,
            device_descriptor: Arc::new(|adapter| wgpu::DeviceDescriptor {
                label: Some("Device"),
                required_limits: adapter.limits(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}
>>>>>>> 4fe8ff633a4952b25cff21a424f9472303fa7488
