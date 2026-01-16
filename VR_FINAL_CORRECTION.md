# VR左右眼最终修正

## 问题
使用Parallax滑块后，左右眼又反了：左边显示右眼画面。

## 解决方案
交换模型偏移方向：

### 修正前
```rust
// 左窗口：模型向右偏移
modified_pos.x += effective_ipd / 2.0;

// 右窗口：模型向左偏移
modified_pos.x -= effective_ipd / 2.0;
```

### 修正后
```rust
// 左窗口：模型向左偏移
modified_pos.x -= effective_ipd / 2.0;

// 右窗口：模型向右偏移
modified_pos.x += effective_ipd / 2.0;
```

## 验证方法
调整Parallax滑块时：
- **增大Parallax**：左右窗口差异增大
- **左窗口**：应该显示左眼视角（模型稍微偏左）
- **右窗口**：应该显示右眼视角（模型稍微偏右）

## 最终效果
✅ 左侧窗口 = 左眼视角
✅ 右侧窗口 = 右眼视角
✅ Parallax滑块正确控制视差强度
✅ 所有其他控制（相机、模型位置）正常工作
