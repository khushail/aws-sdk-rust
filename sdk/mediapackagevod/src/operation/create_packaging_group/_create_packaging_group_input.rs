// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// A new MediaPackage VOD PackagingGroup resource configuration.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreatePackagingGroupInput {
    /// CDN Authorization credentials
    #[doc(hidden)]
    pub authorization: ::std::option::Option<crate::types::Authorization>,
    /// Configure egress access logging.
    #[doc(hidden)]
    pub egress_access_logs: ::std::option::Option<crate::types::EgressAccessLogs>,
    /// The ID of the PackagingGroup.
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// A collection of tags associated with a resource
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreatePackagingGroupInput {
    /// CDN Authorization credentials
    pub fn authorization(&self) -> ::std::option::Option<&crate::types::Authorization> {
        self.authorization.as_ref()
    }
    /// Configure egress access logging.
    pub fn egress_access_logs(&self) -> ::std::option::Option<&crate::types::EgressAccessLogs> {
        self.egress_access_logs.as_ref()
    }
    /// The ID of the PackagingGroup.
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// A collection of tags associated with a resource
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
}
impl CreatePackagingGroupInput {
    /// Creates a new builder-style object to manufacture [`CreatePackagingGroupInput`](crate::operation::create_packaging_group::CreatePackagingGroupInput).
    pub fn builder(
    ) -> crate::operation::create_packaging_group::builders::CreatePackagingGroupInputBuilder {
        crate::operation::create_packaging_group::builders::CreatePackagingGroupInputBuilder::default()
    }
}

/// A builder for [`CreatePackagingGroupInput`](crate::operation::create_packaging_group::CreatePackagingGroupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreatePackagingGroupInputBuilder {
    pub(crate) authorization: ::std::option::Option<crate::types::Authorization>,
    pub(crate) egress_access_logs: ::std::option::Option<crate::types::EgressAccessLogs>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreatePackagingGroupInputBuilder {
    /// CDN Authorization credentials
    pub fn authorization(mut self, input: crate::types::Authorization) -> Self {
        self.authorization = ::std::option::Option::Some(input);
        self
    }
    /// CDN Authorization credentials
    pub fn set_authorization(
        mut self,
        input: ::std::option::Option<crate::types::Authorization>,
    ) -> Self {
        self.authorization = input;
        self
    }
    /// Configure egress access logging.
    pub fn egress_access_logs(mut self, input: crate::types::EgressAccessLogs) -> Self {
        self.egress_access_logs = ::std::option::Option::Some(input);
        self
    }
    /// Configure egress access logging.
    pub fn set_egress_access_logs(
        mut self,
        input: ::std::option::Option<crate::types::EgressAccessLogs>,
    ) -> Self {
        self.egress_access_logs = input;
        self
    }
    /// The ID of the PackagingGroup.
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// The ID of the PackagingGroup.
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// A collection of tags associated with a resource
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// A collection of tags associated with a resource
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`CreatePackagingGroupInput`](crate::operation::create_packaging_group::CreatePackagingGroupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_packaging_group::CreatePackagingGroupInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_packaging_group::CreatePackagingGroupInput {
                authorization: self.authorization,
                egress_access_logs: self.egress_access_logs,
                id: self.id,
                tags: self.tags,
            },
        )
    }
}
