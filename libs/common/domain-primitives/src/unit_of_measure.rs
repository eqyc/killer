//! 计量单位（Unit of Measure）
//!
//! 本模块实现计量单位值对象，支持单位换算。
//!
//! # SAP 参考
//! - 表: T006（计量单位主数据）
//! - 表: T006A（计量单位描述）
//! - 表: T006D（计量单位换算）
//! - 字段: MSEHI（内部计量单位代码）
//! - 字段: ISOCODE（ISO 计量单位代码）
//!
//! # 维度（Dimension）
//! SAP 将计量单位按维度分类：
//! - MASS: 质量（KG, G, T）
//! - LENGTH: 长度（M, CM, MM）
//! - VOLUME: 体积（L, ML, M3）
//! - TIME: 时间（H, MIN, S）
//! - QUANTITY: 数量/个数（PC, EA）
//!
//! # 示例
//! ```rust
//! use killer_domain_primitives::{UnitOfMeasure, Dimension};
//!
//! // 创建计量单位
//! let kg = UnitOfMeasure::kilogram();
//! let g = UnitOfMeasure::gram();
//!
//! // 获取换算因子
//! let factor = kg.conversion_factor_to(&g).unwrap();
//! assert_eq!(factor, rust_decimal::Decimal::from(1000));
//! ```

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::{DomainError, DomainResult};

/// 计量单位维度
///
/// 定义计量单位的物理维度类型，用于验证单位换算的合法性。
/// 只有相同维度的单位之间才能进行换算。
///
/// # SAP 参考
/// 对应 T006 表的 DIMID 字段
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Dimension {
    /// 质量维度（KG, G, T, LB 等）
    Mass,
    /// 长度维度（M, CM, MM, KM 等）
    Length,
    /// 体积维度（L, ML, M3 等）
    Volume,
    /// 面积维度（M2, CM2 等）
    Area,
    /// 时间维度（H, MIN, S, DAY 等）
    Time,
    /// 数量/计件维度（PC, EA, SET 等）
    Quantity,
    /// 温度维度（CEL, FAH, KEL）
    Temperature,
    /// 电力维度（KWH, MWH 等）
    Energy,
    /// 货币维度（用于价格单位）
    Currency,
    /// 无量纲（百分比、比率等）
    Dimensionless,
}

impl Dimension {
    /// 获取维度的中文描述
    pub fn description_zh(&self) -> &'static str {
        match self {
            Dimension::Mass => "质量",
            Dimension::Length => "长度",
            Dimension::Volume => "体积",
            Dimension::Area => "面积",
            Dimension::Time => "时间",
            Dimension::Quantity => "数量",
            Dimension::Temperature => "温度",
            Dimension::Energy => "能量",
            Dimension::Currency => "货币",
            Dimension::Dimensionless => "无量纲",
        }
    }

    /// 获取维度的英文描述
    pub fn description_en(&self) -> &'static str {
        match self {
            Dimension::Mass => "Mass",
            Dimension::Length => "Length",
            Dimension::Volume => "Volume",
            Dimension::Area => "Area",
            Dimension::Time => "Time",
            Dimension::Quantity => "Quantity",
            Dimension::Temperature => "Temperature",
            Dimension::Energy => "Energy",
            Dimension::Currency => "Currency",
            Dimension::Dimensionless => "Dimensionless",
        }
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description_en())
    }
}

/// 计量单位
///
/// 表示一个计量单位，包含代码、描述、维度和换算基准。
/// 这是一个不可变的值对象。
///
/// # SAP 参考
/// - 表: T006（计量单位定义）
/// - 表: T006A（计量单位文本）
/// - 表: T006D（维度定义）
///
/// # 换算机制
/// 每个单位都有一个 `base_factor`，表示该单位与其维度基准单位的换算比例。
/// 例如：
/// - KG 的 base_factor = 1（质量维度的基准单位）
/// - G 的 base_factor = 0.001（1G = 0.001KG）
/// - T 的 base_factor = 1000（1T = 1000KG）
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UnitOfMeasure {
    /// SAP 内部代码（如 "KG", "PC"）
    /// 对应 T006.MSEHI
    code: String,

    /// ISO 代码（如 "KGM", "PCE"）
    /// 对应 T006.ISOCODE
    iso_code: Option<String>,

    /// 单位描述
    /// 对应 T006A.MSEHL
    description: String,

    /// 物理维度
    /// 对应 T006.DIMID
    dimension: Dimension,

    /// 换算到基准单位的因子
    /// 对应 T006.ZAEHL / T006.NENNR
    #[serde(with = "rust_decimal::serde::str")]
    base_factor: Decimal,
}

