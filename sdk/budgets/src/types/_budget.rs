// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the output of the <code>CreateBudget</code> operation. The content consists of the detailed metadata and data file information, and the current status of the <code>budget</code> object.</p>
/// <p>This is the Amazon Resource Name (ARN) pattern for a budget: </p>
/// <p> <code>arn:aws:budgets::AccountId:budget/budgetName</code> </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Budget {
    /// <p>The name of a budget. The name must be unique within an account. The <code>:</code> and <code>\</code> characters aren't allowed in <code>BudgetName</code>.</p>
    #[doc(hidden)]
    pub budget_name: ::std::option::Option<::std::string::String>,
    /// <p>The total amount of cost, usage, RI utilization, RI coverage, Savings Plans utilization, or Savings Plans coverage that you want to track with your budget.</p>
    /// <p> <code>BudgetLimit</code> is required for cost or usage budgets, but optional for RI or Savings Plans utilization or coverage budgets. RI and Savings Plans utilization or coverage budgets default to <code>100</code>. This is the only valid value for RI or Savings Plans utilization or coverage budgets. You can't use <code>BudgetLimit</code> with <code>PlannedBudgetLimits</code> for <code>CreateBudget</code> and <code>UpdateBudget</code> actions. </p>
    #[doc(hidden)]
    pub budget_limit: ::std::option::Option<crate::types::Spend>,
    /// <p>A map containing multiple <code>BudgetLimit</code>, including current or future limits.</p>
    /// <p> <code>PlannedBudgetLimits</code> is available for cost or usage budget and supports both monthly and quarterly <code>TimeUnit</code>. </p>
    /// <p>For monthly budgets, provide 12 months of <code>PlannedBudgetLimits</code> values. This must start from the current month and include the next 11 months. The <code>key</code> is the start of the month, <code>UTC</code> in epoch seconds. </p>
    /// <p>For quarterly budgets, provide four quarters of <code>PlannedBudgetLimits</code> value entries in standard calendar quarter increments. This must start from the current quarter and include the next three quarters. The <code>key</code> is the start of the quarter, <code>UTC</code> in epoch seconds. </p>
    /// <p>If the planned budget expires before 12 months for monthly or four quarters for quarterly, provide the <code>PlannedBudgetLimits</code> values only for the remaining periods.</p>
    /// <p>If the budget begins at a date in the future, provide <code>PlannedBudgetLimits</code> values from the start date of the budget. </p>
    /// <p>After all of the <code>BudgetLimit</code> values in <code>PlannedBudgetLimits</code> are used, the budget continues to use the last limit as the <code>BudgetLimit</code>. At that point, the planned budget provides the same experience as a fixed budget. </p>
    /// <p> <code>DescribeBudget</code> and <code>DescribeBudgets</code> response along with <code>PlannedBudgetLimits</code> also contain <code>BudgetLimit</code> representing the current month or quarter limit present in <code>PlannedBudgetLimits</code>. This only applies to budgets that are created with <code>PlannedBudgetLimits</code>. Budgets that are created without <code>PlannedBudgetLimits</code> only contain <code>BudgetLimit</code>. They don't contain <code>PlannedBudgetLimits</code>.</p>
    #[doc(hidden)]
    pub planned_budget_limits: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::Spend>,
    >,
    /// <p>The cost filters, such as <code>Region</code>, <code>Service</code>, <code>member account</code>, <code>Tag</code>, or <code>Cost Category</code>, that are applied to a budget.</p>
    /// <p>Amazon Web Services Budgets supports the following services as a <code>Service</code> filter for RI budgets:</p>
    /// <ul>
    /// <li> <p>Amazon EC2</p> </li>
    /// <li> <p>Amazon Redshift</p> </li>
    /// <li> <p>Amazon Relational Database Service</p> </li>
    /// <li> <p>Amazon ElastiCache</p> </li>
    /// <li> <p>Amazon OpenSearch Service</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub cost_filters: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::string::String>>,
    >,
    /// <p>The types of costs that are included in this <code>COST</code> budget.</p>
    /// <p> <code>USAGE</code>, <code>RI_UTILIZATION</code>, <code>RI_COVERAGE</code>, <code>SAVINGS_PLANS_UTILIZATION</code>, and <code>SAVINGS_PLANS_COVERAGE</code> budgets do not have <code>CostTypes</code>.</p>
    #[doc(hidden)]
    pub cost_types: ::std::option::Option<crate::types::CostTypes>,
    /// <p>The length of time until a budget resets the actual and forecasted spend.</p>
    #[doc(hidden)]
    pub time_unit: ::std::option::Option<crate::types::TimeUnit>,
    /// <p>The period of time that's covered by a budget. You setthe start date and end date. The start date must come before the end date. The end date must come before <code>06/15/87 00:00 UTC</code>. </p>
    /// <p>If you create your budget and don't specify a start date, Amazon Web Services defaults to the start of your chosen time period (DAILY, MONTHLY, QUARTERLY, or ANNUALLY). For example, if you created your budget on January 24, 2018, chose <code>DAILY</code>, and didn't set a start date, Amazon Web Services set your start date to <code>01/24/18 00:00 UTC</code>. If you chose <code>MONTHLY</code>, Amazon Web Services set your start date to <code>01/01/18 00:00 UTC</code>. If you didn't specify an end date, Amazon Web Services set your end date to <code>06/15/87 00:00 UTC</code>. The defaults are the same for the Billing and Cost Management console and the API. </p>
    /// <p>You can change either date with the <code>UpdateBudget</code> operation.</p>
    /// <p>After the end date, Amazon Web Services deletes the budget and all the associated notifications and subscribers.</p>
    #[doc(hidden)]
    pub time_period: ::std::option::Option<crate::types::TimePeriod>,
    /// <p>The actual and forecasted cost or usage that the budget tracks.</p>
    #[doc(hidden)]
    pub calculated_spend: ::std::option::Option<crate::types::CalculatedSpend>,
    /// <p>Specifies whether this budget tracks costs, usage, RI utilization, RI coverage, Savings Plans utilization, or Savings Plans coverage.</p>
    #[doc(hidden)]
    pub budget_type: ::std::option::Option<crate::types::BudgetType>,
    /// <p>The last time that you updated this budget.</p>
    #[doc(hidden)]
    pub last_updated_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The parameters that determine the budget amount for an auto-adjusting budget.</p>
    #[doc(hidden)]
    pub auto_adjust_data: ::std::option::Option<crate::types::AutoAdjustData>,
}
impl Budget {
    /// <p>The name of a budget. The name must be unique within an account. The <code>:</code> and <code>\</code> characters aren't allowed in <code>BudgetName</code>.</p>
    pub fn budget_name(&self) -> ::std::option::Option<&str> {
        self.budget_name.as_deref()
    }
    /// <p>The total amount of cost, usage, RI utilization, RI coverage, Savings Plans utilization, or Savings Plans coverage that you want to track with your budget.</p>
    /// <p> <code>BudgetLimit</code> is required for cost or usage budgets, but optional for RI or Savings Plans utilization or coverage budgets. RI and Savings Plans utilization or coverage budgets default to <code>100</code>. This is the only valid value for RI or Savings Plans utilization or coverage budgets. You can't use <code>BudgetLimit</code> with <code>PlannedBudgetLimits</code> for <code>CreateBudget</code> and <code>UpdateBudget</code> actions. </p>
    pub fn budget_limit(&self) -> ::std::option::Option<&crate::types::Spend> {
        self.budget_limit.as_ref()
    }
    /// <p>A map containing multiple <code>BudgetLimit</code>, including current or future limits.</p>
    /// <p> <code>PlannedBudgetLimits</code> is available for cost or usage budget and supports both monthly and quarterly <code>TimeUnit</code>. </p>
    /// <p>For monthly budgets, provide 12 months of <code>PlannedBudgetLimits</code> values. This must start from the current month and include the next 11 months. The <code>key</code> is the start of the month, <code>UTC</code> in epoch seconds. </p>
    /// <p>For quarterly budgets, provide four quarters of <code>PlannedBudgetLimits</code> value entries in standard calendar quarter increments. This must start from the current quarter and include the next three quarters. The <code>key</code> is the start of the quarter, <code>UTC</code> in epoch seconds. </p>
    /// <p>If the planned budget expires before 12 months for monthly or four quarters for quarterly, provide the <code>PlannedBudgetLimits</code> values only for the remaining periods.</p>
    /// <p>If the budget begins at a date in the future, provide <code>PlannedBudgetLimits</code> values from the start date of the budget. </p>
    /// <p>After all of the <code>BudgetLimit</code> values in <code>PlannedBudgetLimits</code> are used, the budget continues to use the last limit as the <code>BudgetLimit</code>. At that point, the planned budget provides the same experience as a fixed budget. </p>
    /// <p> <code>DescribeBudget</code> and <code>DescribeBudgets</code> response along with <code>PlannedBudgetLimits</code> also contain <code>BudgetLimit</code> representing the current month or quarter limit present in <code>PlannedBudgetLimits</code>. This only applies to budgets that are created with <code>PlannedBudgetLimits</code>. Budgets that are created without <code>PlannedBudgetLimits</code> only contain <code>BudgetLimit</code>. They don't contain <code>PlannedBudgetLimits</code>.</p>
    pub fn planned_budget_limits(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, crate::types::Spend>,
    > {
        self.planned_budget_limits.as_ref()
    }
    /// <p>The cost filters, such as <code>Region</code>, <code>Service</code>, <code>member account</code>, <code>Tag</code>, or <code>Cost Category</code>, that are applied to a budget.</p>
    /// <p>Amazon Web Services Budgets supports the following services as a <code>Service</code> filter for RI budgets:</p>
    /// <ul>
    /// <li> <p>Amazon EC2</p> </li>
    /// <li> <p>Amazon Redshift</p> </li>
    /// <li> <p>Amazon Relational Database Service</p> </li>
    /// <li> <p>Amazon ElastiCache</p> </li>
    /// <li> <p>Amazon OpenSearch Service</p> </li>
    /// </ul>
    pub fn cost_filters(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::string::String>>,
    > {
        self.cost_filters.as_ref()
    }
    /// <p>The types of costs that are included in this <code>COST</code> budget.</p>
    /// <p> <code>USAGE</code>, <code>RI_UTILIZATION</code>, <code>RI_COVERAGE</code>, <code>SAVINGS_PLANS_UTILIZATION</code>, and <code>SAVINGS_PLANS_COVERAGE</code> budgets do not have <code>CostTypes</code>.</p>
    pub fn cost_types(&self) -> ::std::option::Option<&crate::types::CostTypes> {
        self.cost_types.as_ref()
    }
    /// <p>The length of time until a budget resets the actual and forecasted spend.</p>
    pub fn time_unit(&self) -> ::std::option::Option<&crate::types::TimeUnit> {
        self.time_unit.as_ref()
    }
    /// <p>The period of time that's covered by a budget. You setthe start date and end date. The start date must come before the end date. The end date must come before <code>06/15/87 00:00 UTC</code>. </p>
    /// <p>If you create your budget and don't specify a start date, Amazon Web Services defaults to the start of your chosen time period (DAILY, MONTHLY, QUARTERLY, or ANNUALLY). For example, if you created your budget on January 24, 2018, chose <code>DAILY</code>, and didn't set a start date, Amazon Web Services set your start date to <code>01/24/18 00:00 UTC</code>. If you chose <code>MONTHLY</code>, Amazon Web Services set your start date to <code>01/01/18 00:00 UTC</code>. If you didn't specify an end date, Amazon Web Services set your end date to <code>06/15/87 00:00 UTC</code>. The defaults are the same for the Billing and Cost Management console and the API. </p>
    /// <p>You can change either date with the <code>UpdateBudget</code> operation.</p>
    /// <p>After the end date, Amazon Web Services deletes the budget and all the associated notifications and subscribers.</p>
    pub fn time_period(&self) -> ::std::option::Option<&crate::types::TimePeriod> {
        self.time_period.as_ref()
    }
    /// <p>The actual and forecasted cost or usage that the budget tracks.</p>
    pub fn calculated_spend(&self) -> ::std::option::Option<&crate::types::CalculatedSpend> {
        self.calculated_spend.as_ref()
    }
    /// <p>Specifies whether this budget tracks costs, usage, RI utilization, RI coverage, Savings Plans utilization, or Savings Plans coverage.</p>
    pub fn budget_type(&self) -> ::std::option::Option<&crate::types::BudgetType> {
        self.budget_type.as_ref()
    }
    /// <p>The last time that you updated this budget.</p>
    pub fn last_updated_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_updated_time.as_ref()
    }
    /// <p>The parameters that determine the budget amount for an auto-adjusting budget.</p>
    pub fn auto_adjust_data(&self) -> ::std::option::Option<&crate::types::AutoAdjustData> {
        self.auto_adjust_data.as_ref()
    }
}
impl Budget {
    /// Creates a new builder-style object to manufacture [`Budget`](crate::types::Budget).
    pub fn builder() -> crate::types::builders::BudgetBuilder {
        crate::types::builders::BudgetBuilder::default()
    }
}

/// A builder for [`Budget`](crate::types::Budget).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BudgetBuilder {
    pub(crate) budget_name: ::std::option::Option<::std::string::String>,
    pub(crate) budget_limit: ::std::option::Option<crate::types::Spend>,
    pub(crate) planned_budget_limits: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::Spend>,
    >,
    pub(crate) cost_filters: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::string::String>>,
    >,
    pub(crate) cost_types: ::std::option::Option<crate::types::CostTypes>,
    pub(crate) time_unit: ::std::option::Option<crate::types::TimeUnit>,
    pub(crate) time_period: ::std::option::Option<crate::types::TimePeriod>,
    pub(crate) calculated_spend: ::std::option::Option<crate::types::CalculatedSpend>,
    pub(crate) budget_type: ::std::option::Option<crate::types::BudgetType>,
    pub(crate) last_updated_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) auto_adjust_data: ::std::option::Option<crate::types::AutoAdjustData>,
}
impl BudgetBuilder {
    /// <p>The name of a budget. The name must be unique within an account. The <code>:</code> and <code>\</code> characters aren't allowed in <code>BudgetName</code>.</p>
    pub fn budget_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.budget_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of a budget. The name must be unique within an account. The <code>:</code> and <code>\</code> characters aren't allowed in <code>BudgetName</code>.</p>
    pub fn set_budget_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.budget_name = input;
        self
    }
    /// <p>The total amount of cost, usage, RI utilization, RI coverage, Savings Plans utilization, or Savings Plans coverage that you want to track with your budget.</p>
    /// <p> <code>BudgetLimit</code> is required for cost or usage budgets, but optional for RI or Savings Plans utilization or coverage budgets. RI and Savings Plans utilization or coverage budgets default to <code>100</code>. This is the only valid value for RI or Savings Plans utilization or coverage budgets. You can't use <code>BudgetLimit</code> with <code>PlannedBudgetLimits</code> for <code>CreateBudget</code> and <code>UpdateBudget</code> actions. </p>
    pub fn budget_limit(mut self, input: crate::types::Spend) -> Self {
        self.budget_limit = ::std::option::Option::Some(input);
        self
    }
    /// <p>The total amount of cost, usage, RI utilization, RI coverage, Savings Plans utilization, or Savings Plans coverage that you want to track with your budget.</p>
    /// <p> <code>BudgetLimit</code> is required for cost or usage budgets, but optional for RI or Savings Plans utilization or coverage budgets. RI and Savings Plans utilization or coverage budgets default to <code>100</code>. This is the only valid value for RI or Savings Plans utilization or coverage budgets. You can't use <code>BudgetLimit</code> with <code>PlannedBudgetLimits</code> for <code>CreateBudget</code> and <code>UpdateBudget</code> actions. </p>
    pub fn set_budget_limit(mut self, input: ::std::option::Option<crate::types::Spend>) -> Self {
        self.budget_limit = input;
        self
    }
    /// Adds a key-value pair to `planned_budget_limits`.
    ///
    /// To override the contents of this collection use [`set_planned_budget_limits`](Self::set_planned_budget_limits).
    ///
    /// <p>A map containing multiple <code>BudgetLimit</code>, including current or future limits.</p>
    /// <p> <code>PlannedBudgetLimits</code> is available for cost or usage budget and supports both monthly and quarterly <code>TimeUnit</code>. </p>
    /// <p>For monthly budgets, provide 12 months of <code>PlannedBudgetLimits</code> values. This must start from the current month and include the next 11 months. The <code>key</code> is the start of the month, <code>UTC</code> in epoch seconds. </p>
    /// <p>For quarterly budgets, provide four quarters of <code>PlannedBudgetLimits</code> value entries in standard calendar quarter increments. This must start from the current quarter and include the next three quarters. The <code>key</code> is the start of the quarter, <code>UTC</code> in epoch seconds. </p>
    /// <p>If the planned budget expires before 12 months for monthly or four quarters for quarterly, provide the <code>PlannedBudgetLimits</code> values only for the remaining periods.</p>
    /// <p>If the budget begins at a date in the future, provide <code>PlannedBudgetLimits</code> values from the start date of the budget. </p>
    /// <p>After all of the <code>BudgetLimit</code> values in <code>PlannedBudgetLimits</code> are used, the budget continues to use the last limit as the <code>BudgetLimit</code>. At that point, the planned budget provides the same experience as a fixed budget. </p>
    /// <p> <code>DescribeBudget</code> and <code>DescribeBudgets</code> response along with <code>PlannedBudgetLimits</code> also contain <code>BudgetLimit</code> representing the current month or quarter limit present in <code>PlannedBudgetLimits</code>. This only applies to budgets that are created with <code>PlannedBudgetLimits</code>. Budgets that are created without <code>PlannedBudgetLimits</code> only contain <code>BudgetLimit</code>. They don't contain <code>PlannedBudgetLimits</code>.</p>
    pub fn planned_budget_limits(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::Spend,
    ) -> Self {
        let mut hash_map = self.planned_budget_limits.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.planned_budget_limits = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A map containing multiple <code>BudgetLimit</code>, including current or future limits.</p>
    /// <p> <code>PlannedBudgetLimits</code> is available for cost or usage budget and supports both monthly and quarterly <code>TimeUnit</code>. </p>
    /// <p>For monthly budgets, provide 12 months of <code>PlannedBudgetLimits</code> values. This must start from the current month and include the next 11 months. The <code>key</code> is the start of the month, <code>UTC</code> in epoch seconds. </p>
    /// <p>For quarterly budgets, provide four quarters of <code>PlannedBudgetLimits</code> value entries in standard calendar quarter increments. This must start from the current quarter and include the next three quarters. The <code>key</code> is the start of the quarter, <code>UTC</code> in epoch seconds. </p>
    /// <p>If the planned budget expires before 12 months for monthly or four quarters for quarterly, provide the <code>PlannedBudgetLimits</code> values only for the remaining periods.</p>
    /// <p>If the budget begins at a date in the future, provide <code>PlannedBudgetLimits</code> values from the start date of the budget. </p>
    /// <p>After all of the <code>BudgetLimit</code> values in <code>PlannedBudgetLimits</code> are used, the budget continues to use the last limit as the <code>BudgetLimit</code>. At that point, the planned budget provides the same experience as a fixed budget. </p>
    /// <p> <code>DescribeBudget</code> and <code>DescribeBudgets</code> response along with <code>PlannedBudgetLimits</code> also contain <code>BudgetLimit</code> representing the current month or quarter limit present in <code>PlannedBudgetLimits</code>. This only applies to budgets that are created with <code>PlannedBudgetLimits</code>. Budgets that are created without <code>PlannedBudgetLimits</code> only contain <code>BudgetLimit</code>. They don't contain <code>PlannedBudgetLimits</code>.</p>
    pub fn set_planned_budget_limits(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::Spend>,
        >,
    ) -> Self {
        self.planned_budget_limits = input;
        self
    }
    /// Adds a key-value pair to `cost_filters`.
    ///
    /// To override the contents of this collection use [`set_cost_filters`](Self::set_cost_filters).
    ///
    /// <p>The cost filters, such as <code>Region</code>, <code>Service</code>, <code>member account</code>, <code>Tag</code>, or <code>Cost Category</code>, that are applied to a budget.</p>
    /// <p>Amazon Web Services Budgets supports the following services as a <code>Service</code> filter for RI budgets:</p>
    /// <ul>
    /// <li> <p>Amazon EC2</p> </li>
    /// <li> <p>Amazon Redshift</p> </li>
    /// <li> <p>Amazon Relational Database Service</p> </li>
    /// <li> <p>Amazon ElastiCache</p> </li>
    /// <li> <p>Amazon OpenSearch Service</p> </li>
    /// </ul>
    pub fn cost_filters(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: ::std::vec::Vec<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.cost_filters.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.cost_filters = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The cost filters, such as <code>Region</code>, <code>Service</code>, <code>member account</code>, <code>Tag</code>, or <code>Cost Category</code>, that are applied to a budget.</p>
    /// <p>Amazon Web Services Budgets supports the following services as a <code>Service</code> filter for RI budgets:</p>
    /// <ul>
    /// <li> <p>Amazon EC2</p> </li>
    /// <li> <p>Amazon Redshift</p> </li>
    /// <li> <p>Amazon Relational Database Service</p> </li>
    /// <li> <p>Amazon ElastiCache</p> </li>
    /// <li> <p>Amazon OpenSearch Service</p> </li>
    /// </ul>
    pub fn set_cost_filters(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<
                ::std::string::String,
                ::std::vec::Vec<::std::string::String>,
            >,
        >,
    ) -> Self {
        self.cost_filters = input;
        self
    }
    /// <p>The types of costs that are included in this <code>COST</code> budget.</p>
    /// <p> <code>USAGE</code>, <code>RI_UTILIZATION</code>, <code>RI_COVERAGE</code>, <code>SAVINGS_PLANS_UTILIZATION</code>, and <code>SAVINGS_PLANS_COVERAGE</code> budgets do not have <code>CostTypes</code>.</p>
    pub fn cost_types(mut self, input: crate::types::CostTypes) -> Self {
        self.cost_types = ::std::option::Option::Some(input);
        self
    }
    /// <p>The types of costs that are included in this <code>COST</code> budget.</p>
    /// <p> <code>USAGE</code>, <code>RI_UTILIZATION</code>, <code>RI_COVERAGE</code>, <code>SAVINGS_PLANS_UTILIZATION</code>, and <code>SAVINGS_PLANS_COVERAGE</code> budgets do not have <code>CostTypes</code>.</p>
    pub fn set_cost_types(mut self, input: ::std::option::Option<crate::types::CostTypes>) -> Self {
        self.cost_types = input;
        self
    }
    /// <p>The length of time until a budget resets the actual and forecasted spend.</p>
    pub fn time_unit(mut self, input: crate::types::TimeUnit) -> Self {
        self.time_unit = ::std::option::Option::Some(input);
        self
    }
    /// <p>The length of time until a budget resets the actual and forecasted spend.</p>
    pub fn set_time_unit(mut self, input: ::std::option::Option<crate::types::TimeUnit>) -> Self {
        self.time_unit = input;
        self
    }
    /// <p>The period of time that's covered by a budget. You setthe start date and end date. The start date must come before the end date. The end date must come before <code>06/15/87 00:00 UTC</code>. </p>
    /// <p>If you create your budget and don't specify a start date, Amazon Web Services defaults to the start of your chosen time period (DAILY, MONTHLY, QUARTERLY, or ANNUALLY). For example, if you created your budget on January 24, 2018, chose <code>DAILY</code>, and didn't set a start date, Amazon Web Services set your start date to <code>01/24/18 00:00 UTC</code>. If you chose <code>MONTHLY</code>, Amazon Web Services set your start date to <code>01/01/18 00:00 UTC</code>. If you didn't specify an end date, Amazon Web Services set your end date to <code>06/15/87 00:00 UTC</code>. The defaults are the same for the Billing and Cost Management console and the API. </p>
    /// <p>You can change either date with the <code>UpdateBudget</code> operation.</p>
    /// <p>After the end date, Amazon Web Services deletes the budget and all the associated notifications and subscribers.</p>
    pub fn time_period(mut self, input: crate::types::TimePeriod) -> Self {
        self.time_period = ::std::option::Option::Some(input);
        self
    }
    /// <p>The period of time that's covered by a budget. You setthe start date and end date. The start date must come before the end date. The end date must come before <code>06/15/87 00:00 UTC</code>. </p>
    /// <p>If you create your budget and don't specify a start date, Amazon Web Services defaults to the start of your chosen time period (DAILY, MONTHLY, QUARTERLY, or ANNUALLY). For example, if you created your budget on January 24, 2018, chose <code>DAILY</code>, and didn't set a start date, Amazon Web Services set your start date to <code>01/24/18 00:00 UTC</code>. If you chose <code>MONTHLY</code>, Amazon Web Services set your start date to <code>01/01/18 00:00 UTC</code>. If you didn't specify an end date, Amazon Web Services set your end date to <code>06/15/87 00:00 UTC</code>. The defaults are the same for the Billing and Cost Management console and the API. </p>
    /// <p>You can change either date with the <code>UpdateBudget</code> operation.</p>
    /// <p>After the end date, Amazon Web Services deletes the budget and all the associated notifications and subscribers.</p>
    pub fn set_time_period(
        mut self,
        input: ::std::option::Option<crate::types::TimePeriod>,
    ) -> Self {
        self.time_period = input;
        self
    }
    /// <p>The actual and forecasted cost or usage that the budget tracks.</p>
    pub fn calculated_spend(mut self, input: crate::types::CalculatedSpend) -> Self {
        self.calculated_spend = ::std::option::Option::Some(input);
        self
    }
    /// <p>The actual and forecasted cost or usage that the budget tracks.</p>
    pub fn set_calculated_spend(
        mut self,
        input: ::std::option::Option<crate::types::CalculatedSpend>,
    ) -> Self {
        self.calculated_spend = input;
        self
    }
    /// <p>Specifies whether this budget tracks costs, usage, RI utilization, RI coverage, Savings Plans utilization, or Savings Plans coverage.</p>
    pub fn budget_type(mut self, input: crate::types::BudgetType) -> Self {
        self.budget_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether this budget tracks costs, usage, RI utilization, RI coverage, Savings Plans utilization, or Savings Plans coverage.</p>
    pub fn set_budget_type(
        mut self,
        input: ::std::option::Option<crate::types::BudgetType>,
    ) -> Self {
        self.budget_type = input;
        self
    }
    /// <p>The last time that you updated this budget.</p>
    pub fn last_updated_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_updated_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The last time that you updated this budget.</p>
    pub fn set_last_updated_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_updated_time = input;
        self
    }
    /// <p>The parameters that determine the budget amount for an auto-adjusting budget.</p>
    pub fn auto_adjust_data(mut self, input: crate::types::AutoAdjustData) -> Self {
        self.auto_adjust_data = ::std::option::Option::Some(input);
        self
    }
    /// <p>The parameters that determine the budget amount for an auto-adjusting budget.</p>
    pub fn set_auto_adjust_data(
        mut self,
        input: ::std::option::Option<crate::types::AutoAdjustData>,
    ) -> Self {
        self.auto_adjust_data = input;
        self
    }
    /// Consumes the builder and constructs a [`Budget`](crate::types::Budget).
    pub fn build(self) -> crate::types::Budget {
        crate::types::Budget {
            budget_name: self.budget_name,
            budget_limit: self.budget_limit,
            planned_budget_limits: self.planned_budget_limits,
            cost_filters: self.cost_filters,
            cost_types: self.cost_types,
            time_unit: self.time_unit,
            time_period: self.time_period,
            calculated_spend: self.calculated_spend,
            budget_type: self.budget_type,
            last_updated_time: self.last_updated_time,
            auto_adjust_data: self.auto_adjust_data,
        }
    }
}
