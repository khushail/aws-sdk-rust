// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>The number of active statements exceeds the limit.</p>
    ActiveStatementsExceededException(crate::types::error::ActiveStatementsExceededException),
    /// <p>An SQL statement encountered an environmental error while running.</p>
    BatchExecuteStatementException(crate::types::error::BatchExecuteStatementException),
    /// <p>Connection to a database failed.</p>
    DatabaseConnectionException(crate::types::error::DatabaseConnectionException),
    /// <p>The SQL statement encountered an environmental error while running.</p>
    ExecuteStatementException(crate::types::error::ExecuteStatementException),
    /// <p>The Amazon Redshift Data API operation failed due to invalid input. </p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p>The Amazon Redshift Data API operation failed due to a missing resource. </p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>The Amazon Redshift Data API operation failed due to invalid input. </p>
    ValidationException(crate::types::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ActiveStatementsExceededException(inner) => inner.fmt(f),
            Error::BatchExecuteStatementException(inner) => inner.fmt(f),
            Error::DatabaseConnectionException(inner) => inner.fmt(f),
            Error::ExecuteStatementException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_execute_statement::BatchExecuteStatementError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::batch_execute_statement::BatchExecuteStatementError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::batch_execute_statement::BatchExecuteStatementError> for Error {
    fn from(err: crate::operation::batch_execute_statement::BatchExecuteStatementError) -> Self {
        match err {
            crate::operation::batch_execute_statement::BatchExecuteStatementError::ActiveStatementsExceededException(inner) => Error::ActiveStatementsExceededException(inner),
            crate::operation::batch_execute_statement::BatchExecuteStatementError::BatchExecuteStatementException(inner) => Error::BatchExecuteStatementException(inner),
            crate::operation::batch_execute_statement::BatchExecuteStatementError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::batch_execute_statement::BatchExecuteStatementError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::cancel_statement::CancelStatementError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::cancel_statement::CancelStatementError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::cancel_statement::CancelStatementError> for Error {
    fn from(err: crate::operation::cancel_statement::CancelStatementError) -> Self {
        match err {
            crate::operation::cancel_statement::CancelStatementError::DatabaseConnectionException(inner) => Error::DatabaseConnectionException(inner),
            crate::operation::cancel_statement::CancelStatementError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::cancel_statement::CancelStatementError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::cancel_statement::CancelStatementError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::cancel_statement::CancelStatementError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_statement::DescribeStatementError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::describe_statement::DescribeStatementError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::describe_statement::DescribeStatementError> for Error {
    fn from(err: crate::operation::describe_statement::DescribeStatementError) -> Self {
        match err {
            crate::operation::describe_statement::DescribeStatementError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::describe_statement::DescribeStatementError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::describe_statement::DescribeStatementError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::describe_statement::DescribeStatementError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_table::DescribeTableError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::describe_table::DescribeTableError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::describe_table::DescribeTableError> for Error {
    fn from(err: crate::operation::describe_table::DescribeTableError) -> Self {
        match err {
            crate::operation::describe_table::DescribeTableError::DatabaseConnectionException(
                inner,
            ) => Error::DatabaseConnectionException(inner),
            crate::operation::describe_table::DescribeTableError::InternalServerException(
                inner,
            ) => Error::InternalServerException(inner),
            crate::operation::describe_table::DescribeTableError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::describe_table::DescribeTableError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::execute_statement::ExecuteStatementError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::execute_statement::ExecuteStatementError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::execute_statement::ExecuteStatementError> for Error {
    fn from(err: crate::operation::execute_statement::ExecuteStatementError) -> Self {
        match err {
            crate::operation::execute_statement::ExecuteStatementError::ActiveStatementsExceededException(inner) => Error::ActiveStatementsExceededException(inner),
            crate::operation::execute_statement::ExecuteStatementError::ExecuteStatementException(inner) => Error::ExecuteStatementException(inner),
            crate::operation::execute_statement::ExecuteStatementError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::execute_statement::ExecuteStatementError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_statement_result::GetStatementResultError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::get_statement_result::GetStatementResultError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::get_statement_result::GetStatementResultError> for Error {
    fn from(err: crate::operation::get_statement_result::GetStatementResultError) -> Self {
        match err {
            crate::operation::get_statement_result::GetStatementResultError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_statement_result::GetStatementResultError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::get_statement_result::GetStatementResultError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_statement_result::GetStatementResultError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_databases::ListDatabasesError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::list_databases::ListDatabasesError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::list_databases::ListDatabasesError> for Error {
    fn from(err: crate::operation::list_databases::ListDatabasesError) -> Self {
        match err {
            crate::operation::list_databases::ListDatabasesError::DatabaseConnectionException(
                inner,
            ) => Error::DatabaseConnectionException(inner),
            crate::operation::list_databases::ListDatabasesError::InternalServerException(
                inner,
            ) => Error::InternalServerException(inner),
            crate::operation::list_databases::ListDatabasesError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::list_databases::ListDatabasesError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<::aws_smithy_http::result::SdkError<crate::operation::list_schemas::ListSchemasError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::list_schemas::ListSchemasError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::list_schemas::ListSchemasError> for Error {
    fn from(err: crate::operation::list_schemas::ListSchemasError) -> Self {
        match err {
            crate::operation::list_schemas::ListSchemasError::DatabaseConnectionException(
                inner,
            ) => Error::DatabaseConnectionException(inner),
            crate::operation::list_schemas::ListSchemasError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::list_schemas::ListSchemasError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::list_schemas::ListSchemasError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_statements::ListStatementsError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::list_statements::ListStatementsError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::list_statements::ListStatementsError> for Error {
    fn from(err: crate::operation::list_statements::ListStatementsError) -> Self {
        match err {
            crate::operation::list_statements::ListStatementsError::InternalServerException(
                inner,
            ) => Error::InternalServerException(inner),
            crate::operation::list_statements::ListStatementsError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::list_statements::ListStatementsError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R> From<::aws_smithy_http::result::SdkError<crate::operation::list_tables::ListTablesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<crate::operation::list_tables::ListTablesError, R>,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::list_tables::ListTablesError> for Error {
    fn from(err: crate::operation::list_tables::ListTablesError) -> Self {
        match err {
            crate::operation::list_tables::ListTablesError::DatabaseConnectionException(inner) => {
                Error::DatabaseConnectionException(inner)
            }
            crate::operation::list_tables::ListTablesError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::list_tables::ListTablesError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::list_tables::ListTablesError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::ActiveStatementsExceededException(inner) => inner.source(),
            Error::BatchExecuteStatementException(inner) => inner.source(),
            Error::DatabaseConnectionException(inner) => inner.source(),
            Error::ExecuteStatementException(inner) => inner.source(),
            Error::InternalServerException(inner) => inner.source(),
            Error::ResourceNotFoundException(inner) => inner.source(),
            Error::ValidationException(inner) => inner.source(),
            Error::Unhandled(inner) => inner.source(),
        }
    }
}
impl ::aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::ActiveStatementsExceededException(e) => e.request_id(),
            Self::BatchExecuteStatementException(e) => e.request_id(),
            Self::DatabaseConnectionException(e) => e.request_id(),
            Self::ExecuteStatementException(e) => e.request_id(),
            Self::InternalServerException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::ValidationException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}
