// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the method setting properties.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MethodSetting {
    /// <p>Specifies whether Amazon CloudWatch metrics are enabled for this method. The PATCH path for this setting is <code>/{method_setting_key}/metrics/enabled</code>, and the value is a Boolean.</p>
    #[doc(hidden)]
    pub metrics_enabled: bool,
    /// <p>Specifies the logging level for this method, which affects the log entries pushed to Amazon CloudWatch Logs. The PATCH path for this setting is <code>/{method_setting_key}/logging/loglevel</code>, and the available levels are <code>OFF</code>, <code>ERROR</code>, and <code>INFO</code>. Choose <code>ERROR</code> to write only error-level entries to CloudWatch Logs, or choose <code>INFO</code> to include all <code>ERROR</code> events as well as extra informational events.</p>
    #[doc(hidden)]
    pub logging_level: ::std::option::Option<::std::string::String>,
    /// <p>Specifies whether data trace logging is enabled for this method, which affects the log entries pushed to Amazon CloudWatch Logs. The PATCH path for this setting is <code>/{method_setting_key}/logging/dataTrace</code>, and the value is a Boolean.</p>
    #[doc(hidden)]
    pub data_trace_enabled: bool,
    /// <p>Specifies the throttling burst limit. The PATCH path for this setting is <code>/{method_setting_key}/throttling/burstLimit</code>, and the value is an integer.</p>
    #[doc(hidden)]
    pub throttling_burst_limit: i32,
    /// <p>Specifies the throttling rate limit. The PATCH path for this setting is <code>/{method_setting_key}/throttling/rateLimit</code>, and the value is a double.</p>
    #[doc(hidden)]
    pub throttling_rate_limit: f64,
    /// <p>Specifies whether responses should be cached and returned for requests. A cache cluster must be enabled on the stage for responses to be cached. The PATCH path for this setting is <code>/{method_setting_key}/caching/enabled</code>, and the value is a Boolean.</p>
    #[doc(hidden)]
    pub caching_enabled: bool,
    /// <p>Specifies the time to live (TTL), in seconds, for cached responses. The higher the TTL, the longer the response will be cached. The PATCH path for this setting is <code>/{method_setting_key}/caching/ttlInSeconds</code>, and the value is an integer.</p>
    #[doc(hidden)]
    pub cache_ttl_in_seconds: i32,
    /// <p>Specifies whether the cached responses are encrypted. The PATCH path for this setting is <code>/{method_setting_key}/caching/dataEncrypted</code>, and the value is a Boolean.</p>
    #[doc(hidden)]
    pub cache_data_encrypted: bool,
    /// <p>Specifies whether authorization is required for a cache invalidation request. The PATCH path for this setting is <code>/{method_setting_key}/caching/requireAuthorizationForCacheControl</code>, and the value is a Boolean.</p>
    #[doc(hidden)]
    pub require_authorization_for_cache_control: bool,
    /// <p>Specifies how to handle unauthorized requests for cache invalidation. The PATCH path for this setting is <code>/{method_setting_key}/caching/unauthorizedCacheControlHeaderStrategy</code>, and the available values are <code>FAIL_WITH_403</code>, <code>SUCCEED_WITH_RESPONSE_HEADER</code>, <code>SUCCEED_WITHOUT_RESPONSE_HEADER</code>.</p>
    #[doc(hidden)]
    pub unauthorized_cache_control_header_strategy:
        ::std::option::Option<crate::types::UnauthorizedCacheControlHeaderStrategy>,
}
impl MethodSetting {
    /// <p>Specifies whether Amazon CloudWatch metrics are enabled for this method. The PATCH path for this setting is <code>/{method_setting_key}/metrics/enabled</code>, and the value is a Boolean.</p>
    pub fn metrics_enabled(&self) -> bool {
        self.metrics_enabled
    }
    /// <p>Specifies the logging level for this method, which affects the log entries pushed to Amazon CloudWatch Logs. The PATCH path for this setting is <code>/{method_setting_key}/logging/loglevel</code>, and the available levels are <code>OFF</code>, <code>ERROR</code>, and <code>INFO</code>. Choose <code>ERROR</code> to write only error-level entries to CloudWatch Logs, or choose <code>INFO</code> to include all <code>ERROR</code> events as well as extra informational events.</p>
    pub fn logging_level(&self) -> ::std::option::Option<&str> {
        self.logging_level.as_deref()
    }
    /// <p>Specifies whether data trace logging is enabled for this method, which affects the log entries pushed to Amazon CloudWatch Logs. The PATCH path for this setting is <code>/{method_setting_key}/logging/dataTrace</code>, and the value is a Boolean.</p>
    pub fn data_trace_enabled(&self) -> bool {
        self.data_trace_enabled
    }
    /// <p>Specifies the throttling burst limit. The PATCH path for this setting is <code>/{method_setting_key}/throttling/burstLimit</code>, and the value is an integer.</p>
    pub fn throttling_burst_limit(&self) -> i32 {
        self.throttling_burst_limit
    }
    /// <p>Specifies the throttling rate limit. The PATCH path for this setting is <code>/{method_setting_key}/throttling/rateLimit</code>, and the value is a double.</p>
    pub fn throttling_rate_limit(&self) -> f64 {
        self.throttling_rate_limit
    }
    /// <p>Specifies whether responses should be cached and returned for requests. A cache cluster must be enabled on the stage for responses to be cached. The PATCH path for this setting is <code>/{method_setting_key}/caching/enabled</code>, and the value is a Boolean.</p>
    pub fn caching_enabled(&self) -> bool {
        self.caching_enabled
    }
    /// <p>Specifies the time to live (TTL), in seconds, for cached responses. The higher the TTL, the longer the response will be cached. The PATCH path for this setting is <code>/{method_setting_key}/caching/ttlInSeconds</code>, and the value is an integer.</p>
    pub fn cache_ttl_in_seconds(&self) -> i32 {
        self.cache_ttl_in_seconds
    }
    /// <p>Specifies whether the cached responses are encrypted. The PATCH path for this setting is <code>/{method_setting_key}/caching/dataEncrypted</code>, and the value is a Boolean.</p>
    pub fn cache_data_encrypted(&self) -> bool {
        self.cache_data_encrypted
    }
    /// <p>Specifies whether authorization is required for a cache invalidation request. The PATCH path for this setting is <code>/{method_setting_key}/caching/requireAuthorizationForCacheControl</code>, and the value is a Boolean.</p>
    pub fn require_authorization_for_cache_control(&self) -> bool {
        self.require_authorization_for_cache_control
    }
    /// <p>Specifies how to handle unauthorized requests for cache invalidation. The PATCH path for this setting is <code>/{method_setting_key}/caching/unauthorizedCacheControlHeaderStrategy</code>, and the available values are <code>FAIL_WITH_403</code>, <code>SUCCEED_WITH_RESPONSE_HEADER</code>, <code>SUCCEED_WITHOUT_RESPONSE_HEADER</code>.</p>
    pub fn unauthorized_cache_control_header_strategy(
        &self,
    ) -> ::std::option::Option<&crate::types::UnauthorizedCacheControlHeaderStrategy> {
        self.unauthorized_cache_control_header_strategy.as_ref()
    }
}
impl MethodSetting {
    /// Creates a new builder-style object to manufacture [`MethodSetting`](crate::types::MethodSetting).
    pub fn builder() -> crate::types::builders::MethodSettingBuilder {
        crate::types::builders::MethodSettingBuilder::default()
    }
}

