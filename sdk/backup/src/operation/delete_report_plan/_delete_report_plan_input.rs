// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteReportPlanInput {
    /// <p>The unique name of a report plan.</p>
    #[doc(hidden)]
    pub report_plan_name: ::std::option::Option<::std::string::String>,
}
impl DeleteReportPlanInput {
    /// <p>The unique name of a report plan.</p>
    pub fn report_plan_name(&self) -> ::std::option::Option<&str> {
        self.report_plan_name.as_deref()
    }
}
impl DeleteReportPlanInput {
    /// Creates a new builder-style object to manufacture [`DeleteReportPlanInput`](crate::operation::delete_report_plan::DeleteReportPlanInput).
    pub fn builder() -> crate::operation::delete_report_plan::builders::DeleteReportPlanInputBuilder
    {
        crate::operation::delete_report_plan::builders::DeleteReportPlanInputBuilder::default()
    }
}

/// A builder for [`DeleteReportPlanInput`](crate::operation::delete_report_plan::DeleteReportPlanInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteReportPlanInputBuilder {
    pub(crate) report_plan_name: ::std::option::Option<::std::string::String>,
}
impl DeleteReportPlanInputBuilder {
    /// <p>The unique name of a report plan.</p>
    pub fn report_plan_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.report_plan_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique name of a report plan.</p>
    pub fn set_report_plan_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.report_plan_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteReportPlanInput`](crate::operation::delete_report_plan::DeleteReportPlanInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_report_plan::DeleteReportPlanInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_report_plan::DeleteReportPlanInput {
                report_plan_name: self.report_plan_name,
            },
        )
    }
}