impl UnitOfMeasure {
    /// 创建新的计量单位
    ///
    /// # 参数
    /// - `code`: SAP 内部代码（1-3 个字符）
    /// - `description`: 单位描述
    /// - `dimension`: 物理维度
    /// - `base_factor`: 换算到基准单位的因子
    ///
    /// # 错误
    /// - 代码为空或超过 3 个字符
    /// - 换算因子为零或负数
    pub fn new(
        code: impl Into<String>,
        description: impl Into<String>,
        dimension: Dimension,
        base_factor: Decimal,
    ) -> DomainResult<Self> {
        let code = code.into().to_uppercase();
        let description = description.into();

        // 验证代码
        if code.is_empty() {
            return Err(DomainError::unit_of_measure_invalid_code(
                code,
                "代码不能为空",
            ));
        }
        if code.len() > 6 {
            return Err(DomainError::unit_of_measure_invalid_code(
                code,
                "代码长度不能超过 6 个字符",
            ));
        }

        // 验证换算因子
        if base_factor <= Decimal::ZERO {
            return Err(DomainError::unit_of_measure_invalid_code(
                code,
                "换算因子必须为正数",
            ));
        }

        Ok(Self {
            code,
            iso_code: None,
            description,
            dimension,
            base_factor,
        })
    }

    /// 创建带 ISO 代码的计量单位
    pub fn with_iso_code(mut self, iso_code: impl Into<String>) -> Self {
        self.iso_code = Some(iso_code.into().to_uppercase());
        self
    }

    /// 获取 SAP 内部代码
    pub fn code(&self) -> &str {
        &self.code
    }

    /// 获取 ISO 代码
    pub fn iso_code(&self) -> Option<&str> {
        self.iso_code.as_deref()
    }

    /// 获取单位描述
    pub fn description(&self) -> &str {
        &self.description
    }

    /// 获取物理维度
    pub fn dimension(&self) -> Dimension {
        self.dimension
    }

    /// 获取换算到基准单位的因子
    pub fn base_factor(&self) -> Decimal {
        self.base_factor
    }

    /// 计算到目标单位的换算因子
    ///
    /// # 参数
    /// - `target`: 目标计量单位
    ///
    /// # 返回
    /// 换算因子，使得 `self_value * factor = target_value`
    ///
    /// # 错误
    /// - 两个单位的维度不同
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::UnitOfMeasure;
    ///
    /// let kg = UnitOfMeasure::kilogram();
    /// let g = UnitOfMeasure::gram();
    ///
    /// // 1 KG = 1000 G
    /// let factor = kg.conversion_factor_to(&g).unwrap();
    /// assert_eq!(factor, rust_decimal::Decimal::from(1000));
    /// ```
    pub fn conversion_factor_to(&self, target: &UnitOfMeasure) -> DomainResult<Decimal> {
        if self.dimension != target.dimension {
            return Err(DomainError::unit_of_measure_incompatible_dimension(
                self.code.clone(),
                target.code.clone(),
            ));
        }

        // 换算公式: self_value * (self.base_factor / target.base_factor) = target_value
        Ok(self.base_factor / target.base_factor)
    }

    /// 检查是否可以换算到目标单位
    pub fn can_convert_to(&self, target: &UnitOfMeasure) -> bool {
        self.dimension == target.dimension
    }

    /// 转换到 ISO 代码格式
    ///
    /// 如果有 ISO 代码则返回 ISO 代码，否则返回 SAP 内部代码
    pub fn to_iso_code(&self) -> &str {
        self.iso_code.as_deref().unwrap_or(&self.code)
    }
}

