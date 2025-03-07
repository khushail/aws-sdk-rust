// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetRelationalDatabaseLogStreams`](crate::operation::get_relational_database_log_streams::builders::GetRelationalDatabaseLogStreamsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`relational_database_name(impl ::std::convert::Into<String>)`](crate::operation::get_relational_database_log_streams::builders::GetRelationalDatabaseLogStreamsFluentBuilder::relational_database_name) / [`set_relational_database_name(Option<String>)`](crate::operation::get_relational_database_log_streams::builders::GetRelationalDatabaseLogStreamsFluentBuilder::set_relational_database_name): <p>The name of your database for which to get log streams.</p>
    /// - On success, responds with [`GetRelationalDatabaseLogStreamsOutput`](crate::operation::get_relational_database_log_streams::GetRelationalDatabaseLogStreamsOutput) with field(s):
    ///   - [`log_streams(Option<Vec<String>>)`](crate::operation::get_relational_database_log_streams::GetRelationalDatabaseLogStreamsOutput::log_streams): <p>An object describing the result of your get relational database log streams request.</p>
    /// - On failure, responds with [`SdkError<GetRelationalDatabaseLogStreamsError>`](crate::operation::get_relational_database_log_streams::GetRelationalDatabaseLogStreamsError)
    pub fn get_relational_database_log_streams(&self) -> crate::operation::get_relational_database_log_streams::builders::GetRelationalDatabaseLogStreamsFluentBuilder{
        crate::operation::get_relational_database_log_streams::builders::GetRelationalDatabaseLogStreamsFluentBuilder::new(self.handle.clone())
    }
}
