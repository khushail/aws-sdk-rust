// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetReportDefinition`](crate::operation::get_report_definition::builders::GetReportDefinitionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`report_id(impl ::std::convert::Into<String>)`](crate::operation::get_report_definition::builders::GetReportDefinitionFluentBuilder::report_id) / [`set_report_id(Option<String>)`](crate::operation::get_report_definition::builders::GetReportDefinitionFluentBuilder::set_report_id): <p>ID of the report to retrieve.</p>
    /// - On success, responds with [`GetReportDefinitionOutput`](crate::operation::get_report_definition::GetReportDefinitionOutput) with field(s):
    ///   - [`report_id(Option<String>)`](crate::operation::get_report_definition::GetReportDefinitionOutput::report_id): <p>ID of the report retrieved.</p>
    ///   - [`report_description(Option<String>)`](crate::operation::get_report_definition::GetReportDefinitionOutput::report_description): <p>Description of the report.</p>
    ///   - [`report_frequency(Option<ReportFrequency>)`](crate::operation::get_report_definition::GetReportDefinitionOutput::report_frequency): <p>Cadence used to generate the report.</p>
    ///   - [`format(Option<Format>)`](crate::operation::get_report_definition::GetReportDefinitionOutput::format): <p>Format of the generated report.</p>
    ///   - [`destination_s3_location(Option<S3Location>)`](crate::operation::get_report_definition::GetReportDefinitionOutput::destination_s3_location): <p>Amazon Simple Storage Service (Amazon S3) location where the report is uploaded.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::operation::get_report_definition::GetReportDefinitionOutput::created_at): <p>Timestamp (milliseconds) when this report definition was created.</p>
    ///   - [`last_updated(Option<DateTime>)`](crate::operation::get_report_definition::GetReportDefinitionOutput::last_updated): <p>Timestamp (milliseconds) when this report definition was last updated.</p>
    /// - On failure, responds with [`SdkError<GetReportDefinitionError>`](crate::operation::get_report_definition::GetReportDefinitionError)
    pub fn get_report_definition(
        &self,
    ) -> crate::operation::get_report_definition::builders::GetReportDefinitionFluentBuilder {
        crate::operation::get_report_definition::builders::GetReportDefinitionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
