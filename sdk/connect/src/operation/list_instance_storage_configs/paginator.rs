// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`ListInstanceStorageConfigs`](crate::operation::list_instance_storage_configs::ListInstanceStorageConfigs)
pub struct ListInstanceStorageConfigsPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::operation::list_instance_storage_configs::builders::ListInstanceStorageConfigsInputBuilder,
                stop_on_duplicate_token: bool,
            }

impl ListInstanceStorageConfigsPaginator {
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle>,
        builder: crate::operation::list_instance_storage_configs::builders::ListInstanceStorageConfigsInputBuilder,
    ) -> Self {
        Self {
            handle,
            builder,
            stop_on_duplicate_token: true,
        }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = ::std::option::Option::Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `storage_configs`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::operation::list_instance_storage_configs::paginator::ListInstanceStorageConfigsPaginatorItems{
        crate::operation::list_instance_storage_configs::paginator::ListInstanceStorageConfigsPaginatorItems(self)
    }

    /// Stop paginating when the service returns the same pagination token twice in a row.
    ///
    /// Defaults to true.
    ///
    /// For certain operations, it may be useful to continue on duplicate token. For example,
    /// if an operation is for tailing a log file in real-time, then continuing may be desired.
    /// This option can be set to `false` to accommodate these use cases.
    pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
        self.stop_on_duplicate_token = stop_on_duplicate_token;
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl ::tokio_stream::Stream<
        Item = ::std::result::Result<
            crate::operation::list_instance_storage_configs::ListInstanceStorageConfigsOutput,
            ::aws_smithy_http::result::SdkError<
                crate::operation::list_instance_storage_configs::ListInstanceStorageConfigsError,
            >,
        >,
    > + ::std::marker::Unpin {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        ::aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            ::std::boxed::Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder
                    .build()
                    .map_err(::aws_smithy_http::result::SdkError::construction_failure)
                {
                    ::std::result::Result::Ok(input) => input,
                    ::std::result::Result::Err(e) => {
                        let _ = tx.send(::std::result::Result::Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input
                        .make_operation(&handle.conf)
                        .await
                        .map_err(::aws_smithy_http::result::SdkError::construction_failure)
                    {
                        ::std::result::Result::Ok(op) => op,
                        ::std::result::Result::Err(e) => {
                            let _ = tx.send(::std::result::Result::Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        ::std::result::Result::Ok(ref resp) => {
                            let new_token = crate::lens::reflens_list_instance_storage_configs_output_next_token(resp);
                            let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                            if !is_empty
                                && new_token == input.next_token.as_ref()
                                && self.stop_on_duplicate_token
                            {
                                true
                            } else {
                                input.next_token = new_token.cloned();
                                is_empty
                            }
                        }
                        ::std::result::Result::Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Flattened paginator for `ListInstanceStorageConfigsPaginator`
///
/// This is created with [`.items()`](ListInstanceStorageConfigsPaginator::items)
pub struct ListInstanceStorageConfigsPaginatorItems(ListInstanceStorageConfigsPaginator);

impl ListInstanceStorageConfigsPaginatorItems {
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl ::tokio_stream::Stream<
        Item = ::std::result::Result<
            crate::types::InstanceStorageConfig,
            ::aws_smithy_http::result::SdkError<
                crate::operation::list_instance_storage_configs::ListInstanceStorageConfigsError,
            >,
        >,
    > + ::std::marker::Unpin {
        ::aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
            crate::lens::lens_list_instance_storage_configs_output_storage_configs(page)
                .unwrap_or_default()
                .into_iter()
        })
    }
}
