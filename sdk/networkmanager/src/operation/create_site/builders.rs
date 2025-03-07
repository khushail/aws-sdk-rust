// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_site::_create_site_output::CreateSiteOutputBuilder;

pub use crate::operation::create_site::_create_site_input::CreateSiteInputBuilder;

/// Fluent builder constructing a request to `CreateSite`.
///
/// <p>Creates a new site in a global network.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateSiteFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_site::builders::CreateSiteInputBuilder,
}
impl CreateSiteFluentBuilder {
    /// Creates a new `CreateSite`.
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
            crate::operation::create_site::CreateSite,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_site::CreateSiteError>,
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
        crate::operation::create_site::CreateSiteOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_site::CreateSiteError>,
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
        crate::operation::create_site::CreateSiteOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_site::CreateSiteError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_site::CreateSite,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_site::CreateSiteError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the global network.</p>
    pub fn global_network_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.global_network_id(input.into());
        self
    }
    /// <p>The ID of the global network.</p>
    pub fn set_global_network_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_global_network_id(input);
        self
    }
    /// <p>A description of your site.</p>
    /// <p>Constraints: Maximum length of 256 characters.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of your site.</p>
    /// <p>Constraints: Maximum length of 256 characters.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The site location. This information is used for visualization in the Network Manager console. If you specify the address, the latitude and longitude are automatically calculated.</p>
    /// <ul>
    /// <li> <p> <code>Address</code>: The physical address of the site.</p> </li>
    /// <li> <p> <code>Latitude</code>: The latitude of the site. </p> </li>
    /// <li> <p> <code>Longitude</code>: The longitude of the site.</p> </li>
    /// </ul>
    pub fn location(mut self, input: crate::types::Location) -> Self {
        self.inner = self.inner.location(input);
        self
    }
    /// <p>The site location. This information is used for visualization in the Network Manager console. If you specify the address, the latitude and longitude are automatically calculated.</p>
    /// <ul>
    /// <li> <p> <code>Address</code>: The physical address of the site.</p> </li>
    /// <li> <p> <code>Latitude</code>: The latitude of the site. </p> </li>
    /// <li> <p> <code>Longitude</code>: The longitude of the site.</p> </li>
    /// </ul>
    pub fn set_location(mut self, input: ::std::option::Option<crate::types::Location>) -> Self {
        self.inner = self.inner.set_location(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to apply to the resource during creation.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags to apply to the resource during creation.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
