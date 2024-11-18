# WQL 查詢與報告生成指南

本文檔說明如何使用 WQL API 端點來查詢資料並獲取 PDF 格式的報告。

## 使用方式

### 方式一：直接獲取 PDF（推薦）

這是最簡單的方式，只需要一個指令就可以直接獲取 PDF 報告：

```bash
curl -X POST "http://localhost:3001/wql/redteam2?format=pdf" -o report.pdf
```

參數說明：
- `-X POST`：使用 POST 方法
- `format=pdf`：指定直接返回 PDF 格式
- `-o report.pdf`：將結果保存為 report.pdf 檔案

這種方式的優點：
- 操作簡單，一步到位
- 直接獲得 PDF 檔案
- 不需要額外的處理步驟

### 方式二：分步驟獲取

如果您需要先查看報告的基本資訊，再決定是否下載 PDF，可以使用這種方式：

1. 首先獲取報告資訊：
```bash
curl -X POST http://localhost:3001/wql/redteam2
```

會得到類似這樣的 JSON 回應：
```json
{
    "status": "success",
    "group": "redteam2",
    "total_agents": 11,
    "report_file": "redteam2-20240118-123456.pdf",
    "pdf_url": "http://localhost:3001/reports/redteam2-20240118-123456.pdf",
    "note": "To get PDF directly, add ?format=pdf to the URL"
}
```

2. 然後使用回應中的 pdf_url 下載 PDF：
```bash
curl -O http://localhost:3001/reports/redteam2-20240118-123456.pdf
```

這種方式的優點：
- 可以先查看報告的基本資訊
- 可以選擇性下載 PDF
- 提供更多的中繼資訊

## 注意事項

1. PDF 檔案會自動以正確的 Content-Type（application/pdf）返回
2. 可以直接在瀏覽器中打開 PDF URL
3. 所有報告都會保存在伺服器的 reports 目錄中
4. 檔案名稱包含時間戳，確保唯一性

## 錯誤處理

如果遇到錯誤，系統會返回適當的錯誤訊息：

- 404 Not Found：找不到指定的 PDF 檔案
- 500 Internal Server Error：伺服器內部錯誤

## 建議使用方式

對於大多數使用情況，我們建議使用方式一（直接獲取 PDF），因為：
1. 操作更簡單
2. 減少網路請求次數
3. 直接得到所需的 PDF 檔案

但如果您的應用程式需要先檢查報告的基本資訊（如代理數量、警報數等），則可以使用方式二。
