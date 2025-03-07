// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that contains the catalog data for a repository. This data is publicly visible in the Amazon ECR Public Gallery.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RepositoryCatalogDataInput {
    /// <p>A short description of the contents of the repository. This text appears in both the image details and also when searching for repositories on the Amazon ECR Public Gallery.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The system architecture that the images in the repository are compatible with. On the Amazon ECR Public Gallery, the following supported architectures appear as badges on the repository and are used as search filters.</p> <note>
    /// <p>If an unsupported tag is added to your repository catalog data, it's associated with the repository and can be retrieved using the API but isn't discoverable in the Amazon ECR Public Gallery.</p>
    /// </note>
    /// <ul>
    /// <li> <p> <code>ARM</code> </p> </li>
    /// <li> <p> <code>ARM 64</code> </p> </li>
    /// <li> <p> <code>x86</code> </p> </li>
    /// <li> <p> <code>x86-64</code> </p> </li>
    /// </ul>
    #[doc(hidden)]
    pub architectures: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The operating systems that the images in the repository are compatible with. On the Amazon ECR Public Gallery, the following supported operating systems appear as badges on the repository and are used as search filters.</p> <note>
    /// <p>If an unsupported tag is added to your repository catalog data, it's associated with the repository and can be retrieved using the API but isn't discoverable in the Amazon ECR Public Gallery.</p>
    /// </note>
    /// <ul>
    /// <li> <p> <code>Linux</code> </p> </li>
    /// <li> <p> <code>Windows</code> </p> </li>
    /// </ul>
    #[doc(hidden)]
    pub operating_systems: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The base64-encoded repository logo payload.</p> <note>
    /// <p>The repository logo is only publicly visible in the Amazon ECR Public Gallery for verified accounts.</p>
    /// </note>
    #[doc(hidden)]
    pub logo_image_blob: ::std::option::Option<::aws_smithy_types::Blob>,
    /// <p>A detailed description of the contents of the repository. It's publicly visible in the Amazon ECR Public Gallery. The text must be in markdown format.</p>
    #[doc(hidden)]
    pub about_text: ::std::option::Option<::std::string::String>,
    /// <p>Detailed information about how to use the contents of the repository. It's publicly visible in the Amazon ECR Public Gallery. The usage text provides context, support information, and additional usage details for users of the repository. The text must be in markdown format.</p>
    #[doc(hidden)]
    pub usage_text: ::std::option::Option<::std::string::String>,
}
impl RepositoryCatalogDataInput {
    /// <p>A short description of the contents of the repository. This text appears in both the image details and also when searching for repositories on the Amazon ECR Public Gallery.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The system architecture that the images in the repository are compatible with. On the Amazon ECR Public Gallery, the following supported architectures appear as badges on the repository and are used as search filters.</p> <note>
    /// <p>If an unsupported tag is added to your repository catalog data, it's associated with the repository and can be retrieved using the API but isn't discoverable in the Amazon ECR Public Gallery.</p>
    /// </note>
    /// <ul>
    /// <li> <p> <code>ARM</code> </p> </li>
    /// <li> <p> <code>ARM 64</code> </p> </li>
    /// <li> <p> <code>x86</code> </p> </li>
    /// <li> <p> <code>x86-64</code> </p> </li>
    /// </ul>
    pub fn architectures(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.architectures.as_deref()
    }
    /// <p>The operating systems that the images in the repository are compatible with. On the Amazon ECR Public Gallery, the following supported operating systems appear as badges on the repository and are used as search filters.</p> <note>
    /// <p>If an unsupported tag is added to your repository catalog data, it's associated with the repository and can be retrieved using the API but isn't discoverable in the Amazon ECR Public Gallery.</p>
    /// </note>
    /// <ul>
    /// <li> <p> <code>Linux</code> </p> </li>
    /// <li> <p> <code>Windows</code> </p> </li>
    /// </ul>
    pub fn operating_systems(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.operating_systems.as_deref()
    }
    /// <p>The base64-encoded repository logo payload.</p> <note>
    /// <p>The repository logo is only publicly visible in the Amazon ECR Public Gallery for verified accounts.</p>
    /// </note>
    pub fn logo_image_blob(&self) -> ::std::option::Option<&::aws_smithy_types::Blob> {
        self.logo_image_blob.as_ref()
    }
    /// <p>A detailed description of the contents of the repository. It's publicly visible in the Amazon ECR Public Gallery. The text must be in markdown format.</p>
    pub fn about_text(&self) -> ::std::option::Option<&str> {
        self.about_text.as_deref()
    }
    /// <p>Detailed information about how to use the contents of the repository. It's publicly visible in the Amazon ECR Public Gallery. The usage text provides context, support information, and additional usage details for users of the repository. The text must be in markdown format.</p>
    pub fn usage_text(&self) -> ::std::option::Option<&str> {
        self.usage_text.as_deref()
    }
}
impl RepositoryCatalogDataInput {
    /// Creates a new builder-style object to manufacture [`RepositoryCatalogDataInput`](crate::types::RepositoryCatalogDataInput).
    pub fn builder() -> crate::types::builders::RepositoryCatalogDataInputBuilder {
        crate::types::builders::RepositoryCatalogDataInputBuilder::default()
    }
}

