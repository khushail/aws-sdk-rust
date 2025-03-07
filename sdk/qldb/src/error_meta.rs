// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>One or more parameters in the request aren't valid.</p>
    InvalidParameterException(crate::types::error::InvalidParameterException),
    /// <p>You have reached the limit on the maximum number of resources allowed.</p>
    LimitExceededException(crate::types::error::LimitExceededException),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExistsException(crate::types::error::ResourceAlreadyExistsException),
    /// <p>The specified resource can't be modified at this time.</p>
    ResourceInUseException(crate::types::error::ResourceInUseException),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>The operation failed because a condition wasn't satisfied in advance.</p>
    ResourcePreconditionNotMetException(crate::types::error::ResourcePreconditionNotMetException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidParameterException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ResourceAlreadyExistsException(inner) => inner.fmt(f),
            Error::ResourceInUseException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ResourcePreconditionNotMetException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::cancel_journal_kinesis_stream::CancelJournalKinesisStreamError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::cancel_journal_kinesis_stream::CancelJournalKinesisStreamError,
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
impl From<crate::operation::cancel_journal_kinesis_stream::CancelJournalKinesisStreamError>
    for Error
{
    fn from(
        err: crate::operation::cancel_journal_kinesis_stream::CancelJournalKinesisStreamError,
    ) -> Self {
        match err {
            crate::operation::cancel_journal_kinesis_stream::CancelJournalKinesisStreamError::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::operation::cancel_journal_kinesis_stream::CancelJournalKinesisStreamError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::cancel_journal_kinesis_stream::CancelJournalKinesisStreamError::ResourcePreconditionNotMetException(inner) => Error::ResourcePreconditionNotMetException(inner),
            crate::operation::cancel_journal_kinesis_stream::CancelJournalKinesisStreamError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<::aws_smithy_http::result::SdkError<crate::operation::create_ledger::CreateLedgerError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::create_ledger::CreateLedgerError,
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
impl From<crate::operation::create_ledger::CreateLedgerError> for Error {
    fn from(err: crate::operation::create_ledger::CreateLedgerError) -> Self {
        match err {
            crate::operation::create_ledger::CreateLedgerError::InvalidParameterException(
                inner,
            ) => Error::InvalidParameterException(inner),
            crate::operation::create_ledger::CreateLedgerError::LimitExceededException(inner) => {
                Error::LimitExceededException(inner)
            }
            crate::operation::create_ledger::CreateLedgerError::ResourceAlreadyExistsException(
                inner,
            ) => Error::ResourceAlreadyExistsException(inner),
            crate::operation::create_ledger::CreateLedgerError::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::operation::create_ledger::CreateLedgerError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<::aws_smithy_http::result::SdkError<crate::operation::delete_ledger::DeleteLedgerError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::delete_ledger::DeleteLedgerError,
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
impl From<crate::operation::delete_ledger::DeleteLedgerError> for Error {
    fn from(err: crate::operation::delete_ledger::DeleteLedgerError) -> Self {
        match err {
            crate::operation::delete_ledger::DeleteLedgerError::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::operation::delete_ledger::DeleteLedgerError::ResourceInUseException(inner) => Error::ResourceInUseException(inner),
            crate::operation::delete_ledger::DeleteLedgerError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::delete_ledger::DeleteLedgerError::ResourcePreconditionNotMetException(inner) => Error::ResourcePreconditionNotMetException(inner),
            crate::operation::delete_ledger::DeleteLedgerError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_journal_kinesis_stream::DescribeJournalKinesisStreamError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::describe_journal_kinesis_stream::DescribeJournalKinesisStreamError,
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
impl From<crate::operation::describe_journal_kinesis_stream::DescribeJournalKinesisStreamError>
    for Error
{
    fn from(
        err: crate::operation::describe_journal_kinesis_stream::DescribeJournalKinesisStreamError,
    ) -> Self {
        match err {
            crate::operation::describe_journal_kinesis_stream::DescribeJournalKinesisStreamError::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::operation::describe_journal_kinesis_stream::DescribeJournalKinesisStreamError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::describe_journal_kinesis_stream::DescribeJournalKinesisStreamError::ResourcePreconditionNotMetException(inner) => Error::ResourcePreconditionNotMetException(inner),
            crate::operation::describe_journal_kinesis_stream::DescribeJournalKinesisStreamError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_journal_s3_export::DescribeJournalS3ExportError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::describe_journal_s3_export::DescribeJournalS3ExportError,
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
impl From<crate::operation::describe_journal_s3_export::DescribeJournalS3ExportError> for Error {
    fn from(
        err: crate::operation::describe_journal_s3_export::DescribeJournalS3ExportError,
    ) -> Self {
        match err {
            crate::operation::describe_journal_s3_export::DescribeJournalS3ExportError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::describe_journal_s3_export::DescribeJournalS3ExportError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_ledger::DescribeLedgerError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::describe_ledger::DescribeLedgerError,
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
impl From<crate::operation::describe_ledger::DescribeLedgerError> for Error {
    fn from(err: crate::operation::describe_ledger::DescribeLedgerError) -> Self {
        match err {
            crate::operation::describe_ledger::DescribeLedgerError::InvalidParameterException(
                inner,
            ) => Error::InvalidParameterException(inner),
            crate::operation::describe_ledger::DescribeLedgerError::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::operation::describe_ledger::DescribeLedgerError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::export_journal_to_s3::ExportJournalToS3Error,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::export_journal_to_s3::ExportJournalToS3Error,
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
impl From<crate::operation::export_journal_to_s3::ExportJournalToS3Error> for Error {
    fn from(err: crate::operation::export_journal_to_s3::ExportJournalToS3Error) -> Self {
        match err {
            crate::operation::export_journal_to_s3::ExportJournalToS3Error::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::export_journal_to_s3::ExportJournalToS3Error::ResourcePreconditionNotMetException(inner) => Error::ResourcePreconditionNotMetException(inner),
            crate::operation::export_journal_to_s3::ExportJournalToS3Error::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_http::result::SdkError<crate::operation::get_block::GetBlockError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<crate::operation::get_block::GetBlockError, R>,
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
impl From<crate::operation::get_block::GetBlockError> for Error {
    fn from(err: crate::operation::get_block::GetBlockError) -> Self {
        match err {
            crate::operation::get_block::GetBlockError::InvalidParameterException(inner) => {
                Error::InvalidParameterException(inner)
            }
            crate::operation::get_block::GetBlockError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::get_block::GetBlockError::ResourcePreconditionNotMetException(
                inner,
            ) => Error::ResourcePreconditionNotMetException(inner),
            crate::operation::get_block::GetBlockError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_http::result::SdkError<crate::operation::get_digest::GetDigestError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<crate::operation::get_digest::GetDigestError, R>,
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
impl From<crate::operation::get_digest::GetDigestError> for Error {
    fn from(err: crate::operation::get_digest::GetDigestError) -> Self {
        match err {
            crate::operation::get_digest::GetDigestError::InvalidParameterException(inner) => {
                Error::InvalidParameterException(inner)
            }
            crate::operation::get_digest::GetDigestError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::get_digest::GetDigestError::ResourcePreconditionNotMetException(
                inner,
            ) => Error::ResourcePreconditionNotMetException(inner),
            crate::operation::get_digest::GetDigestError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<::aws_smithy_http::result::SdkError<crate::operation::get_revision::GetRevisionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::get_revision::GetRevisionError,
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
impl From<crate::operation::get_revision::GetRevisionError> for Error {
    fn from(err: crate::operation::get_revision::GetRevisionError) -> Self {
        match err {
            crate::operation::get_revision::GetRevisionError::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::operation::get_revision::GetRevisionError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::get_revision::GetRevisionError::ResourcePreconditionNotMetException(inner) => Error::ResourcePreconditionNotMetException(inner),
            crate::operation::get_revision::GetRevisionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_http::result::SdkError<crate::operation::list_journal_kinesis_streams_for_ledger::ListJournalKinesisStreamsForLedgerError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: ::aws_smithy_http::result::SdkError<crate::operation::list_journal_kinesis_streams_for_ledger::ListJournalKinesisStreamsForLedgerError, R>) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                                            ::aws_smithy_types::error::Unhandled::builder()
                                                .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                                                .source(err)
                                                .build()
                                        ),
        }
    }
}
impl From<crate::operation::list_journal_kinesis_streams_for_ledger::ListJournalKinesisStreamsForLedgerError> for Error {
    fn from(err: crate::operation::list_journal_kinesis_streams_for_ledger::ListJournalKinesisStreamsForLedgerError) -> Self {
        match err {
            crate::operation::list_journal_kinesis_streams_for_ledger::ListJournalKinesisStreamsForLedgerError::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::operation::list_journal_kinesis_streams_for_ledger::ListJournalKinesisStreamsForLedgerError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::list_journal_kinesis_streams_for_ledger::ListJournalKinesisStreamsForLedgerError::ResourcePreconditionNotMetException(inner) => Error::ResourcePreconditionNotMetException(inner),
            crate::operation::list_journal_kinesis_streams_for_ledger::ListJournalKinesisStreamsForLedgerError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_journal_s3_exports::ListJournalS3ExportsError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::list_journal_s3_exports::ListJournalS3ExportsError,
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
impl From<crate::operation::list_journal_s3_exports::ListJournalS3ExportsError> for Error {
    fn from(err: crate::operation::list_journal_s3_exports::ListJournalS3ExportsError) -> Self {
        match err {
            crate::operation::list_journal_s3_exports::ListJournalS3ExportsError::Unhandled(
                inner,
            ) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_http::result::SdkError<crate::operation::list_journal_s3_exports_for_ledger::ListJournalS3ExportsForLedgerError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: ::aws_smithy_http::result::SdkError<crate::operation::list_journal_s3_exports_for_ledger::ListJournalS3ExportsForLedgerError, R>) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                                            ::aws_smithy_types::error::Unhandled::builder()
                                                .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                                                .source(err)
                                                .build()
                                        ),
        }
    }
}
impl From<crate::operation::list_journal_s3_exports_for_ledger::ListJournalS3ExportsForLedgerError>
    for Error
{
    fn from(
        err: crate::operation::list_journal_s3_exports_for_ledger::ListJournalS3ExportsForLedgerError,
    ) -> Self {
        match err {
            crate::operation::list_journal_s3_exports_for_ledger::ListJournalS3ExportsForLedgerError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<::aws_smithy_http::result::SdkError<crate::operation::list_ledgers::ListLedgersError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::list_ledgers::ListLedgersError,
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
impl From<crate::operation::list_ledgers::ListLedgersError> for Error {
    fn from(err: crate::operation::list_ledgers::ListLedgersError) -> Self {
        match err {
            crate::operation::list_ledgers::ListLedgersError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_tags_for_resource::ListTagsForResourceError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::list_tags_for_resource::ListTagsForResourceError,
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
impl From<crate::operation::list_tags_for_resource::ListTagsForResourceError> for Error {
    fn from(err: crate::operation::list_tags_for_resource::ListTagsForResourceError) -> Self {
        match err {
            crate::operation::list_tags_for_resource::ListTagsForResourceError::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::list_tags_for_resource::ListTagsForResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::stream_journal_to_kinesis::StreamJournalToKinesisError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::stream_journal_to_kinesis::StreamJournalToKinesisError,
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
impl From<crate::operation::stream_journal_to_kinesis::StreamJournalToKinesisError> for Error {
    fn from(err: crate::operation::stream_journal_to_kinesis::StreamJournalToKinesisError) -> Self {
        match err {
            crate::operation::stream_journal_to_kinesis::StreamJournalToKinesisError::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::operation::stream_journal_to_kinesis::StreamJournalToKinesisError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::stream_journal_to_kinesis::StreamJournalToKinesisError::ResourcePreconditionNotMetException(inner) => Error::ResourcePreconditionNotMetException(inner),
            crate::operation::stream_journal_to_kinesis::StreamJournalToKinesisError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<::aws_smithy_http::result::SdkError<crate::operation::tag_resource::TagResourceError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::tag_resource::TagResourceError,
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
impl From<crate::operation::tag_resource::TagResourceError> for Error {
    fn from(err: crate::operation::tag_resource::TagResourceError) -> Self {
        match err {
            crate::operation::tag_resource::TagResourceError::InvalidParameterException(inner) => {
                Error::InvalidParameterException(inner)
            }
            crate::operation::tag_resource::TagResourceError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::tag_resource::TagResourceError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::untag_resource::UntagResourceError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::untag_resource::UntagResourceError,
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
impl From<crate::operation::untag_resource::UntagResourceError> for Error {
    fn from(err: crate::operation::untag_resource::UntagResourceError) -> Self {
        match err {
            crate::operation::untag_resource::UntagResourceError::InvalidParameterException(
                inner,
            ) => Error::InvalidParameterException(inner),
            crate::operation::untag_resource::UntagResourceError::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::operation::untag_resource::UntagResourceError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<::aws_smithy_http::result::SdkError<crate::operation::update_ledger::UpdateLedgerError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::update_ledger::UpdateLedgerError,
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
impl From<crate::operation::update_ledger::UpdateLedgerError> for Error {
    fn from(err: crate::operation::update_ledger::UpdateLedgerError) -> Self {
        match err {
            crate::operation::update_ledger::UpdateLedgerError::InvalidParameterException(
                inner,
            ) => Error::InvalidParameterException(inner),
            crate::operation::update_ledger::UpdateLedgerError::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::operation::update_ledger::UpdateLedgerError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_ledger_permissions_mode::UpdateLedgerPermissionsModeError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::update_ledger_permissions_mode::UpdateLedgerPermissionsModeError,
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
impl From<crate::operation::update_ledger_permissions_mode::UpdateLedgerPermissionsModeError>
    for Error
{
    fn from(
        err: crate::operation::update_ledger_permissions_mode::UpdateLedgerPermissionsModeError,
    ) -> Self {
        match err {
            crate::operation::update_ledger_permissions_mode::UpdateLedgerPermissionsModeError::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::operation::update_ledger_permissions_mode::UpdateLedgerPermissionsModeError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::update_ledger_permissions_mode::UpdateLedgerPermissionsModeError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::InvalidParameterException(inner) => inner.source(),
            Error::LimitExceededException(inner) => inner.source(),
            Error::ResourceAlreadyExistsException(inner) => inner.source(),
            Error::ResourceInUseException(inner) => inner.source(),
            Error::ResourceNotFoundException(inner) => inner.source(),
            Error::ResourcePreconditionNotMetException(inner) => inner.source(),
            Error::Unhandled(inner) => inner.source(),
        }
    }
}
impl ::aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::InvalidParameterException(e) => e.request_id(),
            Self::LimitExceededException(e) => e.request_id(),
            Self::ResourceAlreadyExistsException(e) => e.request_id(),
            Self::ResourceInUseException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::ResourcePreconditionNotMetException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}
