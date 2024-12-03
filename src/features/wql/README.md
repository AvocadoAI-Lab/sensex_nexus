# WQL 查詢與報告生成指南

本文檔說明如何使用 WQL API 端點來查詢資料並獲取 PDF 格式的報告。

## 技術實現說明

### 數據傳輸機制

系統使用優化的數據傳輸策略，特別針對GCP環境進行了改進：

1. 獨立連接模式：
   - 每個Agent使用獨立的連接進行數據傳輸
   - 避免多個Agent共用連接時超過GCP的單一連接限制(10MB/s)
   - 確保數據傳輸的穩定性和完整性

2. 分塊傳輸：
   - 使用64KB的chunks進行數據傳輸
   - 避免大量數據一次性傳輸造成的問題
   - 提高傳輸穩定性和可靠性

3. 錯誤處理：
   - 實現了指數退避和隨機抖動的重試機制
   - 最多重試5次，避免網絡波動影響
   - 智能調整重試間隔，防止重試風暴

4. 數據流程：
   - Wazuh → sensex_nexus：獨立連接獲取每個Agent的數據
   - sensex_nexus內部：整合所有Agent的數據
   - sensex_nexus → generate-report：發送完整的整合數據

## 使用方式

### 方式一：直接獲取 PDF（推薦）

這是最簡單的方式，只需要一個指令就可以直接獲取 PDF 報告：
curl -X POST http://localhost:29000/wql/poc2?format=pdf&report_type=monthly

```bash
# 獲取每日報告（預設）
curl -X POST "http://localhost:3001/wql/redteam2?format=pdf" -o report.pdf

# 獲取每週報告
curl -X POST "http://localhost:3001/wql/redteam2?format=pdf&report_type=weekly" -o weekly_report.pdf

# 獲取每月報告
curl -X POST "http://localhost:3001/wql/redteam2?format=pdf&report_type=monthly" -o monthly_report.pdf
```

參數說明：
- `-X POST`：使用 POST 方法
- `format=pdf`：指定直接返回 PDF 格式
- `report_type`：指定報告類型（可選）
  - `daily`：每日報告（默認）
  - `weekly`：每週報告
  - `monthly`：每月報告
- `-o report.pdf`：將結果保存為 report.pdf 檔案

這種方式的優點：
- 操作簡單，一步到位
- 直接獲得 PDF 檔案
- 不需要額外的處理步驟

### 方式二：分步驟獲取

如果您需要先查看報告的基本資訊，再決定是否下載 PDF，可以使用這種方式：

1. 首先獲取報告資訊：
```bash
# 獲取每日報告資訊（預設）
curl -X POST http://localhost:3001/wql/redteam2

# 獲取每週報告資訊
curl -X POST "http://localhost:3001/wql/redteam2?report_type=weekly"

# 獲取每月報告資訊
curl -X POST "http://localhost:3001/wql/redteam2?report_type=monthly"
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

## 報告類型說明

系統提供三種不同時間範圍的報告：

1. 每日報告（daily）：
   - 預設選項
   - 顯示最近24小時內的警報
   - 適合日常監控和快速回應

2. 每週報告（weekly）：
   - 顯示最近一週的警報
   - 適合週期性審查和趨勢分析
   - 包含更長時間範圍的數據

3. 每月報告（monthly）：
   - 顯示最近一個月的警報
   - 適合長期趨勢分析和月度總結
   - 提供最全面的數據概覽

## 注意事項

1. PDF 檔案會自動以正確的 Content-Type（application/pdf）返回
2. 可以直接在瀏覽器中打開 PDF URL
3. 所有報告都會保存在伺服器的 reports 目錄中
4. 檔案名稱包含時間戳，確保唯一性
5. 如果未指定 report_type，系統默認使用每日報告（daily）

## 錯誤處理

如果遇到錯誤，系統會返回適當的錯誤訊息：

- 404 Not Found：找不到指定的 PDF 檔案
- 500 Internal Server Error：伺服器內部錯誤

## 建議使用方式

對於大多數使用情況，我們建議：

1. 日常監控：
   - 使用每日報告（daily）
   - 直接使用 format=pdf 參數獲取 PDF

2. 週期性審查：
   - 使用每週報告（weekly）或每月報告（monthly）
   - 可以先獲取 JSON 回應查看基本資訊

3. 自動化腳本：
   - 使用完整的 URL 參數
   - 根據需求選擇適當的報告類型
   - 建議加入錯誤處理機制

## 性能考量

1. 大量Agent場景：
   - 系統使用獨立連接模式，每個Agent單獨處理
   - 避免了GCP環境下的數據截斷問題
   - 建議根據Agent數量調整查詢頻率

2. 網絡穩定性：
   - 實現了智能重試機制
   - 使用分塊傳輸提高可靠性
   - 自動處理網絡波動情況

3. 資源使用：
   - 數據在sensex_nexus中完成整合
   - generate-report接收完整數據集
   - 避免重複的數據傳輸
