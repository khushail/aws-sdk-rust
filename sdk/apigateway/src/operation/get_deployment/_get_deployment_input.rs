// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Requests API Gateway to get information about a Deployment resource.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetDeploymentInput {
    /// <p>The string identifier of the associated RestApi.</p>
    #[doc(hidden)]
    pub rest_api_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the Deployment resource to get information about.</p>
    #[doc(hidden)]
    pub deployment_id: ::std::option::Option<::std::string::String>,
    /// <p>A query parameter to retrieve the specified embedded resources of the returned Deployment resource in the response. In a REST API call, this <code>embed</code> parameter value is a list of comma-separated strings, as in <code>GET /restapis/{restapi_id}/deployments/{deployment_id}?embed=var1,var2</code>. The SDK and other platform-dependent libraries might use a different format for the list. Currently, this request supports only retrieval of the embedded API summary this way. Hence, the parameter value must be a single-valued list containing only the <code>"apisummary"</code> string. For example, <code>GET /restapis/{restapi_id}/deployments/{deployment_id}?embed=apisummary</code>.</p>
    #[doc(hidden)]
    pub embed: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl GetDeploymentInput {
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn rest_api_id(&self) -> ::std::option::Option<&str> {
        self.rest_api_id.as_deref()
    }
    /// <p>The identifier of the Deployment resource to get information about.</p>
    pub fn deployment_id(&self) -> ::std::option::Option<&str> {
        self.deployment_id.as_deref()
    }
    /// <p>A query parameter to retrieve the specified embedded resources of the returned Deployment resource in the response. In a REST API call, this <code>embed</code> parameter value is a list of comma-separated strings, as in <code>GET /restapis/{restapi_id}/deployments/{deployment_id}?embed=var1,var2</code>. The SDK and other platform-dependent libraries might use a different format for the list. Currently, this request supports only retrieval of the embedded API summary this way. Hence, the parameter value must be a single-valued list containing only the <code>"apisummary"</code> string. For example, <code>GET /restapis/{restapi_id}/deployments/{deployment_id}?embed=apisummary</code>.</p>
    pub fn embed(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.embed.as_deref()
    }
}
impl GetDeploymentInput {
    /// Creates a new builder-style object to manufacture [`GetDeploymentInput`](crate::operation::get_deployment::GetDeploymentInput).
    pub fn builder() -> crate::operation::get_deployment::builders::GetDeploymentInputBuilder {
        crate::operation::get_deployment::builders::GetDeploymentInputBuilder::default()
    }
}

/// A builder for [`GetDeploymentInput`](crate::operation::get_deployment::GetDeploymentInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetDeploymentInputBuilder {
    pub(crate) rest_api_id: ::std::option::Option<::std::string::String>,
    pub(crate) deployment_id: ::std::option::Option<::std::string::String>,
    pub(crate) embed: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl GetDeploymentInputBuilder {
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn rest_api_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.rest_api_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn set_rest_api_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.rest_api_id = input;
        self
    }
    /// <p>The identifier of the Deployment resource to get information about.</p>
    pub fn deployment_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.deployment_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the Deployment resource to get information about.</p>
    pub fn set_deployment_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.deployment_id = input;
        self
    }
    /// Appends an item to `embed`.
    ///
    /// To override the contents of this collection use [`set_embed`](Self::set_embed).
    ///
    /// <p>A query parameter to retrieve the specified embedded resources of the returned Deployment resource in the response. In a REST API call, this <code>embed</code> parameter value is a list of comma-separated strings, as in <code>GET /restapis/{restapi_id}/deployments/{deployment_id}?embed=var1,var2</code>. The SDK and other platform-dependent libraries might use a different format for the list. Currently, this request supports only retrieval of the embedded API summary this way. Hence, the parameter value must be a single-valued list containing only the <code>"apisummary"</code> string. For example, <code>GET /restapis/{restapi_id}/deployments/{deployment_id}?embed=apisummary</code>.</p>
    pub fn embed(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.embed.unwrap_or_default();
        v.push(input.into());
        self.embed = ::std::option::Option::Some(v);
        self
    }
    /// <p>A query parameter to retrieve the specified embedded resources of the returned Deployment resource in the response. In a REST API call, this <code>embed</code> parameter value is a list of comma-separated strings, as in <code>GET /restapis/{restapi_id}/deployments/{deployment_id}?embed=var1,var2</code>. The SDK and other platform-dependent libraries might use a different format for the list. Currently, this request supports only retrieval of the embedded API summary this way. Hence, the parameter value must be a single-valued list containing only the <code>"apisummary"</code> string. For example, <code>GET /restapis/{restapi_id}/deployments/{deployment_id}?embed=apisummary</code>.</p>
    pub fn set_embed(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.embed = input;
        self
    }
    /// Consumes the builder and constructs a [`GetDeploymentInput`](crate::operation::get_deployment::GetDeploymentInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_deployment::GetDeploymentInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_deployment::GetDeploymentInput {
            rest_api_id: self.rest_api_id,
            deployment_id: self.deployment_id,
            embed: self.embed,
        })
    }
}
