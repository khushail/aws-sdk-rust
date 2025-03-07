// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateAlertInput {
    /// <p>The ARN of the alert to update.</p>
    #[doc(hidden)]
    pub alert_arn: ::std::option::Option<::std::string::String>,
    /// <p>A description of the alert.</p>
    #[doc(hidden)]
    pub alert_description: ::std::option::Option<::std::string::String>,
    /// <p>An integer from 0 to 100 specifying the alert sensitivity threshold.</p>
    #[doc(hidden)]
    pub alert_sensitivity_threshold: i32,
    /// <p>Action that will be triggered when there is an alert.</p>
    #[doc(hidden)]
    pub action: ::std::option::Option<crate::types::Action>,
    /// <p>The configuration of the alert filters, containing MetricList and DimensionFilterList.</p>
    #[doc(hidden)]
    pub alert_filters: ::std::option::Option<crate::types::AlertFilters>,
}
impl UpdateAlertInput {
    /// <p>The ARN of the alert to update.</p>
    pub fn alert_arn(&self) -> ::std::option::Option<&str> {
        self.alert_arn.as_deref()
    }
    /// <p>A description of the alert.</p>
    pub fn alert_description(&self) -> ::std::option::Option<&str> {
        self.alert_description.as_deref()
    }
    /// <p>An integer from 0 to 100 specifying the alert sensitivity threshold.</p>
    pub fn alert_sensitivity_threshold(&self) -> i32 {
        self.alert_sensitivity_threshold
    }
    /// <p>Action that will be triggered when there is an alert.</p>
    pub fn action(&self) -> ::std::option::Option<&crate::types::Action> {
        self.action.as_ref()
    }
    /// <p>The configuration of the alert filters, containing MetricList and DimensionFilterList.</p>
    pub fn alert_filters(&self) -> ::std::option::Option<&crate::types::AlertFilters> {
        self.alert_filters.as_ref()
    }
}
impl UpdateAlertInput {
    /// Creates a new builder-style object to manufacture [`UpdateAlertInput`](crate::operation::update_alert::UpdateAlertInput).
    pub fn builder() -> crate::operation::update_alert::builders::UpdateAlertInputBuilder {
        crate::operation::update_alert::builders::UpdateAlertInputBuilder::default()
    }
}

/// A builder for [`UpdateAlertInput`](crate::operation::update_alert::UpdateAlertInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateAlertInputBuilder {
    pub(crate) alert_arn: ::std::option::Option<::std::string::String>,
    pub(crate) alert_description: ::std::option::Option<::std::string::String>,
    pub(crate) alert_sensitivity_threshold: ::std::option::Option<i32>,
    pub(crate) action: ::std::option::Option<crate::types::Action>,
    pub(crate) alert_filters: ::std::option::Option<crate::types::AlertFilters>,
}
impl UpdateAlertInputBuilder {
    /// <p>The ARN of the alert to update.</p>
    pub fn alert_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.alert_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the alert to update.</p>
    pub fn set_alert_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.alert_arn = input;
        self
    }
    /// <p>A description of the alert.</p>
    pub fn alert_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.alert_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the alert.</p>
    pub fn set_alert_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.alert_description = input;
        self
    }
    /// <p>An integer from 0 to 100 specifying the alert sensitivity threshold.</p>
    pub fn alert_sensitivity_threshold(mut self, input: i32) -> Self {
        self.alert_sensitivity_threshold = ::std::option::Option::Some(input);
        self
    }
    /// <p>An integer from 0 to 100 specifying the alert sensitivity threshold.</p>
    pub fn set_alert_sensitivity_threshold(mut self, input: ::std::option::Option<i32>) -> Self {
        self.alert_sensitivity_threshold = input;
        self
    }
    /// <p>Action that will be triggered when there is an alert.</p>
    pub fn action(mut self, input: crate::types::Action) -> Self {
        self.action = ::std::option::Option::Some(input);
        self
    }
    /// <p>Action that will be triggered when there is an alert.</p>
    pub fn set_action(mut self, input: ::std::option::Option<crate::types::Action>) -> Self {
        self.action = input;
        self
    }
    /// <p>The configuration of the alert filters, containing MetricList and DimensionFilterList.</p>
    pub fn alert_filters(mut self, input: crate::types::AlertFilters) -> Self {
        self.alert_filters = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration of the alert filters, containing MetricList and DimensionFilterList.</p>
    pub fn set_alert_filters(
        mut self,
        input: ::std::option::Option<crate::types::AlertFilters>,
    ) -> Self {
        self.alert_filters = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateAlertInput`](crate::operation::update_alert::UpdateAlertInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_alert::UpdateAlertInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_alert::UpdateAlertInput {
            alert_arn: self.alert_arn,
            alert_description: self.alert_description,
            alert_sensitivity_threshold: self.alert_sensitivity_threshold.unwrap_or_default(),
            action: self.action,
            alert_filters: self.alert_filters,
        })
    }
}
