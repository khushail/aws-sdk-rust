// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RegisterDbProxyTargetsOutput {
    /// <p>One or more <code>DBProxyTarget</code> objects that are created when you register targets with a target group.</p>
    #[doc(hidden)]
    pub db_proxy_targets: ::std::option::Option<::std::vec::Vec<crate::types::DbProxyTarget>>,
    _request_id: Option<String>,
}
impl RegisterDbProxyTargetsOutput {
    /// <p>One or more <code>DBProxyTarget</code> objects that are created when you register targets with a target group.</p>
    pub fn db_proxy_targets(&self) -> ::std::option::Option<&[crate::types::DbProxyTarget]> {
        self.db_proxy_targets.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for RegisterDbProxyTargetsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RegisterDbProxyTargetsOutput {
    /// Creates a new builder-style object to manufacture [`RegisterDbProxyTargetsOutput`](crate::operation::register_db_proxy_targets::RegisterDbProxyTargetsOutput).
    pub fn builder(
    ) -> crate::operation::register_db_proxy_targets::builders::RegisterDbProxyTargetsOutputBuilder
    {
        crate::operation::register_db_proxy_targets::builders::RegisterDbProxyTargetsOutputBuilder::default()
    }
}

/// A builder for [`RegisterDbProxyTargetsOutput`](crate::operation::register_db_proxy_targets::RegisterDbProxyTargetsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RegisterDbProxyTargetsOutputBuilder {
    pub(crate) db_proxy_targets:
        ::std::option::Option<::std::vec::Vec<crate::types::DbProxyTarget>>,
    _request_id: Option<String>,
}
impl RegisterDbProxyTargetsOutputBuilder {
    /// Appends an item to `db_proxy_targets`.
    ///
    /// To override the contents of this collection use [`set_db_proxy_targets`](Self::set_db_proxy_targets).
    ///
    /// <p>One or more <code>DBProxyTarget</code> objects that are created when you register targets with a target group.</p>
    pub fn db_proxy_targets(mut self, input: crate::types::DbProxyTarget) -> Self {
        let mut v = self.db_proxy_targets.unwrap_or_default();
        v.push(input);
        self.db_proxy_targets = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more <code>DBProxyTarget</code> objects that are created when you register targets with a target group.</p>
    pub fn set_db_proxy_targets(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DbProxyTarget>>,
    ) -> Self {
        self.db_proxy_targets = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`RegisterDbProxyTargetsOutput`](crate::operation::register_db_proxy_targets::RegisterDbProxyTargetsOutput).
    pub fn build(
        self,
    ) -> crate::operation::register_db_proxy_targets::RegisterDbProxyTargetsOutput {
        crate::operation::register_db_proxy_targets::RegisterDbProxyTargetsOutput {
            db_proxy_targets: self.db_proxy_targets,
            _request_id: self._request_id,
        }
    }
}
