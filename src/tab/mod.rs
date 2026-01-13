// 导入标准库中的 HashMap，用于存储键值对映射
use std::collections::HashMap;

// 从 strum 库导入枚举相关的特性，用于自动生成枚举迭代等功能
use strum::{EnumCount, EnumIter, IntoEnumIterator};

// 声明并导入各个子模块
mod camera;      // 相机相关功能模块
mod mask;        // 掩码相关功能模块
mod measurement; // 测量相关功能模块
mod metadata;    // 元数据相关功能模块
mod models;      // 模型相关功能模块
pub mod scene;   // 场景相关功能模块（公开导出）
mod selection;   // 选择相关功能模块
mod transform;   // 变换相关功能模块

// 导入项目中其他模块的类型
use crate::app;
use camera::Camera;      // 导入相机类型
use mask::Mask;          // 导入掩码类型
use measurement::Measurement; // 导入测量类型
use metadata::Metadata;   // 导入元数据类型
use models::Models;       // 导入模型类型
use scene::Scene;         // 导入场景类型
use selection::Selection; // 导入选择类型
use transform::Transform; // 导入变换类型

/// 标签页的类型枚举
/// 使用 derive 宏自动生成一些常用的 trait 实现
#[derive(
    Debug,              // 支持调试输出
    Clone,              // 支持克隆
    Copy,               // 支持复制（栈上拷贝）
    PartialEq,          // 支持相等比较
    Eq,                 // 支持完全相等比较
    Hash,               // 支持哈希计算
    EnumCount,          // 由 strum 提供，自动计算枚举变体数量
    EnumIter,           // 由 strum 提供，支持枚举迭代
    serde::Deserialize, // 支持反序列化
    serde::Serialize,   // 支持序列化
)]
pub enum Type {
    Scene,       // 场景标签页
    Transform,   // 变换标签页
    Camera,      // 相机标签页
    Measurement, // 测量标签页
    Selection,   // 选择标签页
    Metadata,    // 元数据标签页
    Models,      // 模型标签页
    Mask,        // 掩码标签页
}

// 为 Type 枚举实现方法
impl Type {
    /// 获取标签页在菜单中的标题
    pub fn menu_title(&self) -> &'static str {
        match self {
            Self::Scene => "Scene",       // 场景标签页标题
            Self::Transform => "Transform", // 变换标签页标题
            Self::Camera => "Camera",     // 相机标签页标题
            Self::Measurement => "Measurement", // 测量标签页标题
            Self::Selection => "Selection", // 选择标签页标题
            Self::Metadata => "Metadata", // 元数据标签页标题
            Self::Models => "Models",     // 模型标签页标题
            Self::Mask => "Mask",         // 掩码标签页标题
        }
    }
}

/// 标签页管理器
/// 使用 serde 宏支持序列化和反序列化，用于保存和恢复界面布局
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Manager {
    /// 停靠状态，管理标签页的布局
    dock_state: egui_dock::DockState<Type>,

    /// 标签页状态，将标签页类型映射到具体的标签页实现
    #[serde(skip)]  // 序列化时跳过此字段，因为它是运行时状态
    tabs: HashMap<Type, Box<dyn Tab>>,
}

// 为 Manager 实现方法
impl Manager {
    /// 创建一个新的标签页管理器
    pub fn new() -> Self {
        // 创建初始包含场景标签页的停靠状态
        let mut dock_state = egui_dock::DockState::new(vec![Type::Scene]);

        // 将主节点分割，在右侧创建包含变换和掩码标签页的区域，占据70%的空间
        let [_, inspector] = dock_state.main_surface_mut().split_right(
            egui_dock::NodeIndex::root(), // 从根节点开始分割
            0.7,                        // 右侧区域占70%
            vec![Type::Transform, Type::Mask], // 分配的标签页
        );
        
        // 在 inspector 节点下方创建新区域，包含模型、相机和选择标签页，各占50%空间
        let [_, _] = dock_state.main_surface_mut().split_below(
            inspector,                  // 在 inspector 节点下方分割
            0.5,                      // 新区域占50%
            vec![Type::Models, Type::Camera, Type::Selection], // 分配的标签页
        );

        // 创建空的标签页映射表
        let tabs = HashMap::new();

        // 返回新的管理器实例
        Self { dock_state, tabs }
    }

    /// 显示标签页的停靠区域
    pub fn dock_area(
        &mut self,
        ui: &mut egui::Ui,           // egui 用户界面上下文
        frame: &mut eframe::Frame,   // 当前帧对象
        state: &mut app::State,      // 应用程序状态
    ) {
        // 创建停靠区域，应用当前样式，并显示在给定的 UI 上下文中
        egui_dock::DockArea::new(&mut self.dock_state)
            .style(egui_dock::Style::from_egui(ui.style().as_ref())) // 从当前 UI 样式生成停靠区域样式
            .show_inside(
                ui,                     // 在这个 UI 上下文中显示
                &mut Viewer {           // 使用自定义的 Viewer 来处理标签页内容
                    tabs: &mut self.tabs, // 传递标签页映射表的可变引用
                    frame,              // 传递帧对象的可变引用
                    state,              // 传递应用程序状态的可变引用
                },
            );
    }

