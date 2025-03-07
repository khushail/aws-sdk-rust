// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_import::_start_import_output::StartImportOutputBuilder;

pub use crate::operation::start_import::_start_import_input::StartImportInputBuilder;

/// Fluent builder constructing a request to `StartImport`.
///
/// <p>Starts a job to import a resource to Amazon Lex.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartImportFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_import::builders::StartImportInputBuilder,
}
impl StartImportFluentBuilder {
    /// Creates a new `StartImport`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::start_import::StartImport,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::start_import::StartImportError>,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_import::StartImportOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::start_import::StartImportError>,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_import::StartImportOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::start_import::StartImportError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::start_import::StartImport,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::start_import::StartImportError>,
    > {
        self.customize_middleware().await
    }
    /// <p>A zip archive in binary format. The archive should contain one file, a JSON file containing the resource to import. The resource should match the type specified in the <code>resourceType</code> field.</p>
    pub fn payload(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.payload(input);
        self
    }
    /// <p>A zip archive in binary format. The archive should contain one file, a JSON file containing the resource to import. The resource should match the type specified in the <code>resourceType</code> field.</p>
    pub fn set_payload(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_payload(input);
        self
    }
    /// <p>Specifies the type of resource to export. Each resource also exports any resources that it depends on. </p>
    /// <ul>
    /// <li> <p>A bot exports dependent intents.</p> </li>
    /// <li> <p>An intent exports dependent slot types.</p> </li>
    /// </ul>
    pub fn resource_type(mut self, input: crate::types::ResourceType) -> Self {
        self.inner = self.inner.resource_type(input);
        self
    }
    /// <p>Specifies the type of resource to export. Each resource also exports any resources that it depends on. </p>
    /// <ul>
    /// <li> <p>A bot exports dependent intents.</p> </li>
    /// <li> <p>An intent exports dependent slot types.</p> </li>
    /// </ul>
    pub fn set_resource_type(
        mut self,
        input: ::std::option::Option<crate::types::ResourceType>,
    ) -> Self {
        self.inner = self.inner.set_resource_type(input);
        self
    }
    /// <p>Specifies the action that the <code>StartImport</code> operation should take when there is an existing resource with the same name.</p>
    /// <ul>
    /// <li> <p>FAIL_ON_CONFLICT - The import operation is stopped on the first conflict between a resource in the import file and an existing resource. The name of the resource causing the conflict is in the <code>failureReason</code> field of the response to the <code>GetImport</code> operation.</p> <p>OVERWRITE_LATEST - The import operation proceeds even if there is a conflict with an existing resource. The $LASTEST version of the existing resource is overwritten with the data from the import file.</p> </li>
    /// </ul>
    pub fn merge_strategy(mut self, input: crate::types::MergeStrategy) -> Self {
        self.inner = self.inner.merge_strategy(input);
        self
    }
    /// <p>Specifies the action that the <code>StartImport</code> operation should take when there is an existing resource with the same name.</p>
    /// <ul>
    /// <li> <p>FAIL_ON_CONFLICT - The import operation is stopped on the first conflict between a resource in the import file and an existing resource. The name of the resource causing the conflict is in the <code>failureReason</code> field of the response to the <code>GetImport</code> operation.</p> <p>OVERWRITE_LATEST - The import operation proceeds even if there is a conflict with an existing resource. The $LASTEST version of the existing resource is overwritten with the data from the import file.</p> </li>
    /// </ul>
    pub fn set_merge_strategy(
        mut self,
        input: ::std::option::Option<crate::types::MergeStrategy>,
    ) -> Self {
        self.inner = self.inner.set_merge_strategy(input);
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of tags to add to the imported bot. You can only add tags when you import a bot, you can't add tags to an intent or slot type.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of tags to add to the imported bot. You can only add tags when you import a bot, you can't add tags to an intent or slot type.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
