# 專案架構說明

## 整體架構

本專案採用模組化的架構設計，使用 Axum 框架建構 RESTful API 服務。主要功能模組都位於 `features` 目錄下，每個功能模組都遵循相同的結構模式。

## 目錄結構

每個功能模組都包含以下標準檔案結構：

```
features/
├── {feature_name}/
│   ├── mod.rs       # 模組入口點
│   ├── handlers.rs  # 請求處理邏輯
│   ├── models.rs    # 資料模型定義
│   └── routes.rs    # 路由配置
```

## 功能模組

目前包含的主要功能模組：

- **agents**: 代理程式管理
- **auth**: 身份驗證與授權
- **ciscat**: CIS-CAT 掃描與評估
- **decoders**: 日誌解碼器
- **groups**: 群組管理
- **lists**: 清單管理
- **manager**: 系統管理
- **mitre**: MITRE ATT&CK 框架整合
- **rootcheck**: Root 權限檢查
- **rules**: 規則管理
- **sca**: 安全配置評估
- **security**: 安全性管理
- **syscheck**: 系統檔案完整性檢查
- **syscollector**: 系統資訊收集
- **tasks**: 任務管理
- **wql**: WQL 查詢處理

## 模組結構說明

### mod.rs
- 作為模組的入口點
- 匯出模組內的公共介面
- 整合其他子模組

### handlers.rs
- 包含所有 HTTP 請求的處理邏輯
- 實作業務邏輯
- 處理請求驗證和錯誤處理

### models.rs
- 定義資料結構和型別
- 包含序列化和反序列化邏輯
- 實作資料模型的相關方法

### routes.rs
- 定義 API 路由
- 將請求映射到對應的處理函數
- 配置中間件和路由群組

## API 整合

所有功能模組的路由都在 `lib.rs` 中通過 `create_router()` 函數整合：

```rust
Router::new()
    .merge(features::agents::routes())
    .merge(features::auth::routes())
    // ... 其他模組路由
```

## 開發指南

1. 新增功能時，請遵循現有的模組結構
2. 確保實作適當的錯誤處理和日誌記錄
3. 遵循 RESTful API 設計原則
4. 保持代碼風格一致性
5. 新增功能時記得在 `lib.rs` 中整合路由

## 測試

所有功能模組的測試都位於 `tests` 目錄下，確保在開發新功能時編寫對應的測試案例。