    /// 标签页菜单
    pub fn menu(&mut self, ui: &mut egui::Ui) {
        // 记录要添加和删除的标签页
        let mut added = Vec::new();   // 要添加的标签页
        let mut removed = Vec::new(); // 要删除的标签页

        // 遍历所有标签页类型
        for tab in Type::iter() {
            // 查找当前标签页是否存在于停靠状态中
            let curr = self.dock_state.find_tab(&tab);
            // 检查标签页是否已启用（存在）
            let mut enabled = curr.is_some();

            // 创建一个切换按钮，显示标签页标题
            ui.toggle_value(&mut enabled, tab.menu_title());
            
            // 如果用户启用了标签页但当前不存在，则将其加入添加列表
            if enabled && curr.is_none() {
                added.push(tab);
            // 如果用户禁用了标签页且当前存在，则将其加入删除列表
            } else if !enabled && curr.is_some() {
                removed.push(curr.unwrap());
            }
        }

        // 如果有任何添加或删除操作，则关闭菜单
        if !added.is_empty() || !removed.is_empty() {
            ui.close_menu();
        }
        
        // 添加新的标签页到停靠状态
        if !added.is_empty() {
            self.dock_state.add_window(added);
        }
        
        // 从停靠状态中移除标签页
        if !removed.is_empty() {
            for i in removed {
                self.dock_state.remove_tab(i);
            }
        }

        // 绘制分隔线
        ui.separator();

        // 如果点击重置布局按钮，则重新创建管理器
        if ui.button("Reset Layout").clicked() {
            *self = Self::new(); // 替换当前实例为新的默认实例
        }
    }
}

// 为 Manager 实现 Debug trait，用于调试输出
impl std::fmt::Debug for Manager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Manager")           // 创建 Manager 的调试结构
            .field("dock_state", &self.dock_state) // 添加 dock_state 字段
            .finish()                        // 完成构建
    }
}

// 为 Manager 实现 Default trait，提供默认值
impl Default for Manager {
    fn default() -> Self {
        Self::new() // 使用 new 方法创建默认实例
    }
}

/// 标签页 trait，定义了标签页应实现的方法
pub trait Tab {
    /// 创建新标签页
    /// state 参数是应用程序状态的可变引用
    fn create(state: &mut app::State) -> Self
    where
        Self: Sized;  // 自类型约束，确保 Self 是已知大小的类型

    /// 获取标签页的标题
    /// frame 是当前帧对象，state 是应用程序状态
    fn title(&mut self, frame: &mut eframe::Frame, state: &mut app::State) -> egui::WidgetText;

    /// 标签页的用户界面实现
    /// ui 是 egui 用户界面上下文，frame 是当前帧对象，state 是应用程序状态
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame, state: &mut app::State);
}

/// 标签页查看器，实现了 egui_dock 的 TabViewer trait
struct Viewer<'a> {
    /// 标签页状态映射表的可变引用
    tabs: &'a mut HashMap<Type, Box<dyn Tab>>,

    /// 当前帧对象的可变引用
    frame: &'a mut eframe::Frame,

    /// 应用程序状态的可变引用
    state: &'a mut app::State,
}

// 为 Viewer 实现方法
impl Viewer<'_> {
    /// 确保标签页已创建
    /// 如果标签页不存在，则根据类型创建相应的标签页实例
    fn make_sure_created(&mut self, tab: Type) {
        // 使用 entry API 检查标签页是否存在，如果不存在则创建
        self.tabs.entry(tab).or_insert_with(|| match tab {
            Type::Scene => Box::new(Scene::create(self.state)) as Box<dyn Tab>,
            Type::Transform => Box::new(Transform::create(self.state)) as Box<dyn Tab>,
            Type::Camera => Box::new(Camera::create(self.state)) as Box<dyn Tab>,
            Type::Measurement => Box::new(Measurement::create(self.state)) as Box<dyn Tab>,
            Type::Selection => Box::new(Selection::create(self.state)) as Box<dyn Tab>,
            Type::Metadata => Box::new(Metadata::create(self.state)) as Box<dyn Tab>,
            Type::Models => Box::new(Models::create(self.state)) as Box<dyn Tab>,
            Type::Mask => Box::new(Mask::create(self.state)) as Box<dyn Tab>,
        });
    }
}

// 为 Viewer 实现 egui_dock 的 TabViewer trait
impl egui_dock::TabViewer for Viewer<'_> {
    // 指定标签页的类型
    type Tab = Type;

    // 获取标签页标题
    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        // 确保标签页已创建
        self.make_sure_created(*tab);
        // 获取标签页实例并调用其 title 方法
        self.tabs
            .get_mut(tab)
            .expect("tab")  // 如果标签页不存在则 panic
            .title(self.frame, self.state)
    }

    // 渲染标签页 UI
    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        // 确保标签页已创建
        self.make_sure_created(*tab);
        // 获取标签页实例并调用其 ui 方法
        self.tabs
            .get_mut(tab)
            .expect("tab")  // 如果标签页不存在则 panic
            .ui(ui, self.frame, self.state);
    }
}