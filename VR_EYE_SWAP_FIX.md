# VR左右眼交换修复

## 🎯 问题

虽然立体效果已经实现，但左右眼定义反了：
- **左侧窗口**：显示的是右眼视角
- **右侧窗口**：显示的是左眼视角

## 🔍 原因分析

这是一个常见的VR开发问题。可能的原因：

1. **坐标系约定不同**：不同的3D引擎对"左"和"右"的定义可能不同
2. **相机朝向**：相机朝向不同时，"右向量"的方向也会不同
3. **渲染顺序**：左右眼的渲染顺序可能与预期相反

## ✅ 解决方案

简单地交换偏移方向：

### 修复前
```rust
// 左窗口
camera.pos.x -= IPD / 2.0;  // 向左偏移

// 右窗口  
camera.pos.x += IPD / 2.0;  // 向右偏移
```

### 修复后
```rust
// 左窗口
camera.pos.x += IPD / 2.0;  // 向右偏移（交换）

// 右窗口
camera.pos.x -= IPD / 2.0;  // 向左偏移（交换）
```

## 🔧 代码修改

### 左窗口（主viewer）
```rust
// 在VR模式下，左窗口应该是左眼，需要向左偏移
// 但实际测试发现需要反向，所以这里向右偏移
let camera_control = if apply_vr_offset {
    let mut modified_camera = gs.camera.control.clone();
    const IPD: f32 = 0.065;
    
    // 左窗口：向右偏移（修正后）
    match &mut modified_camera {
        app::CameraControl::FirstPerson(first_person) => {
            first_person.pos.x += IPD / 2.0;
        }
        app::CameraControl::Orbit(orbit) => {
            let right = calculate_right_vector(orbit);
            orbit.pos += right * (IPD / 2.0);
        }
    }
    
    modified_camera
} else {
    gs.camera.control.clone()
};
```

### 右窗口（VR viewer）
```rust
// 应用X轴偏移
// 右窗口应该是右眼，但实际测试发现需要反向，所以这里向左偏移
match &mut modified_camera_control {
    app::CameraControl::FirstPerson(first_person) => {
        // 右窗口：向左偏移（修正后）
        first_person.pos.x -= IPD / 2.0;
    }
    app::CameraControl::Orbit(orbit) => {
        let right = calculate_right_vector(orbit);
        // 右窗口：沿左向量偏移
        orbit.pos -= right * (IPD / 2.0);
    }
}
```

## 📊 验证方法

### 测试步骤
1. 开启VR模式
2. 调整模型的X轴位置（向左或向右移动）
3. 观察左右窗口的变化

### 预期结果
- **向右移动模型**：
  - 左窗口：模型向右移动更多（左眼看到的）
  - 右窗口：模型向右移动较少（右眼看到的）
  
- **向左移动模型**：
  - 左窗口：模型向左移动较少（左眼看到的）
  - 右窗口：模型向左移动更多（右眼看到的）

### 立体效果验证
- **近处物体**：左右窗口差异明显
- **远处物体**：左右窗口差异较小
- **正确的深度感**：大脑能够融合两个图像

## 🎓 技术说明

### 为什么需要交换？

这取决于坐标系的定义：

1. **右手坐标系 vs 左手坐标系**
   - OpenGL：右手坐标系（Z轴指向屏幕外）
   - DirectX：左手坐标系（Z轴指向屏幕内）
   - wgpu：可以配置

2. **相机朝向**
   - 如果相机朝向-Z方向，"右"是+X
   - 如果相机朝向+Z方向，"右"是-X

3. **渲染目标**
   - 左眼纹理可能映射到右侧显示
   - 右眼纹理可能映射到左侧显示

### 最佳实践

在VR开发中，最可靠的方法是：
1. **先实现基本偏移**
2. **实际测试**
3. **根据测试结果调整方向**

这比试图理解所有坐标系转换更实用。

## 📝 修改文件

- `src/tab/scene.rs`
  - 左窗口preprocess：交换偏移方向
  - 右窗口preprocess：交换偏移方向

## ✅ 最终效果

现在VR模式应该正确显示：
- ✅ 左侧窗口 = 左眼视角
- ✅ 右侧窗口 = 右眼视角
- ✅ 正确的立体深度感
- ✅ 符合人类视觉习惯

## 🚀 后续优化

### 可以添加的功能
1. **眼睛标签**：在窗口上显示"L"和"R"标签
2. **IPD调节**：让用户调整眼间距
3. **交换开关**：如果用户觉得反了，可以一键交换
4. **全屏VR模式**：支持VR头显的并排显示

### 示例代码（添加标签）
```rust
// 在左窗口添加标签
ui.painter().text(
    egui::pos2(10.0, 10.0),
    egui::Align2::LEFT_TOP,
    "L",
    egui::FontId::proportional(24.0),
    egui::Color32::WHITE,
);

// 在右窗口添加标签
ui.painter().text(
    egui::pos2(10.0, 10.0),
    egui::Align2::LEFT_TOP,
    "R",
    egui::FontId::proportional(24.0),
    egui::Color32::WHITE,
);
```

## 📚 参考资料

- [VR坐标系约定](https://developer.oculus.com/documentation/native/pc/dg-render/)
- [立体渲染最佳实践](https://docs.unity3d.com/Manual/SinglePassStereoRendering.html)
- [IPD标准值](https://en.wikipedia.org/wiki/Pupillary_distance)
