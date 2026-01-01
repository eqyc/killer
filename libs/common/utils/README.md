# Utils

通用工具函数库，提供日期处理、字符串处理、验证工具等常用功能。这些工具函数是无状态的纯函数，便于测试和复用。

## 主要导出模块

| 模块 | 说明 |
|------|------|
| `date_utils` | 日期时间处理（格式化、解析、计算） |
| `string_utils` | 字符串处理（截断、填充、转换） |
| `validation` | 通用验证规则（邮箱、手机、身份证等） |
| `mask` | 数据脱敏（手机号、身份证、银行卡） |
| `crypto` | 加密工具（哈希、加密、解密） |

## 使用示例

```text
// 日期工具
let fiscal_year = date_utils::get_fiscal_year(date, FiscalYearVariant::April);
let work_days = date_utils::count_work_days(start, end, &holidays);

// 字符串工具
let padded = string_utils::left_pad("123", 10, '0'); // "0000000123"

// 验证工具
validation::is_valid_email("user@example.com"); // true
validation::is_valid_phone_cn("13800138000"); // true

// 数据脱敏
mask::phone("13800138000"); // "138****8000"
mask::id_card("110101199001011234"); // "1101**********1234"
```

## 设计原则

- **纯函数**：无副作用，输入决定输出
- **惰性初始化**：正则表达式等资源按需加载
- **零分配**：尽可能避免不必要的内存分配
