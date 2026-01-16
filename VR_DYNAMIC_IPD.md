# VR动态IPD - 基于模型Z位置的视差控制（修正版）

## 🎯 正确的实现

将IPD偏移应用到**模型的X位置**，而不是相机位置。这样：
- **相机Z**：保持不变，用于鼠标滚轮控制观察距离
- **模型X**：左右偏移，创建立体视差
- **模型Z**：控制视差强度（Z值越大，视差越强）

## 💡 设计理念

### 为什么偏移模型而不是相机？

1. **相机应该保持一致**：
   - 相机Z用于鼠标滚轮控制观察距离
   - 不应该被VR模式干扰

2. **模型偏移更直观**：
   - 左眼看到模型在右边一点
   - 右眼看到模型在左边一点
   - 符合真实的立体视觉原理

3. **视差强度可控**：
   - 通过模型Z位置控制视差强度
   - Z值越大，模型偏移越大，立体效果越强

## 🔧 技术实现

### 核心公式

```rust
// 基础IPD（人类平均眼间距）
const BASE_IPD: f32 = 0.065;  // 65mm

// 获取模型Z位置
let model_z = gs.selected_model().transform.pos.z;

// 计算视差因子（正比关系）
let parallax_factor = 1.0 + model_z.abs() * 0.5;

// 有效IPD
let effective_ipd = BASE_IPD * parallax_factor;
```

### 视差因子计算

| Model Z | Parallax Factor | Effective IPD | 说明 |
|---------|----------------|---------------|------|
| 0.0 | 1.0 | 65mm | 标准视差 |
| 1.0 | 1.5 | 97mm | 视差增强 |
| 2.0 | 2.0 | 130mm | 视差更强 |
| 3.0 | 2.5 | 162mm | 非常强的视差 |
| -1.0 | 1.5 | 97mm | 负Z也增强视差 |

### 代码实现

#### 左窗口（主viewer）
```rust
// 在VR模式下，偏移模型位置而不是相机
let model_pos = if apply_vr_offset {
    let mut modified_pos = gs.selected_model().transform.pos;
    
    const BASE_IPD: f32 = 0.065;
    let model_z = gs.selected_model().transform.pos.z;
    let parallax_factor = 1.0 + model_z.abs() * 0.5;
    let effective_ipd = BASE_IPD * parallax_factor;
    
    // 左窗口：模型向右偏移
    modified_pos.x += effective_ipd / 2.0;
    
    modified_pos
} else {
    gs.selected_model().transform.pos
};

// 使用偏移后的模型位置
viewer.update_model_transform(queue, &key, model_pos, quat, scale);
```

#### 右窗口（VR viewer）
```rust
// 相机保持不变
vr_viewer.update_camera(queue, &gs.camera.control, viewer_size);

// 计算模型偏移
const BASE_IPD: f32 = 0.065;
let model_z = gs.selected_model().transform.pos.z;
let parallax_factor = 1.0 + model_z.abs() * 0.5;
let effective_ipd = BASE_IPD * parallax_factor;

// 右窗口：模型向左偏移
let mut modified_model_pos = gs.selected_model().transform.pos;
modified_model_pos.x -= effective_ipd / 2.0;

// 使用偏移后的模型位置
vr_viewer.update_model_transform(queue, &key, modified_model_pos, quat, scale);
```

## 📊 使用方法

### 调整立体效果

1. **开启VR模式**
2. **调整Position Z滑块**：
   - **增大Z值（正或负）**：视差增强，立体效果更强
   - **Z值接近0**：标准视差（65mm）
   - **Z值为0**：最小视差

### 最佳实践

#### 观看小物体（需要强立体感）
```
Position Z: 2.0 到 3.0
效果：强烈的立体感，适合观察细节
Effective IPD: 130mm - 162mm
```

#### 观看中等场景
```
Position Z: 0.5 到 1.5
效果：适中的立体感，舒适观看
Effective IPD: 81mm - 97mm
```

#### 观看大场景（需要弱立体感）
```
Position Z: 0.0 到 0.5
效果：轻微的立体感，避免眼睛疲劳
Effective IPD: 65mm - 81mm
```

## 🎓 物理原理

### 为什么这样设计？

在真实世界中：
- **近处物体**：左右眼看到的位置差异大
- **远处物体**：左右眼看到的位置差异小

我们的实现：
- **模型Z大**：模型偏移大，模拟"近处物体"的效果
- **模型Z小**：模型偏移小，模拟"远处物体"的效果

### 与相机偏移的区别

| 方法 | 相机偏移 | 模型偏移（当前） |
|------|---------|----------------|
| 相机Z | 被修改 | 保持不变 ✅ |
| 鼠标滚轮 | 受影响 | 正常工作 ✅ |
| 视差控制 | 复杂 | 简单直观 ✅ |
| 物理准确性 | 较低 | 较高 ✅ |

## 🔍 调试信息

启用VR模式后，日志会显示：
```
👁️ [VR DEBUG] LEFT window - Model Z: 2.000, Parallax factor: 2.000, Model X offset: +0.065
👁️ [VR DEBUG] RIGHT window - Model Z: 2.000, Parallax factor: 2.000, Model X offset: -0.065
```

这表示：
- 模型Z = 2.0
- 视差因子 = 2.0倍
- 左窗口模型向右偏移65mm
- 右窗口模型向左偏移65mm
- 总视差 = 130mm

## 📝 控制变量说明

### 相机Z（Camera Control Z）
- **控制方式**：鼠标滚轮
- **作用**：调整观察距离（整体缩放）
- **VR模式影响**：无影响，保持正常工作 ✅

### 模型X（Transform Position X）
- **控制方式**：UI滑块
- **作用**：模型左右位置
- **VR模式影响**：自动添加IPD偏移

### 模型Z（Transform Position Z）
- **控制方式**：UI滑块
- **作用**：模型前后位置
- **VR模式影响**：控制视差强度 ✅

## 🚀 使用流程

```
1. 用鼠标滚轮调整观察距离（相机Z）
   ↓
2. 开启VR模式
   ↓
3. 调整Position Z滑块控制立体效果强度
   ↓
4. 观察左右窗口的视差变化
   ↓
5. 找到最舒适的观看设置
```

## ✅ 优势总结

### 用户体验
- ✅ **相机独立**：鼠标滚轮正常工作，不受VR影响
- ✅ **直观控制**：Position Z直接控制立体强度
- ✅ **实时反馈**：立即看到视差变化

### 技术优势
- ✅ **物理准确**：模拟真实的立体视觉
- ✅ **简单实现**：只需偏移模型X位置
- ✅ **易于调试**：清晰的日志输出

### 舒适度
- ✅ **可控视差**：用户可以调整到最舒适的强度
- ✅ **避免冲突**：不干扰相机控制
- ✅ **灵活调节**：Z值可正可负，都能增强视差

## 📝 修改文件

- `src/tab/scene.rs`
  - 左窗口：偏移模型X位置（向右）
  - 右窗口：偏移模型X位置（向左）
  - 相机：保持不变

## 🎉 最终效果

现在VR模式提供了正确的立体视觉实现：
- ✅ 相机Z：鼠标滚轮控制，正常工作
- ✅ 模型X：自动偏移，创建视差
- ✅ 模型Z：控制视差强度
- ✅ 物理准确：符合真实立体视觉原理

用户可以：
1. 用鼠标滚轮调整观察距离
2. 用Position Z滑块调整立体效果强度
3. 获得最佳的VR观看体验！

