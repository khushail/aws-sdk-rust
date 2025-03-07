// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartReadSetExportJob`](crate::operation::start_read_set_export_job::builders::StartReadSetExportJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`sequence_store_id(impl ::std::convert::Into<String>)`](crate::operation::start_read_set_export_job::builders::StartReadSetExportJobFluentBuilder::sequence_store_id) / [`set_sequence_store_id(Option<String>)`](crate::operation::start_read_set_export_job::builders::StartReadSetExportJobFluentBuilder::set_sequence_store_id): <p>The read set's sequence store ID.</p>
    ///   - [`destination(impl ::std::convert::Into<String>)`](crate::operation::start_read_set_export_job::builders::StartReadSetExportJobFluentBuilder::destination) / [`set_destination(Option<String>)`](crate::operation::start_read_set_export_job::builders::StartReadSetExportJobFluentBuilder::set_destination): <p>A location for exported files in Amazon S3.</p>
    ///   - [`role_arn(impl ::std::convert::Into<String>)`](crate::operation::start_read_set_export_job::builders::StartReadSetExportJobFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::start_read_set_export_job::builders::StartReadSetExportJobFluentBuilder::set_role_arn): <p>A service role for the job.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::start_read_set_export_job::builders::StartReadSetExportJobFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::start_read_set_export_job::builders::StartReadSetExportJobFluentBuilder::set_client_token): <p>To ensure that jobs don't run multiple times, specify a unique token for each job.</p>
    ///   - [`sources(Vec<ExportReadSet>)`](crate::operation::start_read_set_export_job::builders::StartReadSetExportJobFluentBuilder::sources) / [`set_sources(Option<Vec<ExportReadSet>>)`](crate::operation::start_read_set_export_job::builders::StartReadSetExportJobFluentBuilder::set_sources): <p>The job's source files.</p>
    /// - On success, responds with [`StartReadSetExportJobOutput`](crate::operation::start_read_set_export_job::StartReadSetExportJobOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::operation::start_read_set_export_job::StartReadSetExportJobOutput::id): <p>The job's ID.</p>
    ///   - [`sequence_store_id(Option<String>)`](crate::operation::start_read_set_export_job::StartReadSetExportJobOutput::sequence_store_id): <p>The read set's sequence store ID.</p>
    ///   - [`destination(Option<String>)`](crate::operation::start_read_set_export_job::StartReadSetExportJobOutput::destination): <p>The job's output location.</p>
    ///   - [`status(Option<ReadSetExportJobStatus>)`](crate::operation::start_read_set_export_job::StartReadSetExportJobOutput::status): <p>The job's status.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::operation::start_read_set_export_job::StartReadSetExportJobOutput::creation_time): <p>When the job was created.</p>
    /// - On failure, responds with [`SdkError<StartReadSetExportJobError>`](crate::operation::start_read_set_export_job::StartReadSetExportJobError)
    pub fn start_read_set_export_job(
        &self,
    ) -> crate::operation::start_read_set_export_job::builders::StartReadSetExportJobFluentBuilder
    {
        crate::operation::start_read_set_export_job::builders::StartReadSetExportJobFluentBuilder::new(self.handle.clone())
    }
}