// ============================================================================
// 预定义的常用计量单位
// ============================================================================

impl UnitOfMeasure {
    // ------------------------------------------------------------------------
    // 质量单位（基准单位: KG）
    // ------------------------------------------------------------------------

    /// 千克（基准质量单位）
    pub fn kilogram() -> Self {
        Self {
            code: "KG".to_string(),
            iso_code: Some("KGM".to_string()),
            description: "千克".to_string(),
            dimension: Dimension::Mass,
            base_factor: dec!(1),
        }
    }

    /// 克
    pub fn gram() -> Self {
        Self {
            code: "G".to_string(),
            iso_code: Some("GRM".to_string()),
            description: "克".to_string(),
            dimension: Dimension::Mass,
            base_factor: dec!(0.001),
        }
    }

    /// 毫克
    pub fn milligram() -> Self {
        Self {
            code: "MG".to_string(),
            iso_code: Some("MGM".to_string()),
            description: "毫克".to_string(),
            dimension: Dimension::Mass,
            base_factor: dec!(0.000001),
        }
    }

    /// 吨
    pub fn ton() -> Self {
        Self {
            code: "T".to_string(),
            iso_code: Some("TNE".to_string()),
            description: "吨".to_string(),
            dimension: Dimension::Mass,
            base_factor: dec!(1000),
        }
    }

    /// 磅
    pub fn pound() -> Self {
        Self {
            code: "LB".to_string(),
            iso_code: Some("LBR".to_string()),
            description: "磅".to_string(),
            dimension: Dimension::Mass,
            base_factor: dec!(0.45359237),
        }
    }

    // ------------------------------------------------------------------------
    // 长度单位（基准单位: M）
    // ------------------------------------------------------------------------

    /// 米（基准长度单位）
    pub fn meter() -> Self {
        Self {
            code: "M".to_string(),
            iso_code: Some("MTR".to_string()),
            description: "米".to_string(),
            dimension: Dimension::Length,
            base_factor: dec!(1),
        }
    }

    /// 厘米
    pub fn centimeter() -> Self {
        Self {
            code: "CM".to_string(),
            iso_code: Some("CMT".to_string()),
            description: "厘米".to_string(),
            dimension: Dimension::Length,
            base_factor: dec!(0.01),
        }
    }

    /// 毫米
    pub fn millimeter() -> Self {
        Self {
            code: "MM".to_string(),
            iso_code: Some("MMT".to_string()),
            description: "毫米".to_string(),
            dimension: Dimension::Length,
            base_factor: dec!(0.001),
        }
    }

    /// 千米
    pub fn kilometer() -> Self {
        Self {
            code: "KM".to_string(),
            iso_code: Some("KMT".to_string()),
            description: "千米".to_string(),
            dimension: Dimension::Length,
            base_factor: dec!(1000),
        }
    }

    /// 英寸
    pub fn inch() -> Self {
        Self {
            code: "IN".to_string(),
            iso_code: Some("INH".to_string()),
            description: "英寸".to_string(),
            dimension: Dimension::Length,
            base_factor: dec!(0.0254),
        }
    }

    /// 英尺
    pub fn foot() -> Self {
        Self {
            code: "FT".to_string(),
            iso_code: Some("FOT".to_string()),
            description: "英尺".to_string(),
            dimension: Dimension::Length,
            base_factor: dec!(0.3048),
        }
    }

    // ------------------------------------------------------------------------
    // 体积单位（基准单位: L）
    // ------------------------------------------------------------------------

    /// 升（基准体积单位）
    pub fn liter() -> Self {
        Self {
            code: "L".to_string(),
            iso_code: Some("LTR".to_string()),
            description: "升".to_string(),
            dimension: Dimension::Volume,
            base_factor: dec!(1),
        }
    }

    /// 毫升
    pub fn milliliter() -> Self {
        Self {
            code: "ML".to_string(),
            iso_code: Some("MLT".to_string()),
            description: "毫升".to_string(),
            dimension: Dimension::Volume,
            base_factor: dec!(0.001),
        }
    }

