// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_entitlements::_get_entitlements_output::GetEntitlementsOutputBuilder;

pub use crate::operation::get_entitlements::_get_entitlements_input::GetEntitlementsInputBuilder;

/// Fluent builder constructing a request to `GetEntitlements`.
///
/// <p>GetEntitlements retrieves entitlement values for a given product. The results can be filtered based on customer identifier or product dimensions.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetEntitlementsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_entitlements::builders::GetEntitlementsInputBuilder,
}
impl GetEntitlementsFluentBuilder {
    /// Creates a new `GetEntitlements`.
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
            crate::operation::get_entitlements::GetEntitlements,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_entitlements::GetEntitlementsError,
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
        crate::operation::get_entitlements::GetEntitlementsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_entitlements::GetEntitlementsError,
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
        crate::operation::get_entitlements::GetEntitlementsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_entitlements::GetEntitlementsError,
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
            crate::operation::get_entitlements::GetEntitlements,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_entitlements::GetEntitlementsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code will be provided by AWS Marketplace when the product listing is created.</p>
    pub fn product_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.product_code(input.into());
        self
    }
    /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code will be provided by AWS Marketplace when the product listing is created.</p>
    pub fn set_product_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_product_code(input);
        self
    }
    /// Adds a key-value pair to `Filter`.
    ///
    /// To override the contents of this collection use [`set_filter`](Self::set_filter).
    ///
    /// <p>Filter is used to return entitlements for a specific customer or for a specific dimension. Filters are described as keys mapped to a lists of values. Filtered requests are <i>unioned</i> for each value in the value list, and then <i>intersected</i> for each filter key.</p>
    pub fn filter(
        mut self,
        k: crate::types::GetEntitlementFilterName,
        v: ::std::vec::Vec<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.filter(k, v);
        self
    }
    /// <p>Filter is used to return entitlements for a specific customer or for a specific dimension. Filters are described as keys mapped to a lists of values. Filtered requests are <i>unioned</i> for each value in the value list, and then <i>intersected</i> for each filter key.</p>
    pub fn set_filter(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<
                crate::types::GetEntitlementFilterName,
                ::std::vec::Vec<::std::string::String>,
            >,
        >,
    ) -> Self {
        self.inner = self.inner.set_filter(input);
        self
    }
    /// <p>For paginated calls to GetEntitlements, pass the NextToken from the previous GetEntitlementsResult.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>For paginated calls to GetEntitlements, pass the NextToken from the previous GetEntitlementsResult.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of items to retrieve from the GetEntitlements operation. For pagination, use the NextToken field in subsequent calls to GetEntitlements.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to retrieve from the GetEntitlements operation. For pagination, use the NextToken field in subsequent calls to GetEntitlements.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
