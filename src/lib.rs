// 定义模块声明 - 这些是当前库的内部模块
mod app; // 声明 app 模块，该模块定义在 app.rs 文件中
mod renderer; // 声明 renderer 模块，该模块定义在 renderer/mod.rs 或 renderer.rs 文件中
mod tab; // 声明 tab 模块，该模块定义在 tab/mod.rs 或 tab.rs 文件中
mod util; // 声明 util 模块，该模块定义在 util.rs 文件中

// 将 app 模块中的 App 结构体公开导出，使其可以从库外部访问
pub use app::App;

// Rust 库（library crate）是可被其他项目依赖和使用的代码包。在这个文件中：

// mod 关键字用于声明模块，它告诉 Rust 在当前 crate 中存在这些模块。模块是一种组织和封装代码的方式，可以将相关的功能分组在一起。

// mod app; 表示在当前目录下的 app.rs 文件或 app/mod.rs 文件中定义了一个名为 app 的模块。

// pub use 是重新导出语法，它将模块内部的项（这里是 App 类型）公开到库的公共 API 中，这样使用此库的其他代码可以直接访问 App，而无需写完整的路径 wgpu_3dgs_viewer_app::app::App。

// 这个库定义了四个内部模块：

// app: 包含主应用程序逻辑
// renderer: 负责渲染相关功能
// tab: 包含 UI 标签页功能
// util: 包含通用工具函数
// 这种结构使代码更易于组织和维护，同时通过 pub use 控制哪些功能对外部可见，形成清晰的公共 API。
