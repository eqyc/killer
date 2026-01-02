//! 领域原语错误类型定义
//!
//! 本模块定义了领域原语验证和操作过程中可能产生的所有错误类型。
//! 使用 `thiserror` 派生宏实现标准错误特征。
//!
//! # SAP 参考
//! 错误设计参考 SAP 的消息类（Message Class）机制，
//! 每个错误都有明确的错误码和描述信息。

use thiserror::Error;

/// 领域原语错误类型
///
/// 包含所有领域原语在验证和操作过程中可能产生的错误。
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DomainError {
    // =========================================================================
    // 币种相关错误
    // =========================================================================
    /// 无效的币种代码
    ///
    /// 币种代码必须符合 ISO 4217 标准（3位大写字母）
    #[error("无效的币种代码: {0}，必须为 ISO 4217 标准的3位大写字母")]
    InvalidCurrencyCode(String),

    // =========================================================================
    // 金额相关错误
    // =========================================================================
    /// 金额精度超出限制
    ///
    /// 金额最多支持 4 位小数（参考 SAP CURR 类型）
    #[error("金额精度超出限制: {0}，最多支持4位小数")]
    MoneyPrecisionExceeded(String),

    /// 币种不匹配
    ///
    /// 进行金额运算时，两个金额的币种必须相同
    #[error("币种不匹配: 期望 {expected}，实际 {actual}")]
    CurrencyMismatch {
        /// 期望的币种
        expected: String,
        /// 实际的币种
        actual: String,
    },

    /// 除数为零
    #[error("除数不能为零")]
    DivisionByZero,

    /// 金额溢出
    #[error("金额计算溢出")]
    MoneyOverflow,

    // =========================================================================
    // 计量单位相关错误
    // =========================================================================
    /// 无效的计量单位代码
    #[error("无效的计量单位代码: {0}")]
    InvalidUnitOfMeasure(String),

    /// 单位不兼容，无法换算
    ///
    /// 只有相同维度的单位才能进行换算（如重量单位之间、长度单位之间）
    #[error("单位不兼容，无法换算: {from} -> {to}")]
    IncompatibleUnits {
        /// 源单位
        from: String,
        /// 目标单位
        to: String,
    },

    // =========================================================================
    // 数量相关错误
    // =========================================================================
    /// 数量精度超出限制
    ///
    /// 数量最多支持 3 位小数（参考 SAP QUAN 类型）
    #[error("数量精度超出限制: {0}，最多支持3位小数")]
    QuantityPrecisionExceeded(String),

    /// 单位不匹配
    ///
    /// 进行数量运算时，两个数量的单位必须相同或可换算
    #[error("单位不匹配: 期望 {expected}，实际 {actual}")]
    UnitMismatch {
        /// 期望的单位
        expected: String,
        /// 实际的单位
        actual: String,
    },

    /// 数量溢出
    #[error("数量计算溢出")]
    QuantityOverflow,

    // =========================================================================
    // 会计科目相关错误
    // =========================================================================
    /// 无效的科目代码
    ///
    /// 科目代码必须为 1-10 位字母数字（参考 SAP SKB1.SAKNR）
    #[error("无效的科目代码: {0}，必须为1-10位字母数字")]
    InvalidAccountCode(String),

    /// 无效的科目表
    ///
    /// 科目表代码必须为 4 位字母数字（参考 SAP T004.KTOPL）
    #[error("无效的科目表代码: {0}，必须为4位字母数字")]
    InvalidChartOfAccounts(String),

    // =========================================================================
    // 物料编号相关错误
    // =========================================================================
    /// 无效的物料编号
    ///
    /// 物料编号必须为 1-18 位字母数字（参考 SAP MARA.MATNR）
    #[error("无效的物料编号: {0}，必须为1-18位字母数字")]
    InvalidMaterialNumber(String),

    // =========================================================================
    // 凭证编号相关错误
    // =========================================================================
    /// 无效的凭证编号
    ///
    /// 凭证编号必须为 1-10 位数字（参考 SAP BKPF.BELNR）
    #[error("无效的凭证编号: {0}，必须为1-10位数字")]
    InvalidDocumentNumber(String),

    /// 无效的会计年度
    ///
    /// 会计年度必须为 4 位数字，范围 1900-2099
    #[error("无效的会计年度: {0}，必须为1900-2099之间的4位数字")]
    InvalidFiscalYear(i32),

    /// 无效的凭证类型
    ///
    /// 凭证类型必须为 2 位字母数字（参考 SAP BKPF.BLART）
    #[error("无效的凭证类型: {0}，必须为2位字母数字")]
    InvalidDocumentType(String),

    // =========================================================================
    // 会计期间相关错误
    // =========================================================================
    /// 无效的会计期间
    ///
    /// 会计期间必须为 1-16（SAP 支持 12 个正常期间 + 4 个特殊期间）
    #[error("无效的会计期间: {0}，必须为1-16")]
    InvalidFiscalPeriod(i32),

    // =========================================================================
    // 公司代码相关错误
    // =========================================================================
    /// 无效的公司代码
    ///
    /// 公司代码必须为 4 位字母数字（参考 SAP T001.BUKRS）
    #[error("无效的公司代码: {0}，必须为4位字母数字")]
    InvalidCompanyCode(String),

    // =========================================================================
    // 工厂相关错误
    // =========================================================================
    /// 无效的工厂代码
    ///
    /// 工厂代码必须为 4 位字母数字（参考 SAP T001W.WERKS）
    #[error("无效的工厂代码: {0}，必须为4位字母数字")]
    InvalidPlantCode(String),

    // =========================================================================
    // 成本中心相关错误
    // =========================================================================
    /// 无效的成本中心代码
    ///
    /// 成本中心代码必须为 1-10 位字母数字（参考 SAP CSKS.KOSTL）
    #[error("无效的成本中心代码: {0}，必须为1-10位字母数字")]
    InvalidCostCenterCode(String),

    /// 无效的控制范围
    ///
    /// 控制范围代码必须为 4 位字母数字（参考 SAP TKA01.KOKRS）
    #[error("无效的控制范围代码: {0}，必须为4位字母数字")]
    InvalidControllingArea(String),

    // =========================================================================
    // 百分比相关错误
    // =========================================================================
    /// 百分比超出范围
    ///
    /// 百分比值必须在 0-100 之间（或根据业务场景允许负值）
    #[error("百分比超出范围: {0}，必须在0-100之间")]
    PercentageOutOfRange(String),

    /// 百分比精度超出限制
    #[error("百分比精度超出限制: {0}，最多支持2位小数")]
    PercentagePrecisionExceeded(String),
}

