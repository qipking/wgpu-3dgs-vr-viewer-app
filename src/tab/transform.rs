// 引入外部依赖和模块
use wgpu_3dgs_viewer as gs;  // 将 wgpu_3dgs_viewer 库作为 gs 别名使用

use crate::{app, util};  // 导入当前 crate 的 app 和 util 模块

use super::Tab;  // 从父模块导入 Tab trait

/// 变换编辑器标签页
/// 这个结构体负责处理模型的变换操作（位置、旋转、缩放等）
#[derive(Debug)]
pub struct Transform;

// 为 Transform 结构体实现 Tab trait
impl Tab for Transform {
    // 创建新的 Transform 实例
    fn create(_state: &mut app::State) -> Self
    where
        Self: Sized,
    {
        Self
    }

    // 返回标签页标题
    fn title(&mut self, _frame: &mut eframe::Frame, _state: &mut app::State) -> egui::WidgetText {
        "Transform".into()  // 返回字符串 "Transform"
    }

    // 定义用户界面的主要逻辑
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame, state: &mut app::State) {
        // 匹配状态中的 gs 字段，获取模型和高斯变换数据
        let (model, gaussian, ui_builder) = match &mut state.gs {
            // 如果数据已加载，则获取当前选中模型的变换信息和全局高斯变换
            app::Loadable::Loaded(gs) => (
                &mut gs
                    .models
                    .get_mut(&gs.selected_model_key)  // 获取当前选中的模型
                    .expect("selected model")         // 确保模型存在
                    .transform,                       // 获取模型的变换信息
                &mut gs.gaussian_transform,           // 获取全局高斯变换
                egui::UiBuilder::new(),              // 创建可用的 UI 构建器
            ),
            // 如果数据未加载，则创建默认变换值，并禁用 UI
            app::Loadable::Unloaded { .. } => (
                &mut app::GaussianSplattingModelTransform::new(),  // 创建默认模型变换
                &mut app::GaussianSplattingGaussianTransform::new(), // 创建默认高斯变换
                egui::UiBuilder::new().disabled(),                 // 创建禁用的 UI 构建器
            ),
        };

        // 调整 UI 元素之间的间距
        ui.spacing_mut().item_spacing = egui::vec2(ui.spacing().item_spacing.x, 12.0);

        // 使用 UI 构建器范围创建界面
        ui.scope_builder(ui_builder, |ui| {
            ui.label(egui::RichText::new("Model").strong());  // 添加加粗的 "Model" 标签
            self.model(ui, model);  // 渲染模型变换 UI

            ui.separator();  // 添加分隔线

            ui.label(egui::RichText::new("Gaussian").strong());  // 添加加粗的 "Gaussian" 标签
            self.gaussian(ui, gaussian);  // 渲染高斯变换 UI
        });
    }
}

// 为 Transform 实现具体的方法
impl Transform {
    /// 创建模型变换的 UI
    fn model(&mut self, ui: &mut egui::Ui, transform: &mut app::GaussianSplattingModelTransform) {
        // 使用网格布局展示变换参数
        egui::Grid::new("model_transform_grid").show(ui, |ui| {
            // 定义一个宏用于快速创建坐标轴值输入控件
            macro_rules! value {
                ($ui:expr, $axis:expr, $value:expr) => {
                    $ui.horizontal(|ui| {
                        // 减小水平间距
                        ui.spacing_mut().item_spacing.x /= 2.0;

                        // 显示坐标轴标签
                        ui.label($axis);
                        // 添加可拖拽的数值输入框
                        ui.add(
                            egui::DragValue::new(&mut $value)
                                .speed(0.01)      // 设置拖动速度
                                .fixed_decimals(4), // 设置固定小数位数
                        );
                    });
                };
            }

            // 位置行标签
            ui.label("Position");
            // 水平排列 X、Y、Z 坐标输入
            ui.horizontal(|ui| {
                value!(ui, "X", transform.pos.x);  // X 轴位置
                value!(ui, "Y", transform.pos.y);  // Y 轴位置
                value!(ui, "Z", transform.pos.z);  // Z 轴位置
            });
            ui.end_row();  // 结束当前网格行

            // 旋转行标签
            ui.label("Rotation");
            // 水平排列 X、Y、Z 轴旋转值
            ui.horizontal(|ui| {
                value!(ui, "X", transform.rot.x);  // X 轴旋转
                value!(ui, "Y", transform.rot.y);  // Y 轴旋转
                value!(ui, "Z", transform.rot.z);  // Z 轴旋转
            });
            ui.end_row();  // 结束当前网格行

            // 缩放行标签
            ui.label("Scale");
            // 水平排列 X、Y、Z 轴缩放值
            ui.horizontal(|ui| {
                value!(ui, "X", transform.scale.x);  // X 轴缩放
                value!(ui, "Y", transform.scale.y);  // Y 轴缩放
                value!(ui, "Z", transform.scale.z);  // Z 轴缩放
            });
            ui.end_row();  // 结束当前网格行
        });
    }

    /// 创建高斯变换的 UI
    fn gaussian(
        &mut self,
        ui: &mut egui::Ui,
        transform: &mut app::GaussianSplattingGaussianTransform,
    ) {
        // 使用网格布局展示高斯变换参数
        egui::Grid::new("gaussian_transform_grid").show(ui, |ui| {
            // 设置滑块宽度
            ui.spacing_mut().slider_width = 100.0;

            // 大小控制
            ui.label("Size");
            ui.add(egui::Slider::new(&mut transform.size, 0.0..=2.0).fixed_decimals(2));  // 添加大小滑块，范围 0.0 到 2.0
            ui.end_row();

            // 显示模式选择
            ui.label("Display Mode");
            ui.horizontal(|ui| {
                // 定义一个宏用于快速创建选择标签
                macro_rules! value {
                    ($ui: expr, $value: expr, $label: ident) => {
                        // 创建可选择标签，当点击时设置对应的显示模式
                        if $ui
                            .selectable_label(
                                $value == gs::GaussianDisplayMode::$label,  // 检查当前是否为此模式
                                stringify!($label),                        // 将标识符转换为字符串显示
                            )
                            .clicked()  // 检测点击事件
                        {
                            $value = gs::GaussianDisplayMode::$label;  // 设置新显示模式
                        }
                    };
                }

                // 提供三种显示模式：Splat、Ellipse、Point
                value!(ui, transform.display_mode, Splat);   // 点状显示
                value!(ui, transform.display_mode, Ellipse); // 椭圆显示
                value!(ui, transform.display_mode, Point);   // 点显示
            });
            ui.end_row();

            // 球谐函数度数控制
            ui.label("SH Degree")  // 球谐(Spherical Harmonics)度数标签
                .on_hover_text("Degree of spherical harmonics");  // 鼠标悬停提示文本
            let mut deg = transform.sh_deg.degree();  // 获取当前球谐度数
            ui.add(egui::Slider::new(&mut deg, 0..=3));  // 添加度数滑块，范围 0 到 3
            transform.sh_deg = gs::GaussianShDegree::new(deg).expect("SH degree");  // 设置新的度数值
            ui.end_row();

            // 不使用 SH0 控制
            ui.label("No SH0")  // "不使用0次球谐函数"标签
                .on_hover_text("Exclude the 0th degree of spherical harmonics");  // 鼠标悬停提示
            ui.add(util::toggle(&mut transform.no_sh0));  // 添加开关控件控制是否排除0次球谐
            ui.end_row();
        });
    }
}