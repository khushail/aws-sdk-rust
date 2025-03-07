// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>You don't have sufficient permissions to perform this action.</p>
    AccessDeniedException(crate::types::error::AccessDeniedException),
    /// <p>This operation attempted to create a resource that already exists.</p>
    ConflictException(crate::types::error::ConflictException),
    /// <p>Internal service exception.</p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p>Resource not found.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>This request exceeds a service quota.</p>
    ServiceQuotaExceededException(crate::types::error::ServiceQuotaExceededException),
    /// <p>The request was throttled because of quota limits.</p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// <p>One of the arguments for the request is not valid.</p>
    ValidationException(crate::types::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<::aws_smithy_http::result::SdkError<crate::operation::batch_create_rum_metric_definitions::BatchCreateRumMetricDefinitionsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: ::aws_smithy_http::result::SdkError<crate::operation::batch_create_rum_metric_definitions::BatchCreateRumMetricDefinitionsError, R>) -> Self {
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
impl
    From<
        crate::operation::batch_create_rum_metric_definitions::BatchCreateRumMetricDefinitionsError,
    > for Error
{
    fn from(
        err: crate::operation::batch_create_rum_metric_definitions::BatchCreateRumMetricDefinitionsError,
    ) -> Self {
        match err {
            crate::operation::batch_create_rum_metric_definitions::BatchCreateRumMetricDefinitionsError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::batch_create_rum_metric_definitions::BatchCreateRumMetricDefinitionsError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::batch_create_rum_metric_definitions::BatchCreateRumMetricDefinitionsError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::batch_create_rum_metric_definitions::BatchCreateRumMetricDefinitionsError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::batch_create_rum_metric_definitions::BatchCreateRumMetricDefinitionsError::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::operation::batch_create_rum_metric_definitions::BatchCreateRumMetricDefinitionsError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::batch_create_rum_metric_definitions::BatchCreateRumMetricDefinitionsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::batch_create_rum_metric_definitions::BatchCreateRumMetricDefinitionsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_http::result::SdkError<crate::operation::batch_delete_rum_metric_definitions::BatchDeleteRumMetricDefinitionsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: ::aws_smithy_http::result::SdkError<crate::operation::batch_delete_rum_metric_definitions::BatchDeleteRumMetricDefinitionsError, R>) -> Self {
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
impl
    From<
        crate::operation::batch_delete_rum_metric_definitions::BatchDeleteRumMetricDefinitionsError,
    > for Error
{
    fn from(
        err: crate::operation::batch_delete_rum_metric_definitions::BatchDeleteRumMetricDefinitionsError,
    ) -> Self {
        match err {
            crate::operation::batch_delete_rum_metric_definitions::BatchDeleteRumMetricDefinitionsError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::batch_delete_rum_metric_definitions::BatchDeleteRumMetricDefinitionsError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::batch_delete_rum_metric_definitions::BatchDeleteRumMetricDefinitionsError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::batch_delete_rum_metric_definitions::BatchDeleteRumMetricDefinitionsError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::batch_delete_rum_metric_definitions::BatchDeleteRumMetricDefinitionsError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::batch_delete_rum_metric_definitions::BatchDeleteRumMetricDefinitionsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::batch_delete_rum_metric_definitions::BatchDeleteRumMetricDefinitionsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_get_rum_metric_definitions::BatchGetRumMetricDefinitionsError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::batch_get_rum_metric_definitions::BatchGetRumMetricDefinitionsError,
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
impl From<crate::operation::batch_get_rum_metric_definitions::BatchGetRumMetricDefinitionsError>
    for Error
{
    fn from(
        err: crate::operation::batch_get_rum_metric_definitions::BatchGetRumMetricDefinitionsError,
    ) -> Self {
        match err {
            crate::operation::batch_get_rum_metric_definitions::BatchGetRumMetricDefinitionsError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::batch_get_rum_metric_definitions::BatchGetRumMetricDefinitionsError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::batch_get_rum_metric_definitions::BatchGetRumMetricDefinitionsError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::batch_get_rum_metric_definitions::BatchGetRumMetricDefinitionsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::batch_get_rum_metric_definitions::BatchGetRumMetricDefinitionsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_app_monitor::CreateAppMonitorError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::create_app_monitor::CreateAppMonitorError,
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
impl From<crate::operation::create_app_monitor::CreateAppMonitorError> for Error {
    fn from(err: crate::operation::create_app_monitor::CreateAppMonitorError) -> Self {
        match err {
            crate::operation::create_app_monitor::CreateAppMonitorError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::create_app_monitor::CreateAppMonitorError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::create_app_monitor::CreateAppMonitorError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::create_app_monitor::CreateAppMonitorError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::create_app_monitor::CreateAppMonitorError::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::operation::create_app_monitor::CreateAppMonitorError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::create_app_monitor::CreateAppMonitorError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::create_app_monitor::CreateAppMonitorError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_app_monitor::DeleteAppMonitorError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::delete_app_monitor::DeleteAppMonitorError,
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
impl From<crate::operation::delete_app_monitor::DeleteAppMonitorError> for Error {
    fn from(err: crate::operation::delete_app_monitor::DeleteAppMonitorError) -> Self {
        match err {
            crate::operation::delete_app_monitor::DeleteAppMonitorError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::delete_app_monitor::DeleteAppMonitorError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::delete_app_monitor::DeleteAppMonitorError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::delete_app_monitor::DeleteAppMonitorError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::delete_app_monitor::DeleteAppMonitorError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::delete_app_monitor::DeleteAppMonitorError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::delete_app_monitor::DeleteAppMonitorError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_rum_metrics_destination::DeleteRumMetricsDestinationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::delete_rum_metrics_destination::DeleteRumMetricsDestinationError,
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
impl From<crate::operation::delete_rum_metrics_destination::DeleteRumMetricsDestinationError>
    for Error
{
    fn from(
        err: crate::operation::delete_rum_metrics_destination::DeleteRumMetricsDestinationError,
    ) -> Self {
        match err {
            crate::operation::delete_rum_metrics_destination::DeleteRumMetricsDestinationError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::delete_rum_metrics_destination::DeleteRumMetricsDestinationError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::delete_rum_metrics_destination::DeleteRumMetricsDestinationError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::delete_rum_metrics_destination::DeleteRumMetricsDestinationError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::delete_rum_metrics_destination::DeleteRumMetricsDestinationError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::delete_rum_metrics_destination::DeleteRumMetricsDestinationError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::delete_rum_metrics_destination::DeleteRumMetricsDestinationError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_app_monitor::GetAppMonitorError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::get_app_monitor::GetAppMonitorError,
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
impl From<crate::operation::get_app_monitor::GetAppMonitorError> for Error {
    fn from(err: crate::operation::get_app_monitor::GetAppMonitorError) -> Self {
        match err {
            crate::operation::get_app_monitor::GetAppMonitorError::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::operation::get_app_monitor::GetAppMonitorError::InternalServerException(
                inner,
            ) => Error::InternalServerException(inner),
            crate::operation::get_app_monitor::GetAppMonitorError::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::operation::get_app_monitor::GetAppMonitorError::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::operation::get_app_monitor::GetAppMonitorError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::get_app_monitor::GetAppMonitorError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_app_monitor_data::GetAppMonitorDataError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::get_app_monitor_data::GetAppMonitorDataError,
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
impl From<crate::operation::get_app_monitor_data::GetAppMonitorDataError> for Error {
    fn from(err: crate::operation::get_app_monitor_data::GetAppMonitorDataError) -> Self {
        match err {
            crate::operation::get_app_monitor_data::GetAppMonitorDataError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::get_app_monitor_data::GetAppMonitorDataError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_app_monitor_data::GetAppMonitorDataError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::get_app_monitor_data::GetAppMonitorDataError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::get_app_monitor_data::GetAppMonitorDataError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_app_monitor_data::GetAppMonitorDataError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_app_monitors::ListAppMonitorsError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::list_app_monitors::ListAppMonitorsError,
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
impl From<crate::operation::list_app_monitors::ListAppMonitorsError> for Error {
    fn from(err: crate::operation::list_app_monitors::ListAppMonitorsError) -> Self {
        match err {
            crate::operation::list_app_monitors::ListAppMonitorsError::AccessDeniedException(
                inner,
            ) => Error::AccessDeniedException(inner),
            crate::operation::list_app_monitors::ListAppMonitorsError::InternalServerException(
                inner,
            ) => Error::InternalServerException(inner),
            crate::operation::list_app_monitors::ListAppMonitorsError::ThrottlingException(
                inner,
            ) => Error::ThrottlingException(inner),
            crate::operation::list_app_monitors::ListAppMonitorsError::ValidationException(
                inner,
            ) => Error::ValidationException(inner),
            crate::operation::list_app_monitors::ListAppMonitorsError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_rum_metrics_destinations::ListRumMetricsDestinationsError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::list_rum_metrics_destinations::ListRumMetricsDestinationsError,
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
impl From<crate::operation::list_rum_metrics_destinations::ListRumMetricsDestinationsError>
    for Error
{
    fn from(
        err: crate::operation::list_rum_metrics_destinations::ListRumMetricsDestinationsError,
    ) -> Self {
        match err {
            crate::operation::list_rum_metrics_destinations::ListRumMetricsDestinationsError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::list_rum_metrics_destinations::ListRumMetricsDestinationsError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_rum_metrics_destinations::ListRumMetricsDestinationsError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::list_rum_metrics_destinations::ListRumMetricsDestinationsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_rum_metrics_destinations::ListRumMetricsDestinationsError::Unhandled(inner) => Error::Unhandled(inner),
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
            crate::operation::list_tags_for_resource::ListTagsForResourceError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_tags_for_resource::ListTagsForResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<crate::operation::put_rum_events::PutRumEventsError, R>,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::put_rum_events::PutRumEventsError,
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
impl From<crate::operation::put_rum_events::PutRumEventsError> for Error {
    fn from(err: crate::operation::put_rum_events::PutRumEventsError) -> Self {
        match err {
            crate::operation::put_rum_events::PutRumEventsError::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::operation::put_rum_events::PutRumEventsError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::put_rum_events::PutRumEventsError::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::operation::put_rum_events::PutRumEventsError::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::operation::put_rum_events::PutRumEventsError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::put_rum_events::PutRumEventsError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_rum_metrics_destination::PutRumMetricsDestinationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::put_rum_metrics_destination::PutRumMetricsDestinationError,
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
impl From<crate::operation::put_rum_metrics_destination::PutRumMetricsDestinationError> for Error {
    fn from(
        err: crate::operation::put_rum_metrics_destination::PutRumMetricsDestinationError,
    ) -> Self {
        match err {
            crate::operation::put_rum_metrics_destination::PutRumMetricsDestinationError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::put_rum_metrics_destination::PutRumMetricsDestinationError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::put_rum_metrics_destination::PutRumMetricsDestinationError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::put_rum_metrics_destination::PutRumMetricsDestinationError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::put_rum_metrics_destination::PutRumMetricsDestinationError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::put_rum_metrics_destination::PutRumMetricsDestinationError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::put_rum_metrics_destination::PutRumMetricsDestinationError::Unhandled(inner) => Error::Unhandled(inner),
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
            crate::operation::tag_resource::TagResourceError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::tag_resource::TagResourceError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::tag_resource::TagResourceError::ValidationException(inner) => {
                Error::ValidationException(inner)
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
            crate::operation::untag_resource::UntagResourceError::InternalServerException(
                inner,
            ) => Error::InternalServerException(inner),
            crate::operation::untag_resource::UntagResourceError::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::operation::untag_resource::UntagResourceError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::untag_resource::UntagResourceError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_app_monitor::UpdateAppMonitorError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::update_app_monitor::UpdateAppMonitorError,
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
impl From<crate::operation::update_app_monitor::UpdateAppMonitorError> for Error {
    fn from(err: crate::operation::update_app_monitor::UpdateAppMonitorError) -> Self {
        match err {
            crate::operation::update_app_monitor::UpdateAppMonitorError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::update_app_monitor::UpdateAppMonitorError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::update_app_monitor::UpdateAppMonitorError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::update_app_monitor::UpdateAppMonitorError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::update_app_monitor::UpdateAppMonitorError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::update_app_monitor::UpdateAppMonitorError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::update_app_monitor::UpdateAppMonitorError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_rum_metric_definition::UpdateRumMetricDefinitionError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::update_rum_metric_definition::UpdateRumMetricDefinitionError,
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
impl From<crate::operation::update_rum_metric_definition::UpdateRumMetricDefinitionError>
    for Error
{
    fn from(
        err: crate::operation::update_rum_metric_definition::UpdateRumMetricDefinitionError,
    ) -> Self {
        match err {
            crate::operation::update_rum_metric_definition::UpdateRumMetricDefinitionError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::update_rum_metric_definition::UpdateRumMetricDefinitionError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::update_rum_metric_definition::UpdateRumMetricDefinitionError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::update_rum_metric_definition::UpdateRumMetricDefinitionError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::update_rum_metric_definition::UpdateRumMetricDefinitionError::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::operation::update_rum_metric_definition::UpdateRumMetricDefinitionError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::update_rum_metric_definition::UpdateRumMetricDefinitionError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::update_rum_metric_definition::UpdateRumMetricDefinitionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::AccessDeniedException(inner) => inner.source(),
            Error::ConflictException(inner) => inner.source(),
            Error::InternalServerException(inner) => inner.source(),
            Error::ResourceNotFoundException(inner) => inner.source(),
            Error::ServiceQuotaExceededException(inner) => inner.source(),
            Error::ThrottlingException(inner) => inner.source(),
            Error::ValidationException(inner) => inner.source(),
            Error::Unhandled(inner) => inner.source(),
        }
    }
}
impl ::aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::AccessDeniedException(e) => e.request_id(),
            Self::ConflictException(e) => e.request_id(),
            Self::InternalServerException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::ServiceQuotaExceededException(e) => e.request_id(),
            Self::ThrottlingException(e) => e.request_id(),
            Self::ValidationException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}