/// 领域原语结果类型别名
pub type DomainResult<T> = Result<T, DomainError>;

// ============================================================================
// 错误工厂方法
// ============================================================================

impl DomainError {
    // -------------------------------------------------------------------------
    // 币种相关错误
    // -------------------------------------------------------------------------

    /// 创建无效币种代码错误
    pub fn currency_code_invalid(code: impl Into<String>, _reason: impl Into<String>) -> Self {
        DomainError::InvalidCurrencyCode(code.into())
    }

    // -------------------------------------------------------------------------
    // 金额相关错误
    // -------------------------------------------------------------------------

    /// 创建无效金额错误
    pub fn money_invalid_amount(reason: impl Into<String>) -> Self {
        DomainError::MoneyPrecisionExceeded(reason.into())
    }

    /// 创建币种不匹配错误
    pub fn money_currency_mismatch(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        DomainError::CurrencyMismatch {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    /// 创建金额操作无效错误
    pub fn money_invalid_operation(reason: impl Into<String>) -> Self {
        DomainError::MoneyPrecisionExceeded(reason.into())
    }

    // -------------------------------------------------------------------------
    // 计量单位相关错误
    // -------------------------------------------------------------------------

    /// 创建无效计量单位代码错误
    pub fn unit_of_measure_invalid_code(code: impl Into<String>, _reason: impl Into<String>) -> Self {
        DomainError::InvalidUnitOfMeasure(code.into())
    }

    /// 创建单位维度不兼容错误
    pub fn unit_of_measure_incompatible_dimension(
        from: impl Into<String>,
        to: impl Into<String>,
    ) -> Self {
        DomainError::IncompatibleUnits {
            from: from.into(),
            to: to.into(),
        }
    }

    // -------------------------------------------------------------------------
    // 数量相关错误
    // -------------------------------------------------------------------------

    /// 创建无效数量错误
    pub fn quantity_invalid_value(reason: impl Into<String>) -> Self {
        DomainError::QuantityPrecisionExceeded(reason.into())
    }

    /// 创建数量操作无效错误
    pub fn quantity_invalid_operation(reason: impl Into<String>) -> Self {
        DomainError::QuantityPrecisionExceeded(reason.into())
    }

    // -------------------------------------------------------------------------
    // 会计科目相关错误
    // -------------------------------------------------------------------------

    /// 创建无效科目代码错误
    pub fn account_code_invalid(code: impl Into<String>, _reason: impl Into<String>) -> Self {
        DomainError::InvalidAccountCode(code.into())
    }

    // -------------------------------------------------------------------------
    // 物料编号相关错误
    // -------------------------------------------------------------------------

    /// 创建无效物料编号错误
    pub fn material_number_invalid(number: impl Into<String>, _reason: impl Into<String>) -> Self {
        DomainError::InvalidMaterialNumber(number.into())
    }

    // -------------------------------------------------------------------------
    // 凭证编号相关错误
    // -------------------------------------------------------------------------

    /// 创建无效凭证编号错误
    pub fn document_number_invalid(number: impl Into<String>, _reason: impl Into<String>) -> Self {
        DomainError::InvalidDocumentNumber(number.into())
    }

    // -------------------------------------------------------------------------
    // 会计期间相关错误
    // -------------------------------------------------------------------------

    /// 创建无效会计期间错误
    pub fn fiscal_period_invalid(_year: i32, period: i32, _reason: impl Into<String>) -> Self {
        DomainError::InvalidFiscalPeriod(period)
    }

    // -------------------------------------------------------------------------
    // 公司代码相关错误
    // -------------------------------------------------------------------------

    /// 创建无效公司代码错误
    pub fn company_code_invalid(code: impl Into<String>, _reason: impl Into<String>) -> Self {
        DomainError::InvalidCompanyCode(code.into())
    }

    // -------------------------------------------------------------------------
    // 工厂相关错误
    // -------------------------------------------------------------------------

    /// 创建无效工厂代码错误
    pub fn plant_invalid(code: impl Into<String>, _reason: impl Into<String>) -> Self {
        DomainError::InvalidPlantCode(code.into())
    }

    // -------------------------------------------------------------------------
    // 成本中心相关错误
    // -------------------------------------------------------------------------

    /// 创建无效成本中心代码错误
    pub fn cost_center_invalid(code: impl Into<String>, _reason: impl Into<String>) -> Self {
        DomainError::InvalidCostCenterCode(code.into())
    }

    // -------------------------------------------------------------------------
    // 百分比相关错误
    // -------------------------------------------------------------------------

    /// 创建无效百分比错误
    pub fn percentage_invalid(reason: impl Into<String>) -> Self {
        DomainError::PercentageOutOfRange(reason.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = DomainError::InvalidCurrencyCode("INVALID".to_string());
        assert!(err.to_string().contains("INVALID"));
        assert!(err.to_string().contains("ISO 4217"));
    }

    #[test]
    fn test_currency_mismatch_error() {
        let err = DomainError::CurrencyMismatch {
            expected: "CNY".to_string(),
            actual: "USD".to_string(),
        };
        assert!(err.to_string().contains("CNY"));
        assert!(err.to_string().contains("USD"));
    }

    #[test]
    fn test_error_equality() {
        let err1 = DomainError::DivisionByZero;
        let err2 = DomainError::DivisionByZero;
        assert_eq!(err1, err2);
    }
}
