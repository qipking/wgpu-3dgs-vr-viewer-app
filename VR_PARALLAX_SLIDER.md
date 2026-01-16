# VR视差滑块 - 独立的立体效果控制

## 🎯 最终解决方案

添加一个独立的**Parallax滑块**来控制VR立体效果强度，完全独立于模型的Position参数。

## 💡 为什么需要独立滑块？

### 之前尝试的问题
1. **使用相机Z**：干扰鼠标滚轮控制
2. **使用模型Z**：Position Z本身就是用来调整模型前后位置的，会导致模型放大缩小

### 正确的方案
添加一个**专门的VR视差强度参数**：
- 不影响相机控制
- 不影响模型位置
- 只控制左右眼的视差大小

## 🔧 技术实现

### 1. 添加视差强度参数

```rust
pub struct Scene {
    // ... 其他字段
    vr_mode: bool,
    vr_parallax_strength: f32,  // 🔑 新增：视差强度倍数
}
```

### 2. 添加UI滑块

```rust
// 在VR模式开关旁边添加滑块
if self.vr_mode {
    ui.separator();
    ui.label("👁️ Parallax:");
    ui.add(egui::Slider::new(&mut self.vr_parallax_strength, 0.0..=5.0)
        .text("×")
        .fixed_decimals(1));
}
```

### 3. 使用视差强度计算偏移

```rust
// 基础IPD
const BASE_IPD: f32 = 0.065;  // 65mm

// 有效IPD = 基础IPD × 视差强度
let effective_ipd = BASE_IPD * self.vr_parallax_strength;

// 左窗口：模型向右偏移
modified_pos.x += effective_ipd / 2.0;

// 右窗口：模型向左偏移
modified_pos.x -= effective_ipd / 2.0;
```

## 📊 视差强度对照表

| Parallax值 | 有效IPD | 立体效果 | 适用场景 |
|-----------|---------|---------|---------|
| 0.0× | 0mm | 无视差 | 关闭立体效果 |
| 0.5× | 32mm | 很弱 | 大场景，远景 |
| 1.0× | 65mm | 标准 | 正常观看 ✅ |
| 1.5× | 97mm | 增强 | 中等物体 |
| 2.0× | 130mm | 强 | 小物体，细节 |
| 3.0× | 195mm | 很强 | 微观物体 |
| 5.0× | 325mm | 极强 | 特殊效果 |

## 🎮 使用方法

### 基本操作
1. **开启VR模式**：勾选"VR Mode"复选框
2. **调整视差**：拖动"Parallax"滑块
   - 向左：减小视差（更舒适）
   - 向右：增大视差（更强烈）
3. **实时观察**：左右窗口的差异立即变化

### 推荐设置

#### 舒适观看（推荐新手）
```
Parallax: 0.8× - 1.2×
效果：轻微的立体感，长时间观看不累
```

#### 标准观看
```
Parallax: 1.0×
效果：标准的人眼视差，最自然
```

#### 增强观看
```
Parallax: 1.5× - 2.0×
效果：明显的立体感，适合观察细节
```

#### 极限观看（短时间）
```
Parallax: 3.0× - 5.0×
效果：极强的立体感，可能导致眼睛疲劳
```

## 🔍 与其他参数的关系

### 完全独立的控制

| 参数 | 控制方式 | 作用 | VR视差影响 |
|------|---------|------|-----------|
| **相机Z** | 鼠标滚轮 | 观察距离 | 无 ✅ |
| **模型X** | Position X滑块 | 模型左右位置 | 自动添加视差偏移 |
| **模型Y** | Position Y滑块 | 模型上下位置 | 无 ✅ |
| **模型Z** | Position Z滑块 | 模型前后位置 | 无 ✅ |
| **Parallax** | Parallax滑块 | 视差强度 | 直接控制 ✅ |

### 协同使用
```
1. 鼠标滚轮 → 调整观察距离（相机Z）
2. Position X/Y/Z → 调整模型位置和姿态
3. Parallax滑块 → 调整立体效果强度
4. 三者完全独立，互不干扰 ✅
```

## 🎓 技术优势

### 1. 完全独立
- ✅ 不影响相机控制
- ✅ 不影响模型变换
- ✅ 专门用于VR视差

