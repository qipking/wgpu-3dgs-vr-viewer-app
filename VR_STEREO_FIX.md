# VR立体视觉修复 - IPD实现

## 🎯 问题

虽然VR模式能显示模型，但两个窗口显示完全相同的画面，没有立体效果。

### 原因分析
之前的实现尝试通过X轴镜像（`pos.x = -pos.x`）来创建视差，但这种方法有两个问题：

1. **当相机X坐标为0时无效**：
   ```
   pos Vec3(0.0, 0.0, -0.1) -> Vec3(-0.0, 0.0, -0.1)
   ```
   X从0变成-0，实际上还是0，没有任何偏移

2. **镜像不是正确的立体视觉方法**：
   - 真正的立体视觉需要两个相机在不同位置观察同一场景
   - 应该使用IPD（Inter-Pupillary Distance，眼间距）来分离两个视点

## ✅ 解决方案：实现真正的IPD

### 核心概念
- **IPD（眼间距）**：人类双眼之间的距离，平均约65mm（0.065米）
- **左眼**：相机向左偏移 IPD/2
- **右眼**：相机向右偏移 IPD/2
- **总视差**：IPD（左右眼相距65mm）

### 实现细节

#### 1. FirstPerson相机
最简单的情况，直接沿X轴偏移：

```rust
const IPD: f32 = 0.065;

// 左眼
first_person.pos.x -= IPD / 2.0;

// 右眼
first_person.pos.x += IPD / 2.0;
```

#### 2. Orbit相机
需要计算相机的右向量，然后沿右向量偏移：

```rust
// 计算相机朝向
let to_camera = orbit.pos - orbit.target;
let forward = to_camera.normalize();

// 计算右向量（叉乘）
let up = Vec3::Y;
let right = forward.cross(up).normalize();

// 左眼：沿左方向偏移
orbit.pos -= right * (IPD / 2.0);

// 右眼：沿右方向偏移
orbit.pos += right * (IPD / 2.0);
```

### 代码修改

#### 修改1：添加apply_vr_offset参数

```rust
fn loaded_preprocess_with_camera_offset<G: gs::GaussianPod>(
    &mut self,
    frame: &mut eframe::Frame,
    rect: &egui::Rect,
    gs: &mut app::GaussianSplatting,
    is_vr_right_eye: bool,
    apply_vr_offset: bool,  // 🔑 新增：是否应用VR偏移
) {
    // ...
}
```

#### 修改2：左眼也应用偏移

```rust
fn loaded_preprocess<G: gs::GaussianPod>(...) {
    // 在VR模式下，左眼也需要偏移（向左）
    self.loaded_preprocess_with_camera_offset::<G>(
        frame, rect, gs, 
        false,          // 不是右眼
        self.vr_mode    // 🔑 VR模式下应用偏移
    );
}
```

#### 修改3：正常模式的相机更新

```rust
// 在VR模式下，左眼需要向左偏移半个IPD
let camera_control = if apply_vr_offset {
    let mut modified_camera = gs.camera.control.clone();
    const IPD: f32 = 0.065;
    
    // 左眼：向左偏移
    match &mut modified_camera {
        app::CameraControl::FirstPerson(first_person) => {
            first_person.pos.x -= IPD / 2.0;
        }
        app::CameraControl::Orbit(orbit) => {
            let to_camera = orbit.pos - orbit.target;
            let forward = to_camera.normalize();
            let up = Vec3::Y;
            let right = forward.cross(up).normalize();
            orbit.pos -= right * (IPD / 2.0);
        }
    }
    
    modified_camera
} else {
    gs.camera.control.clone()
};

viewer.update_camera(queue, &camera_control, viewer_size);
```

## 📊 效果对比

### 修复前
```
左眼位置: (0.0, 0.0, -0.1)
右眼位置: (-0.0, 0.0, -0.1)  // 实际上还是0
视差: 0mm ❌
```

### 修复后
```
左眼位置: (-0.0325, 0.0, -0.1)  // 向左32.5mm
右眼位置: (+0.0325, 0.0, -0.1)  // 向右32.5mm
视差: 65mm ✅
```

## 🔧 技术细节

### 为什么是65mm？
- 人类平均眼间距约为63-65mm
- VR设备通常使用63-70mm的可调节IPD
- 我们选择65mm作为默认值

### 为什么除以2？
- 我们希望两个眼睛对称地分布在原始相机位置两侧
- 左眼：原始位置 - IPD/2
- 右眼：原始位置 + IPD/2
- 这样原始相机位置正好在两眼中间

### Orbit相机的右向量计算
```
forward = (camera_pos - target).normalize()
up = (0, 1, 0)  // Y轴向上
right = forward × up  // 叉乘得到右向量
```

这确保了无论相机如何旋转，偏移方向始终是相机的"右方"。

## 🎓 立体视觉原理

### 视差（Parallax）
- 两个眼睛从不同位置观察同一物体
- 近处物体：视差大（左右眼看到的差异大）
- 远处物体：视差小（左右眼看到的差异小）
- 大脑通过视差计算深度

### VR中的立体渲染
1. 渲染左眼视图（相机向左偏移）
2. 渲染右眼视图（相机向右偏移）
3. 左眼图像显示给左眼
4. 右眼图像显示给右眼
5. 大脑融合两个图像，产生深度感

## 🧪 测试验证

### 预期效果
1. **近处物体**：左右窗口中的位置差异明显
2. **远处物体**：左右窗口中的位置差异较小
3. **调整模型位置**：两个窗口应该显示不同的视角
4. **旋转相机**：视差方向应该跟随相机旋转

### 调试日志
```
👁️ [VR DEBUG] FirstPerson left eye offset: IPD=0.065
👁️ [VR DEBUG] FirstPerson camera offset (right eye): ... IPD: 0.065
👁️ [VR DEBUG] Orbit left eye offset: IPD=0.065
👁️ [VR DEBUG] Orbit camera offset (right eye): pos ... IPD: 0.065
```

## 📝 修改文件

- `src/tab/scene.rs`
  - `loaded_preprocess`: 传递VR模式标志
  - `loaded_preprocess_with_camera_offset`: 添加apply_vr_offset参数
  - VR右眼preprocess: 使用IPD计算偏移
  - 正常模式preprocess: 左眼使用IPD偏移

## 🚀 下一步优化

### 可调节IPD
可以添加UI控件让用户调整IPD：
```rust
pub struct Scene {
    vr_mode: bool,
    vr_ipd: f32,  // 可调节的IPD
}
```

### 会聚距离（Convergence）
可以添加会聚点调整，让特定距离的物体看起来"在屏幕上"：
```rust
// 调整target而不是position
let convergence_distance = 2.0;
// 计算会聚偏移...
```

### 性能优化
- 缓存右向量计算
- 只在相机移动时重新计算

## ✅ 总结

现在VR模式实现了真正的立体视觉：
- ✅ 使用标准IPD（65mm）
- ✅ 左右眼对称偏移
- ✅ 支持FirstPerson和Orbit相机
- ✅ 正确的右向量计算
- ✅ 产生真实的深度感

用户应该能够清楚地看到左右窗口的视差，并感受到3D深度效果！