    /// 立方米
    pub fn cubic_meter() -> Self {
        Self {
            code: "M3".to_string(),
            iso_code: Some("MTQ".to_string()),
            description: "立方米".to_string(),
            dimension: Dimension::Volume,
            base_factor: dec!(1000),
        }
    }

    /// 加仑（美制）
    pub fn gallon_us() -> Self {
        Self {
            code: "GAL".to_string(),
            iso_code: Some("GLL".to_string()),
            description: "加仑(美)".to_string(),
            dimension: Dimension::Volume,
            base_factor: dec!(3.785411784),
        }
    }

    // ------------------------------------------------------------------------
    // 面积单位（基准单位: M2）
    // ------------------------------------------------------------------------

    /// 平方米（基准面积单位）
    pub fn square_meter() -> Self {
        Self {
            code: "M2".to_string(),
            iso_code: Some("MTK".to_string()),
            description: "平方米".to_string(),
            dimension: Dimension::Area,
            base_factor: dec!(1),
        }
    }

    /// 平方厘米
    pub fn square_centimeter() -> Self {
        Self {
            code: "CM2".to_string(),
            iso_code: Some("CMK".to_string()),
            description: "平方厘米".to_string(),
            dimension: Dimension::Area,
            base_factor: dec!(0.0001),
        }
    }

    /// 平方千米
    pub fn square_kilometer() -> Self {
        Self {
            code: "KM2".to_string(),
            iso_code: Some("KMK".to_string()),
            description: "平方千米".to_string(),
            dimension: Dimension::Area,
            base_factor: dec!(1000000),
        }
    }

    // ------------------------------------------------------------------------
    // 时间单位（基准单位: H）
    // ------------------------------------------------------------------------

    /// 小时（基准时间单位）
    pub fn hour() -> Self {
        Self {
            code: "H".to_string(),
            iso_code: Some("HUR".to_string()),
            description: "小时".to_string(),
            dimension: Dimension::Time,
            base_factor: dec!(1),
        }
    }

    /// 分钟
    pub fn minute() -> Self {
        Self {
            code: "MIN".to_string(),
            iso_code: Some("MIN".to_string()),
            description: "分钟".to_string(),
            dimension: Dimension::Time,
            base_factor: dec!(0.016666666666666666),
        }
    }

    /// 秒
    pub fn second() -> Self {
        Self {
            code: "S".to_string(),
            iso_code: Some("SEC".to_string()),
            description: "秒".to_string(),
            dimension: Dimension::Time,
            base_factor: dec!(0.000277777777777778),
        }
    }

    /// 天
    pub fn day() -> Self {
        Self {
            code: "DAY".to_string(),
            iso_code: Some("DAY".to_string()),
            description: "天".to_string(),
            dimension: Dimension::Time,
            base_factor: dec!(24),
        }
    }

    /// 周
    pub fn week() -> Self {
        Self {
            code: "WK".to_string(),
            iso_code: Some("WEE".to_string()),
            description: "周".to_string(),
            dimension: Dimension::Time,
            base_factor: dec!(168),
        }
    }

    /// 月（按 30 天计算）
    pub fn month() -> Self {
        Self {
            code: "MON".to_string(),
            iso_code: Some("MON".to_string()),
            description: "月".to_string(),
            dimension: Dimension::Time,
            base_factor: dec!(720),
        }
    }

    /// 年（按 365 天计算）
    pub fn year() -> Self {
        Self {
            code: "YR".to_string(),
            iso_code: Some("ANN".to_string()),
            description: "年".to_string(),
            dimension: Dimension::Time,
            base_factor: dec!(8760),
        }
    }

    // ------------------------------------------------------------------------
    // 数量/计件单位（基准单位: PC）
    // ------------------------------------------------------------------------

    /// 件/个（基准数量单位）
    pub fn piece() -> Self {
        Self {
            code: "PC".to_string(),
            iso_code: Some("PCE".to_string()),
            description: "件".to_string(),
            dimension: Dimension::Quantity,
            base_factor: dec!(1),
        }
    }

