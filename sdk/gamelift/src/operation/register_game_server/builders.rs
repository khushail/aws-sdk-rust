// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::register_game_server::_register_game_server_output::RegisterGameServerOutputBuilder;

pub use crate::operation::register_game_server::_register_game_server_input::RegisterGameServerInputBuilder;

/// Fluent builder constructing a request to `RegisterGameServer`.
///
/// <p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p>
/// <p>Creates a new game server resource and notifies Amazon GameLift FleetIQ that the game server is ready to host gameplay and players. This operation is called by a game server process that is running on an instance in a game server group. Registering game servers enables Amazon GameLift FleetIQ to track available game servers and enables game clients and services to claim a game server for a new game session. </p>
/// <p>To register a game server, identify the game server group and instance where the game server is running, and provide a unique identifier for the game server. You can also include connection and game server data.</p>
/// <p>Once a game server is successfully registered, it is put in status <code>AVAILABLE</code>. A request to register a game server may fail if the instance it is running on is in the process of shutting down as part of instance balancing or scale-down activity. </p>
/// <p> <b>Learn more</b> </p>
/// <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">Amazon GameLift FleetIQ Guide</a> </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RegisterGameServerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::register_game_server::builders::RegisterGameServerInputBuilder,
}
impl RegisterGameServerFluentBuilder {
    /// Creates a new `RegisterGameServer`.
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
            crate::operation::register_game_server::RegisterGameServer,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::register_game_server::RegisterGameServerError,
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
        crate::operation::register_game_server::RegisterGameServerOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::register_game_server::RegisterGameServerError,
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
        crate::operation::register_game_server::RegisterGameServerOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::register_game_server::RegisterGameServerError,
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
            crate::operation::register_game_server::RegisterGameServer,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::register_game_server::RegisterGameServerError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>A unique identifier for the game server group where the game server is running.</p>
    pub fn game_server_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.game_server_group_name(input.into());
        self
    }
    /// <p>A unique identifier for the game server group where the game server is running.</p>
    pub fn set_game_server_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_game_server_group_name(input);
        self
    }
    /// <p>A custom string that uniquely identifies the game server to register. Game server IDs are developer-defined and must be unique across all game server groups in your Amazon Web Services account.</p>
    pub fn game_server_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.game_server_id(input.into());
        self
    }
    /// <p>A custom string that uniquely identifies the game server to register. Game server IDs are developer-defined and must be unique across all game server groups in your Amazon Web Services account.</p>
    pub fn set_game_server_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_game_server_id(input);
        self
    }
    /// <p>The unique identifier for the instance where the game server is running. This ID is available in the instance metadata. EC2 instance IDs use a 17-character format, for example: <code>i-1234567890abcdef0</code>.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The unique identifier for the instance where the game server is running. This ID is available in the instance metadata. EC2 instance IDs use a 17-character format, for example: <code>i-1234567890abcdef0</code>.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>Information that is needed to make inbound client connections to the game server. This might include the IP address and port, DNS name, and other information.</p>
    pub fn connection_info(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.connection_info(input.into());
        self
    }
    /// <p>Information that is needed to make inbound client connections to the game server. This might include the IP address and port, DNS name, and other information.</p>
    pub fn set_connection_info(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_connection_info(input);
        self
    }
    /// <p>A set of custom game server properties, formatted as a single string value. This data is passed to a game client or service when it requests information on game servers. </p>
    pub fn game_server_data(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.game_server_data(input.into());
        self
    }
    /// <p>A set of custom game server properties, formatted as a single string value. This data is passed to a game client or service when it requests information on game servers. </p>
    pub fn set_game_server_data(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_game_server_data(input);
        self
    }
}
