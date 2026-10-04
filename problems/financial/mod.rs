// Automatically generated category module

#[path = "depreciation/solution.rs"]
pub mod depreciation;

#[path = "equated_monthly_installments/solution.rs"]
pub mod equated_monthly_installments;

#[path = "exponential_moving_average/solution.rs"]
pub mod exponential_moving_average;

#[path = "finance_ratios/solution.rs"]
pub mod finance_ratios;

#[path = "interest/solution.rs"]
pub mod interest;

#[path = "npv/solution.rs"]
pub mod npv;

#[path = "npv_sensitivity/solution.rs"]
pub mod npv_sensitivity;

#[path = "payback/solution.rs"]
pub mod payback;

#[path = "present_value/solution.rs"]
pub mod present_value;

#[path = "treynor_ratio/solution.rs"]
pub mod treynor_ratio;


pub use self::depreciation::{
    diminishing_balance_depreciation, double_declining_balance_depreciation,
    straight_line_depreciation, sum_of_years_digits_depreciation, units_of_production_depreciation,
};
pub use self::equated_monthly_installments::equated_monthly_installments;
pub use self::exponential_moving_average::exponential_moving_average;
pub use self::finance_ratios::{
    debt_to_equity, earnings_per_sale, gross_profit_margin, return_on_investment,
};
pub use self::interest::{apr_interest, compound_interest, simple_interest};
pub use self::npv::npv;
pub use self::npv_sensitivity::npv_sensitivity;
pub use self::payback::payback;
pub use self::present_value::present_value;
pub use self::treynor_ratio::treynor_ratio;
