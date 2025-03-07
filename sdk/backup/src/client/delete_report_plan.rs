// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteReportPlan`](crate::operation::delete_report_plan::builders::DeleteReportPlanFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`report_plan_name(impl ::std::convert::Into<String>)`](crate::operation::delete_report_plan::builders::DeleteReportPlanFluentBuilder::report_plan_name) / [`set_report_plan_name(Option<String>)`](crate::operation::delete_report_plan::builders::DeleteReportPlanFluentBuilder::set_report_plan_name): <p>The unique name of a report plan.</p>
    /// - On success, responds with [`DeleteReportPlanOutput`](crate::operation::delete_report_plan::DeleteReportPlanOutput)
    /// - On failure, responds with [`SdkError<DeleteReportPlanError>`](crate::operation::delete_report_plan::DeleteReportPlanError)
    pub fn delete_report_plan(
        &self,
    ) -> crate::operation::delete_report_plan::builders::DeleteReportPlanFluentBuilder {
        crate::operation::delete_report_plan::builders::DeleteReportPlanFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
