// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_managed_prefix_list::_create_managed_prefix_list_output::CreateManagedPrefixListOutputBuilder;

pub use crate::operation::create_managed_prefix_list::_create_managed_prefix_list_input::CreateManagedPrefixListInputBuilder;

/// Fluent builder constructing a request to `CreateManagedPrefixList`.
///
/// <p>Creates a managed prefix list. You can specify one or more entries for the prefix list. Each entry consists of a CIDR block and an optional description.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateManagedPrefixListFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::create_managed_prefix_list::builders::CreateManagedPrefixListInputBuilder,
}
impl CreateManagedPrefixListFluentBuilder {
    /// Creates a new `CreateManagedPrefixList`.
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
            crate::operation::create_managed_prefix_list::CreateManagedPrefixList,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_managed_prefix_list::CreateManagedPrefixListError,
        >,
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
        crate::operation::create_managed_prefix_list::CreateManagedPrefixListOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_managed_prefix_list::CreateManagedPrefixListError,
        >,
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
        crate::operation::create_managed_prefix_list::CreateManagedPrefixListOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_managed_prefix_list::CreateManagedPrefixListError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_managed_prefix_list::CreateManagedPrefixList,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_managed_prefix_list::CreateManagedPrefixListError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>A name for the prefix list.</p>
    /// <p>Constraints: Up to 255 characters in length. The name cannot start with <code>com.amazonaws</code>.</p>
    pub fn prefix_list_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.prefix_list_name(input.into());
        self
    }
    /// <p>A name for the prefix list.</p>
    /// <p>Constraints: Up to 255 characters in length. The name cannot start with <code>com.amazonaws</code>.</p>
    pub fn set_prefix_list_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_prefix_list_name(input);
        self
    }
    /// Appends an item to `Entries`.
    ///
    /// To override the contents of this collection use [`set_entries`](Self::set_entries).
    ///
    /// <p>One or more entries for the prefix list.</p>
    pub fn entries(mut self, input: crate::types::AddPrefixListEntry) -> Self {
        self.inner = self.inner.entries(input);
        self
    }
    /// <p>One or more entries for the prefix list.</p>
    pub fn set_entries(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AddPrefixListEntry>>,
    ) -> Self {
        self.inner = self.inner.set_entries(input);
        self
    }
    /// <p>The maximum number of entries for the prefix list.</p>
    pub fn max_entries(mut self, input: i32) -> Self {
        self.inner = self.inner.max_entries(input);
        self
    }
    /// <p>The maximum number of entries for the prefix list.</p>
    pub fn set_max_entries(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_entries(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the prefix list during creation.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the prefix list during creation.</p>
    pub fn set_tag_specifications(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>The IP address type.</p>
    /// <p>Valid Values: <code>IPv4</code> | <code>IPv6</code> </p>
    pub fn address_family(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.address_family(input.into());
        self
    }
    /// <p>The IP address type.</p>
    /// <p>Valid Values: <code>IPv4</code> | <code>IPv6</code> </p>
    pub fn set_address_family(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_address_family(input);
        self
    }
    /// <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    /// <p>Constraints: Up to 255 UTF-8 characters in length.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    /// <p>Constraints: Up to 255 UTF-8 characters in length.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}