    /// 个（EA = Each）
    pub fn each() -> Self {
        Self {
            code: "EA".to_string(),
            iso_code: Some("EA".to_string()),
            description: "个".to_string(),
            dimension: Dimension::Quantity,
            base_factor: dec!(1),
        }
    }

    /// 套
    pub fn set() -> Self {
        Self {
            code: "SET".to_string(),
            iso_code: Some("SET".to_string()),
            description: "套".to_string(),
            dimension: Dimension::Quantity,
            base_factor: dec!(1),
        }
    }

    /// 箱
    pub fn carton() -> Self {
        Self {
            code: "CT".to_string(),
            iso_code: Some("CT".to_string()),
            description: "箱".to_string(),
            dimension: Dimension::Quantity,
            base_factor: dec!(1),
        }
    }

    /// 打（12 个）
    pub fn dozen() -> Self {
        Self {
            code: "DZ".to_string(),
            iso_code: Some("DZN".to_string()),
            description: "打".to_string(),
            dimension: Dimension::Quantity,
            base_factor: dec!(12),
        }
    }

    /// 百
    pub fn hundred() -> Self {
        Self {
            code: "C".to_string(),
            iso_code: Some("CEN".to_string()),
            description: "百".to_string(),
            dimension: Dimension::Quantity,
            base_factor: dec!(100),
        }
    }

    /// 千
    pub fn thousand() -> Self {
        Self {
            code: "K".to_string(),
            iso_code: Some("MIL".to_string()),
            description: "千".to_string(),
            dimension: Dimension::Quantity,
            base_factor: dec!(1000),
        }
    }
}

