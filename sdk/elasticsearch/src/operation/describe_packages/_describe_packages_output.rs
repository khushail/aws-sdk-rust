// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Container for response returned by <code> <code>DescribePackages</code> </code> operation. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribePackagesOutput {
    /// <p>List of <code>PackageDetails</code> objects.</p>
    #[doc(hidden)]
    pub package_details_list: ::std::option::Option<::std::vec::Vec<crate::types::PackageDetails>>,
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribePackagesOutput {
    /// <p>List of <code>PackageDetails</code> objects.</p>
    pub fn package_details_list(&self) -> ::std::option::Option<&[crate::types::PackageDetails]> {
        self.package_details_list.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribePackagesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribePackagesOutput {
    /// Creates a new builder-style object to manufacture [`DescribePackagesOutput`](crate::operation::describe_packages::DescribePackagesOutput).
    pub fn builder() -> crate::operation::describe_packages::builders::DescribePackagesOutputBuilder
    {
        crate::operation::describe_packages::builders::DescribePackagesOutputBuilder::default()
    }
}

/// A builder for [`DescribePackagesOutput`](crate::operation::describe_packages::DescribePackagesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribePackagesOutputBuilder {
    pub(crate) package_details_list:
        ::std::option::Option<::std::vec::Vec<crate::types::PackageDetails>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribePackagesOutputBuilder {
    /// Appends an item to `package_details_list`.
    ///
    /// To override the contents of this collection use [`set_package_details_list`](Self::set_package_details_list).
    ///
    /// <p>List of <code>PackageDetails</code> objects.</p>
    pub fn package_details_list(mut self, input: crate::types::PackageDetails) -> Self {
        let mut v = self.package_details_list.unwrap_or_default();
        v.push(input);
        self.package_details_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>List of <code>PackageDetails</code> objects.</p>
    pub fn set_package_details_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PackageDetails>>,
    ) -> Self {
        self.package_details_list = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`DescribePackagesOutput`](crate::operation::describe_packages::DescribePackagesOutput).
    pub fn build(self) -> crate::operation::describe_packages::DescribePackagesOutput {
        crate::operation::describe_packages::DescribePackagesOutput {
            package_details_list: self.package_details_list,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