### 2. 直观易用
- ✅ 滑块范围：0.0× - 5.0×
- ✅ 默认值：1.0×（标准IPD）
- ✅ 实时反馈

### 3. 灵活可控
- ✅ 可以完全关闭视差（0.0×）
- ✅ 可以微调视差（0.1×精度）
- ✅ 可以极限增强（5.0×）

## 📝 UI布局

```
┌─────────────────────────────────────────┐
│ ☑ VR Mode │ 👁️ Parallax: [====|====] 1.0× │
└─────────────────────────────────────────┘
```

- **VR Mode复选框**：开启/关闭VR模式
- **Parallax滑块**：仅在VR模式下显示
- **实时显示**：当前倍数（如"1.0×"）

## 🔧 实现细节

### 数据流

```
用户拖动滑块
    ↓
self.vr_parallax_strength 更新
    ↓
计算 effective_ipd = BASE_IPD × vr_parallax_strength
    ↓
左窗口：model.x += effective_ipd / 2
右窗口：model.x -= effective_ipd / 2
    ↓
左右窗口显示不同的模型位置
    ↓
产生立体视差效果
```

### 性能考虑
- ✅ 只是简单的乘法运算
- ✅ 无额外GPU开销
- ✅ 实时响应，无延迟

## 🧪 测试验证

### 测试步骤
1. 开启VR模式
2. 将Parallax设为0.0×
   - 预期：左右窗口完全相同
3. 将Parallax设为1.0×
   - 预期：标准视差，左右窗口有轻微差异
4. 将Parallax设为3.0×
   - 预期：强烈视差，左右窗口差异明显
5. 调整Position Z
   - 预期：模型前后移动，视差保持不变

### 调试日志
```
👁️ [VR DEBUG] LEFT window - Parallax strength: 2.0×, Model X offset: +0.065
👁️ [VR DEBUG] RIGHT window - Parallax strength: 2.0×, Model X offset: -0.065
```

## ✅ 最终效果

### 用户体验
- ✅ **简单直观**：一个滑块控制所有
- ✅ **实时反馈**：拖动立即生效
- ✅ **完全独立**：不干扰其他控制

### 技术实现
- ✅ **代码简洁**：只需一个参数
- ✅ **性能优秀**：无额外开销
- ✅ **易于维护**：逻辑清晰

### 物理准确性
- ✅ **符合原理**：模拟真实的眼间距
- ✅ **可调范围**：0× - 5×覆盖所有需求
- ✅ **默认合理**：1.0×是标准人眼视差

## 🚀 未来扩展

### 1. 预设按钮
```rust
if ui.button("Comfortable").clicked() {
    self.vr_parallax_strength = 0.8;
}
if ui.button("Standard").clicked() {
    self.vr_parallax_strength = 1.0;
}
if ui.button("Enhanced").clicked() {
    self.vr_parallax_strength = 2.0;
}
```

### 2. 自动调节
```rust
// 根据模型大小自动建议视差强度
fn suggest_parallax(model_size: f32) -> f32 {
    if model_size < 0.1 {
        3.0  // 小物体，强视差
    } else if model_size < 1.0 {
        1.5  // 中等物体
    } else {
        0.8  // 大物体，弱视差
    }
}
```

### 3. 舒适度提示
```rust
if self.vr_parallax_strength > 3.0 {
    ui.colored_label(Color32::YELLOW, "⚠️ 高视差可能导致眼睛疲劳");
}
```

## 📝 修改文件

- `src/tab/scene.rs`
  - `Scene`结构体：添加`vr_parallax_strength`字段
  - `Scene::create`：初始化为1.0
  - UI代码：添加Parallax滑块
  - 左窗口preprocess：使用`vr_parallax_strength`
  - 右窗口preprocess：使用`vr_parallax_strength`

## 🎉 总结

现在VR模式拥有完美的控制方案：
- ✅ **相机Z**：鼠标滚轮，观察距离
- ✅ **模型X/Y/Z**：Position滑块，模型位置
- ✅ **Parallax**：专用滑块，视差强度

三者完全独立，互不干扰，用户可以自由调整到最佳观看效果！