/// A builder for [`RepositoryCatalogDataInput`](crate::types::RepositoryCatalogDataInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RepositoryCatalogDataInputBuilder {
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) architectures: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) operating_systems: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) logo_image_blob: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) about_text: ::std::option::Option<::std::string::String>,
    pub(crate) usage_text: ::std::option::Option<::std::string::String>,
}
impl RepositoryCatalogDataInputBuilder {
    /// <p>A short description of the contents of the repository. This text appears in both the image details and also when searching for repositories on the Amazon ECR Public Gallery.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A short description of the contents of the repository. This text appears in both the image details and also when searching for repositories on the Amazon ECR Public Gallery.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Appends an item to `architectures`.
    ///
    /// To override the contents of this collection use [`set_architectures`](Self::set_architectures).
    ///
    /// <p>The system architecture that the images in the repository are compatible with. On the Amazon ECR Public Gallery, the following supported architectures appear as badges on the repository and are used as search filters.</p> <note>
    /// <p>If an unsupported tag is added to your repository catalog data, it's associated with the repository and can be retrieved using the API but isn't discoverable in the Amazon ECR Public Gallery.</p>
    /// </note>
    /// <ul>
    /// <li> <p> <code>ARM</code> </p> </li>
    /// <li> <p> <code>ARM 64</code> </p> </li>
    /// <li> <p> <code>x86</code> </p> </li>
    /// <li> <p> <code>x86-64</code> </p> </li>
    /// </ul>
    pub fn architectures(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.architectures.unwrap_or_default();
        v.push(input.into());
        self.architectures = ::std::option::Option::Some(v);
        self
    }
    /// <p>The system architecture that the images in the repository are compatible with. On the Amazon ECR Public Gallery, the following supported architectures appear as badges on the repository and are used as search filters.</p> <note>
    /// <p>If an unsupported tag is added to your repository catalog data, it's associated with the repository and can be retrieved using the API but isn't discoverable in the Amazon ECR Public Gallery.</p>
    /// </note>
    /// <ul>
    /// <li> <p> <code>ARM</code> </p> </li>
    /// <li> <p> <code>ARM 64</code> </p> </li>
    /// <li> <p> <code>x86</code> </p> </li>
    /// <li> <p> <code>x86-64</code> </p> </li>
    /// </ul>
    pub fn set_architectures(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.architectures = input;
        self
    }
    /// Appends an item to `operating_systems`.
    ///
    /// To override the contents of this collection use [`set_operating_systems`](Self::set_operating_systems).
    ///
    /// <p>The operating systems that the images in the repository are compatible with. On the Amazon ECR Public Gallery, the following supported operating systems appear as badges on the repository and are used as search filters.</p> <note>
    /// <p>If an unsupported tag is added to your repository catalog data, it's associated with the repository and can be retrieved using the API but isn't discoverable in the Amazon ECR Public Gallery.</p>
    /// </note>
    /// <ul>
    /// <li> <p> <code>Linux</code> </p> </li>
    /// <li> <p> <code>Windows</code> </p> </li>
    /// </ul>
    pub fn operating_systems(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.operating_systems.unwrap_or_default();
        v.push(input.into());
        self.operating_systems = ::std::option::Option::Some(v);
        self
    }
    /// <p>The operating systems that the images in the repository are compatible with. On the Amazon ECR Public Gallery, the following supported operating systems appear as badges on the repository and are used as search filters.</p> <note>
    /// <p>If an unsupported tag is added to your repository catalog data, it's associated with the repository and can be retrieved using the API but isn't discoverable in the Amazon ECR Public Gallery.</p>
    /// </note>
    /// <ul>
    /// <li> <p> <code>Linux</code> </p> </li>
    /// <li> <p> <code>Windows</code> </p> </li>
    /// </ul>
    pub fn set_operating_systems(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.operating_systems = input;
        self
    }
    /// <p>The base64-encoded repository logo payload.</p> <note>
    /// <p>The repository logo is only publicly visible in the Amazon ECR Public Gallery for verified accounts.</p>
    /// </note>
    pub fn logo_image_blob(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.logo_image_blob = ::std::option::Option::Some(input);
        self
    }
    /// <p>The base64-encoded repository logo payload.</p> <note>
    /// <p>The repository logo is only publicly visible in the Amazon ECR Public Gallery for verified accounts.</p>
    /// </note>
    pub fn set_logo_image_blob(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.logo_image_blob = input;
        self
    }
    /// <p>A detailed description of the contents of the repository. It's publicly visible in the Amazon ECR Public Gallery. The text must be in markdown format.</p>
    pub fn about_text(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.about_text = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A detailed description of the contents of the repository. It's publicly visible in the Amazon ECR Public Gallery. The text must be in markdown format.</p>
    pub fn set_about_text(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.about_text = input;
        self
    }
    /// <p>Detailed information about how to use the contents of the repository. It's publicly visible in the Amazon ECR Public Gallery. The usage text provides context, support information, and additional usage details for users of the repository. The text must be in markdown format.</p>
    pub fn usage_text(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.usage_text = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Detailed information about how to use the contents of the repository. It's publicly visible in the Amazon ECR Public Gallery. The usage text provides context, support information, and additional usage details for users of the repository. The text must be in markdown format.</p>
    pub fn set_usage_text(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.usage_text = input;
        self
    }
    /// Consumes the builder and constructs a [`RepositoryCatalogDataInput`](crate::types::RepositoryCatalogDataInput).
    pub fn build(self) -> crate::types::RepositoryCatalogDataInput {
        crate::types::RepositoryCatalogDataInput {
            description: self.description,
            architectures: self.architectures,
            operating_systems: self.operating_systems,
            logo_image_blob: self.logo_image_blob,
            about_text: self.about_text,
            usage_text: self.usage_text,
        }
    }
}
