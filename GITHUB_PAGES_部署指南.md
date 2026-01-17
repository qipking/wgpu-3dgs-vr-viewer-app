# GitHub Pages 部署指南

本指南将帮助您将 Rust WASM 项目部署到 GitHub Pages。

## 问题解决总结

我已经为您解决了以下问题：

### ✅ 已修复的问题

1. **GitHub Actions 工作流配置**
   - 移除了 pull_request 触发器，只在推送到 main 分支时部署
   - 简化了部署条件，避免权限问题

2. **代码格式问题**
   - 修复了 `src/tab/scene.rs` 中的尾随空白问题
   - 所有 Rust 代码现在符合格式标准

3. **编译检查**
   - 确认 WASM 目标编译正常
   - 项目可以成功构建

### 📋 部署检查清单

在推送到 GitHub 之前，请确认：

- [ ] 代码格式正确：`cargo fmt` 无错误
- [ ] 编译成功：`cargo check --target wasm32-unknown-unknown` 无错误
- [ ] GitHub Pages 已在仓库设置中启用
- [ ] 选择了 "GitHub Actions" 作为部署源

## 快速部署步骤

### 第一步：启用 GitHub Pages

1. 打开您的 GitHub 仓库页面
2. 点击 **Settings**（设置）选项卡
3. 在左侧菜单中找到并点击 **Pages**
4. 在 **Source**（源）部分，选择 **GitHub Actions**
5. 点击 **Save**（保存）

### 第二步：推送代码触发部署

```bash
git add .
git commit -m "添加 GitHub Pages 部署配置"
git push origin main
```

### 第三步：等待部署完成

1. 转到仓库的 **Actions** 选项卡
2. 查看 "Deploy to GitHub Pages" 工作流状态
3. 等待构建和部署完成（通常需要 2-5 分钟）

### 第四步：访问您的应用

部署成功后，您的应用将在以下地址可用：
`https://yourusername.github.io/your-repo-name/`

## 故障排除

### 常见问题解决方案

**问题：部署失败，显示 "Pages site failed"**
- 解决方案：确保在仓库设置中正确启用了 GitHub Pages，并选择了 "GitHub Actions" 作为源

**问题：代码格式错误**
- 解决方案：运行 `cargo fmt` 修复格式问题

**问题：编译错误**
- 解决方案：运行 `cargo check` 检查并修复编译错误

**问题：首次部署时间过长**
- 解决方案：这是正常的，Rust 依赖需要编译。后续部署会更快

## 注意事项

- 只有推送到 `main` 分支才会触发部署
- 确保您的代码没有格式和编译错误
- 部署完成后，可能需要几分钟才能在浏览器中看到更新

---

现在您的 3D Gaussian Splatting 查看器应用已经可以部署到 GitHub Pages！