/// A builder for [`MethodSetting`](crate::types::MethodSetting).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MethodSettingBuilder {
    pub(crate) metrics_enabled: ::std::option::Option<bool>,
    pub(crate) logging_level: ::std::option::Option<::std::string::String>,
    pub(crate) data_trace_enabled: ::std::option::Option<bool>,
    pub(crate) throttling_burst_limit: ::std::option::Option<i32>,
    pub(crate) throttling_rate_limit: ::std::option::Option<f64>,
    pub(crate) caching_enabled: ::std::option::Option<bool>,
    pub(crate) cache_ttl_in_seconds: ::std::option::Option<i32>,
    pub(crate) cache_data_encrypted: ::std::option::Option<bool>,
    pub(crate) require_authorization_for_cache_control: ::std::option::Option<bool>,
    pub(crate) unauthorized_cache_control_header_strategy:
        ::std::option::Option<crate::types::UnauthorizedCacheControlHeaderStrategy>,
}
impl MethodSettingBuilder {
    /// <p>Specifies whether Amazon CloudWatch metrics are enabled for this method. The PATCH path for this setting is <code>/{method_setting_key}/metrics/enabled</code>, and the value is a Boolean.</p>
    pub fn metrics_enabled(mut self, input: bool) -> Self {
        self.metrics_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether Amazon CloudWatch metrics are enabled for this method. The PATCH path for this setting is <code>/{method_setting_key}/metrics/enabled</code>, and the value is a Boolean.</p>
    pub fn set_metrics_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.metrics_enabled = input;
        self
    }
    /// <p>Specifies the logging level for this method, which affects the log entries pushed to Amazon CloudWatch Logs. The PATCH path for this setting is <code>/{method_setting_key}/logging/loglevel</code>, and the available levels are <code>OFF</code>, <code>ERROR</code>, and <code>INFO</code>. Choose <code>ERROR</code> to write only error-level entries to CloudWatch Logs, or choose <code>INFO</code> to include all <code>ERROR</code> events as well as extra informational events.</p>
    pub fn logging_level(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.logging_level = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the logging level for this method, which affects the log entries pushed to Amazon CloudWatch Logs. The PATCH path for this setting is <code>/{method_setting_key}/logging/loglevel</code>, and the available levels are <code>OFF</code>, <code>ERROR</code>, and <code>INFO</code>. Choose <code>ERROR</code> to write only error-level entries to CloudWatch Logs, or choose <code>INFO</code> to include all <code>ERROR</code> events as well as extra informational events.</p>
    pub fn set_logging_level(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.logging_level = input;
        self
    }
    /// <p>Specifies whether data trace logging is enabled for this method, which affects the log entries pushed to Amazon CloudWatch Logs. The PATCH path for this setting is <code>/{method_setting_key}/logging/dataTrace</code>, and the value is a Boolean.</p>
    pub fn data_trace_enabled(mut self, input: bool) -> Self {
        self.data_trace_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether data trace logging is enabled for this method, which affects the log entries pushed to Amazon CloudWatch Logs. The PATCH path for this setting is <code>/{method_setting_key}/logging/dataTrace</code>, and the value is a Boolean.</p>
    pub fn set_data_trace_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.data_trace_enabled = input;
        self
    }
    /// <p>Specifies the throttling burst limit. The PATCH path for this setting is <code>/{method_setting_key}/throttling/burstLimit</code>, and the value is an integer.</p>
    pub fn throttling_burst_limit(mut self, input: i32) -> Self {
        self.throttling_burst_limit = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the throttling burst limit. The PATCH path for this setting is <code>/{method_setting_key}/throttling/burstLimit</code>, and the value is an integer.</p>
    pub fn set_throttling_burst_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.throttling_burst_limit = input;
        self
    }
    /// <p>Specifies the throttling rate limit. The PATCH path for this setting is <code>/{method_setting_key}/throttling/rateLimit</code>, and the value is a double.</p>
    pub fn throttling_rate_limit(mut self, input: f64) -> Self {
        self.throttling_rate_limit = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the throttling rate limit. The PATCH path for this setting is <code>/{method_setting_key}/throttling/rateLimit</code>, and the value is a double.</p>
    pub fn set_throttling_rate_limit(mut self, input: ::std::option::Option<f64>) -> Self {
        self.throttling_rate_limit = input;
        self
    }
    /// <p>Specifies whether responses should be cached and returned for requests. A cache cluster must be enabled on the stage for responses to be cached. The PATCH path for this setting is <code>/{method_setting_key}/caching/enabled</code>, and the value is a Boolean.</p>
    pub fn caching_enabled(mut self, input: bool) -> Self {
        self.caching_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether responses should be cached and returned for requests. A cache cluster must be enabled on the stage for responses to be cached. The PATCH path for this setting is <code>/{method_setting_key}/caching/enabled</code>, and the value is a Boolean.</p>
    pub fn set_caching_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.caching_enabled = input;
        self
    }
    /// <p>Specifies the time to live (TTL), in seconds, for cached responses. The higher the TTL, the longer the response will be cached. The PATCH path for this setting is <code>/{method_setting_key}/caching/ttlInSeconds</code>, and the value is an integer.</p>
    pub fn cache_ttl_in_seconds(mut self, input: i32) -> Self {
        self.cache_ttl_in_seconds = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the time to live (TTL), in seconds, for cached responses. The higher the TTL, the longer the response will be cached. The PATCH path for this setting is <code>/{method_setting_key}/caching/ttlInSeconds</code>, and the value is an integer.</p>
    pub fn set_cache_ttl_in_seconds(mut self, input: ::std::option::Option<i32>) -> Self {
        self.cache_ttl_in_seconds = input;
        self
    }
    /// <p>Specifies whether the cached responses are encrypted. The PATCH path for this setting is <code>/{method_setting_key}/caching/dataEncrypted</code>, and the value is a Boolean.</p>
    pub fn cache_data_encrypted(mut self, input: bool) -> Self {
        self.cache_data_encrypted = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether the cached responses are encrypted. The PATCH path for this setting is <code>/{method_setting_key}/caching/dataEncrypted</code>, and the value is a Boolean.</p>
    pub fn set_cache_data_encrypted(mut self, input: ::std::option::Option<bool>) -> Self {
        self.cache_data_encrypted = input;
        self
    }
    /// <p>Specifies whether authorization is required for a cache invalidation request. The PATCH path for this setting is <code>/{method_setting_key}/caching/requireAuthorizationForCacheControl</code>, and the value is a Boolean.</p>
    pub fn require_authorization_for_cache_control(mut self, input: bool) -> Self {
        self.require_authorization_for_cache_control = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether authorization is required for a cache invalidation request. The PATCH path for this setting is <code>/{method_setting_key}/caching/requireAuthorizationForCacheControl</code>, and the value is a Boolean.</p>
    pub fn set_require_authorization_for_cache_control(
        mut self,
        input: ::std::option::Option<bool>,
    ) -> Self {
        self.require_authorization_for_cache_control = input;
        self
    }
    /// <p>Specifies how to handle unauthorized requests for cache invalidation. The PATCH path for this setting is <code>/{method_setting_key}/caching/unauthorizedCacheControlHeaderStrategy</code>, and the available values are <code>FAIL_WITH_403</code>, <code>SUCCEED_WITH_RESPONSE_HEADER</code>, <code>SUCCEED_WITHOUT_RESPONSE_HEADER</code>.</p>
    pub fn unauthorized_cache_control_header_strategy(
        mut self,
        input: crate::types::UnauthorizedCacheControlHeaderStrategy,
    ) -> Self {
        self.unauthorized_cache_control_header_strategy = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies how to handle unauthorized requests for cache invalidation. The PATCH path for this setting is <code>/{method_setting_key}/caching/unauthorizedCacheControlHeaderStrategy</code>, and the available values are <code>FAIL_WITH_403</code>, <code>SUCCEED_WITH_RESPONSE_HEADER</code>, <code>SUCCEED_WITHOUT_RESPONSE_HEADER</code>.</p>
    pub fn set_unauthorized_cache_control_header_strategy(
        mut self,
        input: ::std::option::Option<crate::types::UnauthorizedCacheControlHeaderStrategy>,
    ) -> Self {
        self.unauthorized_cache_control_header_strategy = input;
        self
    }
    /// Consumes the builder and constructs a [`MethodSetting`](crate::types::MethodSetting).
    pub fn build(self) -> crate::types::MethodSetting {
        crate::types::MethodSetting {
            metrics_enabled: self.metrics_enabled.unwrap_or_default(),
            logging_level: self.logging_level,
            data_trace_enabled: self.data_trace_enabled.unwrap_or_default(),
            throttling_burst_limit: self.throttling_burst_limit.unwrap_or_default(),
            throttling_rate_limit: self.throttling_rate_limit.unwrap_or_default(),
            caching_enabled: self.caching_enabled.unwrap_or_default(),
            cache_ttl_in_seconds: self.cache_ttl_in_seconds.unwrap_or_default(),
            cache_data_encrypted: self.cache_data_encrypted.unwrap_or_default(),
            require_authorization_for_cache_control: self
                .require_authorization_for_cache_control
                .unwrap_or_default(),
            unauthorized_cache_control_header_strategy: self
                .unauthorized_cache_control_header_strategy,
        }
    }
}
