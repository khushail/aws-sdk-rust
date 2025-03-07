// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetPackageVersionAssetInput {
    /// <p> The name of the domain that contains the repository that contains the package version with the requested asset. </p>
    #[doc(hidden)]
    pub domain: ::std::option::Option<::std::string::String>,
    /// <p> The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces. </p>
    #[doc(hidden)]
    pub domain_owner: ::std::option::Option<::std::string::String>,
    /// <p> The repository that contains the package version with the requested asset. </p>
    #[doc(hidden)]
    pub repository: ::std::option::Option<::std::string::String>,
    /// <p> A format that specifies the type of the package version with the requested asset file. </p>
    #[doc(hidden)]
    pub format: ::std::option::Option<crate::types::PackageFormat>,
    /// <p>The namespace of the package version with the requested asset file. The package version component that specifies its namespace depends on its type. For example:</p>
    /// <ul>
    /// <li> <p> The namespace of a Maven package version is its <code>groupId</code>. </p> </li>
    /// <li> <p> The namespace of an npm package version is its <code>scope</code>. </p> </li>
    /// <li> <p> Python and NuGet package versions do not contain a corresponding component, package versions of those formats do not have a namespace. </p> </li>
    /// <li> <p> The namespace of a generic package is its <code>namespace</code>. </p> </li>
    /// </ul>
    #[doc(hidden)]
    pub namespace: ::std::option::Option<::std::string::String>,
    /// <p> The name of the package that contains the requested asset. </p>
    #[doc(hidden)]
    pub package: ::std::option::Option<::std::string::String>,
    /// <p> A string that contains the package version (for example, <code>3.5.2</code>). </p>
    #[doc(hidden)]
    pub package_version: ::std::option::Option<::std::string::String>,
    /// <p> The name of the requested asset. </p>
    #[doc(hidden)]
    pub asset: ::std::option::Option<::std::string::String>,
    /// <p> The name of the package version revision that contains the requested asset. </p>
    #[doc(hidden)]
    pub package_version_revision: ::std::option::Option<::std::string::String>,
}
impl GetPackageVersionAssetInput {
    /// <p> The name of the domain that contains the repository that contains the package version with the requested asset. </p>
    pub fn domain(&self) -> ::std::option::Option<&str> {
        self.domain.as_deref()
    }
    /// <p> The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces. </p>
    pub fn domain_owner(&self) -> ::std::option::Option<&str> {
        self.domain_owner.as_deref()
    }
    /// <p> The repository that contains the package version with the requested asset. </p>
    pub fn repository(&self) -> ::std::option::Option<&str> {
        self.repository.as_deref()
    }
    /// <p> A format that specifies the type of the package version with the requested asset file. </p>
    pub fn format(&self) -> ::std::option::Option<&crate::types::PackageFormat> {
        self.format.as_ref()
    }
    /// <p>The namespace of the package version with the requested asset file. The package version component that specifies its namespace depends on its type. For example:</p>
    /// <ul>
    /// <li> <p> The namespace of a Maven package version is its <code>groupId</code>. </p> </li>
    /// <li> <p> The namespace of an npm package version is its <code>scope</code>. </p> </li>
    /// <li> <p> Python and NuGet package versions do not contain a corresponding component, package versions of those formats do not have a namespace. </p> </li>
    /// <li> <p> The namespace of a generic package is its <code>namespace</code>. </p> </li>
    /// </ul>
    pub fn namespace(&self) -> ::std::option::Option<&str> {
        self.namespace.as_deref()
    }
    /// <p> The name of the package that contains the requested asset. </p>
    pub fn package(&self) -> ::std::option::Option<&str> {
        self.package.as_deref()
    }
    /// <p> A string that contains the package version (for example, <code>3.5.2</code>). </p>
    pub fn package_version(&self) -> ::std::option::Option<&str> {
        self.package_version.as_deref()
    }
    /// <p> The name of the requested asset. </p>
    pub fn asset(&self) -> ::std::option::Option<&str> {
        self.asset.as_deref()
    }
    /// <p> The name of the package version revision that contains the requested asset. </p>
    pub fn package_version_revision(&self) -> ::std::option::Option<&str> {
        self.package_version_revision.as_deref()
    }
}
impl GetPackageVersionAssetInput {
    /// Creates a new builder-style object to manufacture [`GetPackageVersionAssetInput`](crate::operation::get_package_version_asset::GetPackageVersionAssetInput).
    pub fn builder(
    ) -> crate::operation::get_package_version_asset::builders::GetPackageVersionAssetInputBuilder
    {
        crate::operation::get_package_version_asset::builders::GetPackageVersionAssetInputBuilder::default()
    }
}

