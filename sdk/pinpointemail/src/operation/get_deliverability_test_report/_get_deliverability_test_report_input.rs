// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A request to retrieve the results of a predictive inbox placement test.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetDeliverabilityTestReportInput {
    /// <p>A unique string that identifies the predictive inbox placement test.</p>
    #[doc(hidden)]
    pub report_id: ::std::option::Option<::std::string::String>,
}
impl GetDeliverabilityTestReportInput {
    /// <p>A unique string that identifies the predictive inbox placement test.</p>
    pub fn report_id(&self) -> ::std::option::Option<&str> {
        self.report_id.as_deref()
    }
}
impl GetDeliverabilityTestReportInput {
    /// Creates a new builder-style object to manufacture [`GetDeliverabilityTestReportInput`](crate::operation::get_deliverability_test_report::GetDeliverabilityTestReportInput).
    pub fn builder() -> crate::operation::get_deliverability_test_report::builders::GetDeliverabilityTestReportInputBuilder{
        crate::operation::get_deliverability_test_report::builders::GetDeliverabilityTestReportInputBuilder::default()
    }
}

/// A builder for [`GetDeliverabilityTestReportInput`](crate::operation::get_deliverability_test_report::GetDeliverabilityTestReportInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetDeliverabilityTestReportInputBuilder {
    pub(crate) report_id: ::std::option::Option<::std::string::String>,
}
impl GetDeliverabilityTestReportInputBuilder {
    /// <p>A unique string that identifies the predictive inbox placement test.</p>
    pub fn report_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.report_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique string that identifies the predictive inbox placement test.</p>
    pub fn set_report_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.report_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetDeliverabilityTestReportInput`](crate::operation::get_deliverability_test_report::GetDeliverabilityTestReportInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_deliverability_test_report::GetDeliverabilityTestReportInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_deliverability_test_report::GetDeliverabilityTestReportInput {
                report_id: self.report_id,
            },
        )
    }
}