impl fmt::Display for UnitOfMeasure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_unit() {
        let unit = UnitOfMeasure::new("KG", "千克", Dimension::Mass, dec!(1)).unwrap();
        assert_eq!(unit.code(), "KG");
        assert_eq!(unit.description(), "千克");
        assert_eq!(unit.dimension(), Dimension::Mass);
        assert_eq!(unit.base_factor(), dec!(1));
    }

    #[test]
    fn test_create_unit_with_iso_code() {
        let unit = UnitOfMeasure::new("KG", "千克", Dimension::Mass, dec!(1))
            .unwrap()
            .with_iso_code("KGM");
        assert_eq!(unit.iso_code(), Some("KGM"));
        assert_eq!(unit.to_iso_code(), "KGM");
    }

    #[test]
    fn test_invalid_code_empty() {
        let result = UnitOfMeasure::new("", "空", Dimension::Mass, dec!(1));
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_code_too_long() {
        let result = UnitOfMeasure::new("TOOLONG1", "太长", Dimension::Mass, dec!(1));
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_base_factor_zero() {
        let result = UnitOfMeasure::new("KG", "千克", Dimension::Mass, dec!(0));
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_base_factor_negative() {
        let result = UnitOfMeasure::new("KG", "千克", Dimension::Mass, dec!(-1));
        assert!(result.is_err());
    }

    #[test]
    fn test_predefined_kilogram() {
        let kg = UnitOfMeasure::kilogram();
        assert_eq!(kg.code(), "KG");
        assert_eq!(kg.iso_code(), Some("KGM"));
        assert_eq!(kg.dimension(), Dimension::Mass);
        assert_eq!(kg.base_factor(), dec!(1));
    }

    #[test]
    fn test_predefined_gram() {
        let g = UnitOfMeasure::gram();
        assert_eq!(g.code(), "G");
        assert_eq!(g.dimension(), Dimension::Mass);
        assert_eq!(g.base_factor(), dec!(0.001));
    }

    #[test]
    fn test_conversion_kg_to_g() {
        let kg = UnitOfMeasure::kilogram();
        let g = UnitOfMeasure::gram();

        let factor = kg.conversion_factor_to(&g).unwrap();
        assert_eq!(factor, dec!(1000));
    }

    #[test]
    fn test_conversion_g_to_kg() {
        let kg = UnitOfMeasure::kilogram();
        let g = UnitOfMeasure::gram();

        let factor = g.conversion_factor_to(&kg).unwrap();
        assert_eq!(factor, dec!(0.001));
    }

    #[test]
    fn test_conversion_ton_to_kg() {
        let t = UnitOfMeasure::ton();
        let kg = UnitOfMeasure::kilogram();

        let factor = t.conversion_factor_to(&kg).unwrap();
        assert_eq!(factor, dec!(1000));
    }

    #[test]
    fn test_conversion_m_to_cm() {
        let m = UnitOfMeasure::meter();
        let cm = UnitOfMeasure::centimeter();

        let factor = m.conversion_factor_to(&cm).unwrap();
        assert_eq!(factor, dec!(100));
    }

    #[test]
    fn test_conversion_incompatible_dimension() {
        let kg = UnitOfMeasure::kilogram();
        let m = UnitOfMeasure::meter();

        let result = kg.conversion_factor_to(&m);
        assert!(result.is_err());
    }

    #[test]
    fn test_can_convert_to_same_dimension() {
        let kg = UnitOfMeasure::kilogram();
        let g = UnitOfMeasure::gram();
        assert!(kg.can_convert_to(&g));
    }

    #[test]
    fn test_cannot_convert_to_different_dimension() {
        let kg = UnitOfMeasure::kilogram();
        let m = UnitOfMeasure::meter();
        assert!(!kg.can_convert_to(&m));
    }

    #[test]
    fn test_dozen_to_piece() {
        let dz = UnitOfMeasure::dozen();
        let pc = UnitOfMeasure::piece();

        let factor = dz.conversion_factor_to(&pc).unwrap();
        assert_eq!(factor, dec!(12));
    }

    #[test]
    fn test_day_to_hour() {
        let day = UnitOfMeasure::day();
        let hour = UnitOfMeasure::hour();

        let factor = day.conversion_factor_to(&hour).unwrap();
        assert_eq!(factor, dec!(24));
    }

    #[test]
    fn test_liter_to_milliliter() {
        let l = UnitOfMeasure::liter();
        let ml = UnitOfMeasure::milliliter();

        let factor = l.conversion_factor_to(&ml).unwrap();
        assert_eq!(factor, dec!(1000));
    }

    #[test]
    fn test_cubic_meter_to_liter() {
        let m3 = UnitOfMeasure::cubic_meter();
        let l = UnitOfMeasure::liter();

        let factor = m3.conversion_factor_to(&l).unwrap();
        assert_eq!(factor, dec!(1000));
    }

    #[test]
    fn test_serialization() {
        let kg = UnitOfMeasure::kilogram();
        let json = serde_json::to_string(&kg).unwrap();
        assert!(json.contains("\"code\":\"KG\""));
        assert!(json.contains("\"iso_code\":\"KGM\""));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"code":"KG","iso_code":"KGM","description":"千克","dimension":"MASS","base_factor":"1"}"#;
        let kg: UnitOfMeasure = serde_json::from_str(json).unwrap();
        assert_eq!(kg.code(), "KG");
        assert_eq!(kg.dimension(), Dimension::Mass);
    }

    #[test]
    fn test_display() {
        let kg = UnitOfMeasure::kilogram();
        assert_eq!(format!("{}", kg), "KG");
    }

    #[test]
    fn test_dimension_descriptions() {
        assert_eq!(Dimension::Mass.description_zh(), "质量");
        assert_eq!(Dimension::Mass.description_en(), "Mass");
        assert_eq!(Dimension::Length.description_zh(), "长度");
        assert_eq!(Dimension::Volume.description_zh(), "体积");
    }

    #[test]
    fn test_equality() {
        let kg1 = UnitOfMeasure::kilogram();
        let kg2 = UnitOfMeasure::kilogram();
        assert_eq!(kg1, kg2);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(UnitOfMeasure::kilogram());
        set.insert(UnitOfMeasure::gram());
        set.insert(UnitOfMeasure::kilogram()); // 重复

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_clone() {
        let kg = UnitOfMeasure::kilogram();
        let kg_clone = kg.clone();
        assert_eq!(kg, kg_clone);
    }
}
