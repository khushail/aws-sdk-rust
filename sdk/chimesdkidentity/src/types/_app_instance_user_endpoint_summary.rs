// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Summary of the details of an <code>AppInstanceUserEndpoint</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct AppInstanceUserEndpointSummary {
    /// <p>The ARN of the <code>AppInstanceUser</code>.</p>
    #[doc(hidden)]
    pub app_instance_user_arn: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier of the <code>AppInstanceUserEndpoint</code>.</p>
    #[doc(hidden)]
    pub endpoint_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the <code>AppInstanceUserEndpoint</code>.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The type of the <code>AppInstanceUserEndpoint</code>.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::AppInstanceUserEndpointType>,
    /// <p>BBoolean that controls whether the <code>AppInstanceUserEndpoint</code> is opted in to receive messages. <code>ALL</code> indicates the endpoint will receive all messages. <code>NONE</code> indicates the endpoint will receive no messages.</p>
    #[doc(hidden)]
    pub allow_messages: ::std::option::Option<crate::types::AllowMessages>,
    /// <p>A read-only field that represent the state of an <code>AppInstanceUserEndpoint</code>.</p>
    #[doc(hidden)]
    pub endpoint_state: ::std::option::Option<crate::types::EndpointState>,
}
impl AppInstanceUserEndpointSummary {
    /// <p>The ARN of the <code>AppInstanceUser</code>.</p>
    pub fn app_instance_user_arn(&self) -> ::std::option::Option<&str> {
        self.app_instance_user_arn.as_deref()
    }
    /// <p>The unique identifier of the <code>AppInstanceUserEndpoint</code>.</p>
    pub fn endpoint_id(&self) -> ::std::option::Option<&str> {
        self.endpoint_id.as_deref()
    }
    /// <p>The name of the <code>AppInstanceUserEndpoint</code>.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The type of the <code>AppInstanceUserEndpoint</code>.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::AppInstanceUserEndpointType> {
        self.r#type.as_ref()
    }
    /// <p>BBoolean that controls whether the <code>AppInstanceUserEndpoint</code> is opted in to receive messages. <code>ALL</code> indicates the endpoint will receive all messages. <code>NONE</code> indicates the endpoint will receive no messages.</p>
    pub fn allow_messages(&self) -> ::std::option::Option<&crate::types::AllowMessages> {
        self.allow_messages.as_ref()
    }
    /// <p>A read-only field that represent the state of an <code>AppInstanceUserEndpoint</code>.</p>
    pub fn endpoint_state(&self) -> ::std::option::Option<&crate::types::EndpointState> {
        self.endpoint_state.as_ref()
    }
}
impl ::std::fmt::Debug for AppInstanceUserEndpointSummary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AppInstanceUserEndpointSummary");
        formatter.field("app_instance_user_arn", &"*** Sensitive Data Redacted ***");
        formatter.field("endpoint_id", &"*** Sensitive Data Redacted ***");
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("r#type", &self.r#type);
        formatter.field("allow_messages", &self.allow_messages);
        formatter.field("endpoint_state", &self.endpoint_state);
        formatter.finish()
    }
}
impl AppInstanceUserEndpointSummary {
    /// Creates a new builder-style object to manufacture [`AppInstanceUserEndpointSummary`](crate::types::AppInstanceUserEndpointSummary).
    pub fn builder() -> crate::types::builders::AppInstanceUserEndpointSummaryBuilder {
        crate::types::builders::AppInstanceUserEndpointSummaryBuilder::default()
    }
}

/// A builder for [`AppInstanceUserEndpointSummary`](crate::types::AppInstanceUserEndpointSummary).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct AppInstanceUserEndpointSummaryBuilder {
    pub(crate) app_instance_user_arn: ::std::option::Option<::std::string::String>,
    pub(crate) endpoint_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<crate::types::AppInstanceUserEndpointType>,
    pub(crate) allow_messages: ::std::option::Option<crate::types::AllowMessages>,
    pub(crate) endpoint_state: ::std::option::Option<crate::types::EndpointState>,
}
impl AppInstanceUserEndpointSummaryBuilder {
    /// <p>The ARN of the <code>AppInstanceUser</code>.</p>
    pub fn app_instance_user_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.app_instance_user_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the <code>AppInstanceUser</code>.</p>
    pub fn set_app_instance_user_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.app_instance_user_arn = input;
        self
    }
    /// <p>The unique identifier of the <code>AppInstanceUserEndpoint</code>.</p>
    pub fn endpoint_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.endpoint_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the <code>AppInstanceUserEndpoint</code>.</p>
    pub fn set_endpoint_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.endpoint_id = input;
        self
    }
    /// <p>The name of the <code>AppInstanceUserEndpoint</code>.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the <code>AppInstanceUserEndpoint</code>.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The type of the <code>AppInstanceUserEndpoint</code>.</p>
    pub fn r#type(mut self, input: crate::types::AppInstanceUserEndpointType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of the <code>AppInstanceUserEndpoint</code>.</p>
    pub fn set_type(
        mut self,
        input: ::std::option::Option<crate::types::AppInstanceUserEndpointType>,
    ) -> Self {
        self.r#type = input;
        self
    }
    /// <p>BBoolean that controls whether the <code>AppInstanceUserEndpoint</code> is opted in to receive messages. <code>ALL</code> indicates the endpoint will receive all messages. <code>NONE</code> indicates the endpoint will receive no messages.</p>
    pub fn allow_messages(mut self, input: crate::types::AllowMessages) -> Self {
        self.allow_messages = ::std::option::Option::Some(input);
        self
    }
    /// <p>BBoolean that controls whether the <code>AppInstanceUserEndpoint</code> is opted in to receive messages. <code>ALL</code> indicates the endpoint will receive all messages. <code>NONE</code> indicates the endpoint will receive no messages.</p>
    pub fn set_allow_messages(
        mut self,
        input: ::std::option::Option<crate::types::AllowMessages>,
    ) -> Self {
        self.allow_messages = input;
        self
    }
    /// <p>A read-only field that represent the state of an <code>AppInstanceUserEndpoint</code>.</p>
    pub fn endpoint_state(mut self, input: crate::types::EndpointState) -> Self {
        self.endpoint_state = ::std::option::Option::Some(input);
        self
    }
    /// <p>A read-only field that represent the state of an <code>AppInstanceUserEndpoint</code>.</p>
    pub fn set_endpoint_state(
        mut self,
        input: ::std::option::Option<crate::types::EndpointState>,
    ) -> Self {
        self.endpoint_state = input;
        self
    }
    /// Consumes the builder and constructs a [`AppInstanceUserEndpointSummary`](crate::types::AppInstanceUserEndpointSummary).
    pub fn build(self) -> crate::types::AppInstanceUserEndpointSummary {
        crate::types::AppInstanceUserEndpointSummary {
            app_instance_user_arn: self.app_instance_user_arn,
            endpoint_id: self.endpoint_id,
            name: self.name,
            r#type: self.r#type,
            allow_messages: self.allow_messages,
            endpoint_state: self.endpoint_state,
        }
    }
}
impl ::std::fmt::Debug for AppInstanceUserEndpointSummaryBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AppInstanceUserEndpointSummaryBuilder");
        formatter.field("app_instance_user_arn", &"*** Sensitive Data Redacted ***");
        formatter.field("endpoint_id", &"*** Sensitive Data Redacted ***");
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("r#type", &self.r#type);
        formatter.field("allow_messages", &self.allow_messages);
        formatter.field("endpoint_state", &self.endpoint_state);
        formatter.finish()
    }
}
