# GitHub Pages 部署指南

本指南将帮助您将 Rust WASM 项目部署到 GitHub Pages。

## 前提条件

- 您的项目已经托管在 GitHub 上
- 项目包含 `.github/workflows/deploy.yml` 工作流文件
- 您有仓库的管理员权限

## 详细部署步骤

### 第一步：启用 GitHub Pages

1. 打开您的 GitHub 仓库页面
2. 点击 **Settings**（设置）选项卡
3. 在左侧菜单中找到并点击 **Pages**
4. 在 **Source**（源）部分，选择 **GitHub Actions**
5. 点击 **Save**（保存）

### 第二步：确认工作流文件

确保您的仓库中存在 `.github/workflows/deploy.yml` 文件，内容应包含：

```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

# 设置 GITHUB_TOKEN 权限以允许部署到 GitHub Pages
permissions:
  contents: read
  pages: write
  id-token: write

# 只允许一个并发部署
concurrency:
  group: "pages"
  cancel-in-progress: false

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        targets: wasm32-unknown-unknown
    
    - name: Cache Rust dependencies
      uses: Swatinem/rust-cache@v2
    
    - name: Install Trunk
      uses: jetli/trunk-action@v0.4.0
      with:
        version: 'latest'
    
    - name: Build with Trunk
      run: trunk build --release --public-url /${{ github.event.repository.name }}/
    
    - name: Setup Pages
      uses: actions/configure-pages@v4
    
    - name: Upload artifact
      uses: actions/upload-pages-artifact@v3
      with:
        path: './dist'

  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    needs: build
    if: github.ref == 'refs/heads/main'
    steps:
    - name: Deploy to GitHub Pages
      id: deployment
      uses: actions/deploy-pages@v4
```

### 第三步：推送代码触发部署

1. 确保您的更改已提交到本地仓库：
   ```bash
   git add .
   git commit -m "添加 GitHub Pages 部署配置"
   ```

2. 推送到 `main` 分支：
   ```bash
   git push origin main
   ```

### 第四步：监控部署过程

1. 推送后，转到您的 GitHub 仓库
2. 点击 **Actions** 选项卡
3. 您应该看到一个名为 "Deploy to GitHub Pages" 的工作流正在运行
4. 点击工作流查看详细的构建和部署日志

### 第五步：访问您的应用

部署成功后：

1. 转到仓库的 **Settings** → **Pages**
2. 您会看到一个绿色的勾选标记和您的网站 URL
3. URL 格式为：`https://yourusername.github.io/your-repo-name/`
4. 点击链接访问您的 3D Gaussian Splatting 查看器应用

## 故障排除

### 常见问题

**问题 1：部署失败，显示权限错误**
- 解决方案：确保在仓库设置中启用了 GitHub Actions 的写入权限

**问题 2：应用加载但资源文件 404**
- 解决方案：检查 `trunk build` 命令中的 `--public-url` 参数是否正确

**问题 3：首次部署时间过长**
- 解决方案：这是正常的，Rust 依赖需要编译。后续部署会因为缓存而更快

**问题 4：WebGPU 不支持**
- 解决方案：确保使用支持 WebGPU 的现代浏览器

### 检查部署状态

您可以通过以下方式检查部署状态：

1. **GitHub Actions 页面**：查看工作流运行状态
2. **Pages 设置页面**：查看部署历史和当前状态
3. **仓库主页**：README 中的徽章会显示部署状态

## 自定义域名（可选）

如果您想使用自定义域名：

1. 在仓库根目录创建 `CNAME` 文件
2. 在文件中写入您的域名（如：`example.com`）
3. 在您的域名提供商处设置 DNS 记录指向 GitHub Pages

## 更新和维护

- 每次推送到 `main` 分支都会触发新的部署
- 部署通常需要 2-5 分钟完成
- 您可以在 Actions 页面查看所有部署历史

## 安全注意事项

- 工作流使用最小权限原则
- 只有推送到 `main` 分支才会触发生产部署
- 所有构建都在隔离的 GitHub Actions 环境中进行

---

现在您的 3D Gaussian Splatting 查看器应用已经成功部署到 GitHub Pages！任何人都可以通过您的 GitHub Pages URL 访问和使用您的应用。