/// A builder for [`GetPackageVersionAssetInput`](crate::operation::get_package_version_asset::GetPackageVersionAssetInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetPackageVersionAssetInputBuilder {
    pub(crate) domain: ::std::option::Option<::std::string::String>,
    pub(crate) domain_owner: ::std::option::Option<::std::string::String>,
    pub(crate) repository: ::std::option::Option<::std::string::String>,
    pub(crate) format: ::std::option::Option<crate::types::PackageFormat>,
    pub(crate) namespace: ::std::option::Option<::std::string::String>,
    pub(crate) package: ::std::option::Option<::std::string::String>,
    pub(crate) package_version: ::std::option::Option<::std::string::String>,
    pub(crate) asset: ::std::option::Option<::std::string::String>,
    pub(crate) package_version_revision: ::std::option::Option<::std::string::String>,
}
impl GetPackageVersionAssetInputBuilder {
    /// <p> The name of the domain that contains the repository that contains the package version with the requested asset. </p>
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The name of the domain that contains the repository that contains the package version with the requested asset. </p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain = input;
        self
    }
    /// <p> The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces. </p>
    pub fn domain_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain_owner = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces. </p>
    pub fn set_domain_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain_owner = input;
        self
    }
    /// <p> The repository that contains the package version with the requested asset. </p>
    pub fn repository(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.repository = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The repository that contains the package version with the requested asset. </p>
    pub fn set_repository(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.repository = input;
        self
    }
    /// <p> A format that specifies the type of the package version with the requested asset file. </p>
    pub fn format(mut self, input: crate::types::PackageFormat) -> Self {
        self.format = ::std::option::Option::Some(input);
        self
    }
    /// <p> A format that specifies the type of the package version with the requested asset file. </p>
    pub fn set_format(mut self, input: ::std::option::Option<crate::types::PackageFormat>) -> Self {
        self.format = input;
        self
    }
    /// <p>The namespace of the package version with the requested asset file. The package version component that specifies its namespace depends on its type. For example:</p>
    /// <ul>
    /// <li> <p> The namespace of a Maven package version is its <code>groupId</code>. </p> </li>
    /// <li> <p> The namespace of an npm package version is its <code>scope</code>. </p> </li>
    /// <li> <p> Python and NuGet package versions do not contain a corresponding component, package versions of those formats do not have a namespace. </p> </li>
    /// <li> <p> The namespace of a generic package is its <code>namespace</code>. </p> </li>
    /// </ul>
    pub fn namespace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.namespace = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The namespace of the package version with the requested asset file. The package version component that specifies its namespace depends on its type. For example:</p>
    /// <ul>
    /// <li> <p> The namespace of a Maven package version is its <code>groupId</code>. </p> </li>
    /// <li> <p> The namespace of an npm package version is its <code>scope</code>. </p> </li>
    /// <li> <p> Python and NuGet package versions do not contain a corresponding component, package versions of those formats do not have a namespace. </p> </li>
    /// <li> <p> The namespace of a generic package is its <code>namespace</code>. </p> </li>
    /// </ul>
    pub fn set_namespace(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.namespace = input;
        self
    }
    /// <p> The name of the package that contains the requested asset. </p>
    pub fn package(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.package = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The name of the package that contains the requested asset. </p>
    pub fn set_package(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.package = input;
        self
    }
    /// <p> A string that contains the package version (for example, <code>3.5.2</code>). </p>
    pub fn package_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.package_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> A string that contains the package version (for example, <code>3.5.2</code>). </p>
    pub fn set_package_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.package_version = input;
        self
    }
    /// <p> The name of the requested asset. </p>
    pub fn asset(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.asset = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The name of the requested asset. </p>
    pub fn set_asset(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.asset = input;
        self
    }
    /// <p> The name of the package version revision that contains the requested asset. </p>
    pub fn package_version_revision(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.package_version_revision = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The name of the package version revision that contains the requested asset. </p>
    pub fn set_package_version_revision(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.package_version_revision = input;
        self
    }
    /// Consumes the builder and constructs a [`GetPackageVersionAssetInput`](crate::operation::get_package_version_asset::GetPackageVersionAssetInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_package_version_asset::GetPackageVersionAssetInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_package_version_asset::GetPackageVersionAssetInput {
                domain: self.domain,
                domain_owner: self.domain_owner,
                repository: self.repository,
                format: self.format,
                namespace: self.namespace,
                package: self.package,
                package_version: self.package_version,
                asset: self.asset,
                package_version_revision: self.package_version_revision,
            },
        )
    }
}
