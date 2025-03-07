// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_sol_function_package::_update_sol_function_package_output::UpdateSolFunctionPackageOutputBuilder;

pub use crate::operation::update_sol_function_package::_update_sol_function_package_input::UpdateSolFunctionPackageInputBuilder;

/// Fluent builder constructing a request to `UpdateSolFunctionPackage`.
///
/// <p>Updates the operational state of function package.</p>
/// <p>A function package is a .zip file in CSAR (Cloud Service Archive) format that contains a network function (an ETSI standard telecommunication application) and function package descriptor that uses the TOSCA standard to describe how the network functions should run on your network.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateSolFunctionPackageFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_sol_function_package::builders::UpdateSolFunctionPackageInputBuilder,
}
impl UpdateSolFunctionPackageFluentBuilder {
    /// Creates a new `UpdateSolFunctionPackage`.
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
            crate::operation::update_sol_function_package::UpdateSolFunctionPackage,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_sol_function_package::UpdateSolFunctionPackageError,
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
        crate::operation::update_sol_function_package::UpdateSolFunctionPackageOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_sol_function_package::UpdateSolFunctionPackageError,
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
        crate::operation::update_sol_function_package::UpdateSolFunctionPackageOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_sol_function_package::UpdateSolFunctionPackageError,
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
            crate::operation::update_sol_function_package::UpdateSolFunctionPackage,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_sol_function_package::UpdateSolFunctionPackageError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>ID of the function package.</p>
    pub fn vnf_pkg_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vnf_pkg_id(input.into());
        self
    }
    /// <p>ID of the function package.</p>
    pub fn set_vnf_pkg_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vnf_pkg_id(input);
        self
    }
    /// <p>Operational state of the function package.</p>
    pub fn operational_state(mut self, input: crate::types::OperationalState) -> Self {
        self.inner = self.inner.operational_state(input);
        self
    }
    /// <p>Operational state of the function package.</p>
    pub fn set_operational_state(
        mut self,
        input: ::std::option::Option<crate::types::OperationalState>,
    ) -> Self {
        self.inner = self.inner.set_operational_state(input);
        self
    }
}
