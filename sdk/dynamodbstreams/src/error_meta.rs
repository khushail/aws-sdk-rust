// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>The shard iterator has expired and can no longer be used to retrieve stream records. A shard iterator expires 15 minutes after it is retrieved using the <code>GetShardIterator</code> action.</p>
    ExpiredIteratorException(crate::types::error::ExpiredIteratorException),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(crate::types::error::InternalServerError),
    /// <p>There is no limit to the number of daily on-demand backups that can be taken. </p>
    /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>. </p>
    /// <p>When you are creating a table with one or more secondary indexes, you can have up to 250 such requests running at a time. However, if the table or index specifications are complex, then DynamoDB might temporarily reduce the number of concurrent operations.</p>
    /// <p>When importing into DynamoDB, up to 50 simultaneous import table operations are allowed per account.</p>
    /// <p>There is a soft account quota of 2,500 tables.</p>
    LimitExceededException(crate::types::error::LimitExceededException),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>The operation attempted to read past the oldest stream record in a shard.</p>
    /// <p>In DynamoDB Streams, there is a 24 hour limit on data retention. Stream records whose age exceeds this limit are subject to removal (trimming) from the stream. You might receive a TrimmedDataAccessException if:</p>
    /// <ul>
    /// <li> <p>You request a shard iterator with a sequence number older than the trim point (24 hours).</p> </li>
    /// <li> <p>You obtain a shard iterator, but before you use the iterator in a <code>GetRecords</code> request, a stream record in the shard exceeds the 24 hour period and is trimmed. This causes the iterator to access a record that no longer exists.</p> </li>
    /// </ul>
    TrimmedDataAccessException(crate::types::error::TrimmedDataAccessException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ExpiredIteratorException(inner) => inner.fmt(f),
            Error::InternalServerError(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::TrimmedDataAccessException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_stream::DescribeStreamError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::describe_stream::DescribeStreamError,
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
impl From<crate::operation::describe_stream::DescribeStreamError> for Error {
    fn from(err: crate::operation::describe_stream::DescribeStreamError) -> Self {
        match err {
            crate::operation::describe_stream::DescribeStreamError::InternalServerError(inner) => {
                Error::InternalServerError(inner)
            }
            crate::operation::describe_stream::DescribeStreamError::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::operation::describe_stream::DescribeStreamError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R> From<::aws_smithy_http::result::SdkError<crate::operation::get_records::GetRecordsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<crate::operation::get_records::GetRecordsError, R>,
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
impl From<crate::operation::get_records::GetRecordsError> for Error {
    fn from(err: crate::operation::get_records::GetRecordsError) -> Self {
        match err {
            crate::operation::get_records::GetRecordsError::ExpiredIteratorException(inner) => {
                Error::ExpiredIteratorException(inner)
            }
            crate::operation::get_records::GetRecordsError::InternalServerError(inner) => {
                Error::InternalServerError(inner)
            }
            crate::operation::get_records::GetRecordsError::LimitExceededException(inner) => {
                Error::LimitExceededException(inner)
            }
            crate::operation::get_records::GetRecordsError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::get_records::GetRecordsError::TrimmedDataAccessException(inner) => {
                Error::TrimmedDataAccessException(inner)
            }
            crate::operation::get_records::GetRecordsError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_shard_iterator::GetShardIteratorError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::get_shard_iterator::GetShardIteratorError,
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
impl From<crate::operation::get_shard_iterator::GetShardIteratorError> for Error {
    fn from(err: crate::operation::get_shard_iterator::GetShardIteratorError) -> Self {
        match err {
            crate::operation::get_shard_iterator::GetShardIteratorError::InternalServerError(inner) => Error::InternalServerError(inner),
            crate::operation::get_shard_iterator::GetShardIteratorError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::get_shard_iterator::GetShardIteratorError::TrimmedDataAccessException(inner) => Error::TrimmedDataAccessException(inner),
            crate::operation::get_shard_iterator::GetShardIteratorError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<::aws_smithy_http::result::SdkError<crate::operation::list_streams::ListStreamsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::list_streams::ListStreamsError,
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
impl From<crate::operation::list_streams::ListStreamsError> for Error {
    fn from(err: crate::operation::list_streams::ListStreamsError) -> Self {
        match err {
            crate::operation::list_streams::ListStreamsError::InternalServerError(inner) => {
                Error::InternalServerError(inner)
            }
            crate::operation::list_streams::ListStreamsError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::list_streams::ListStreamsError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::ExpiredIteratorException(inner) => inner.source(),
            Error::InternalServerError(inner) => inner.source(),
            Error::LimitExceededException(inner) => inner.source(),
            Error::ResourceNotFoundException(inner) => inner.source(),
            Error::TrimmedDataAccessException(inner) => inner.source(),
            Error::Unhandled(inner) => inner.source(),
        }
    }
}
impl ::aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::ExpiredIteratorException(e) => e.request_id(),
            Self::InternalServerError(e) => e.request_id(),
            Self::LimitExceededException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::TrimmedDataAccessException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}
