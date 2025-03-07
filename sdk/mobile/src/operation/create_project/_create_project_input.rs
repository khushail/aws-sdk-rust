// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Request structure used to request a project be created. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateProjectInput {
    /// <p> Name of the project. </p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p> Default region where project resources should be created. </p>
    #[doc(hidden)]
    pub region: ::std::option::Option<::std::string::String>,
    /// <p> ZIP or YAML file which contains configuration settings to be used when creating the project. This may be the contents of the file downloaded from the URL provided in an export project operation. </p>
    #[doc(hidden)]
    pub contents: ::std::option::Option<::aws_smithy_types::Blob>,
    /// <p> Unique identifier for an exported snapshot of project configuration. This snapshot identifier is included in the share URL when a project is exported. </p>
    #[doc(hidden)]
    pub snapshot_id: ::std::option::Option<::std::string::String>,
}
impl CreateProjectInput {
    /// <p> Name of the project. </p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p> Default region where project resources should be created. </p>
    pub fn region(&self) -> ::std::option::Option<&str> {
        self.region.as_deref()
    }
    /// <p> ZIP or YAML file which contains configuration settings to be used when creating the project. This may be the contents of the file downloaded from the URL provided in an export project operation. </p>
    pub fn contents(&self) -> ::std::option::Option<&::aws_smithy_types::Blob> {
        self.contents.as_ref()
    }
    /// <p> Unique identifier for an exported snapshot of project configuration. This snapshot identifier is included in the share URL when a project is exported. </p>
    pub fn snapshot_id(&self) -> ::std::option::Option<&str> {
        self.snapshot_id.as_deref()
    }
}
impl CreateProjectInput {
    /// Creates a new builder-style object to manufacture [`CreateProjectInput`](crate::operation::create_project::CreateProjectInput).
    pub fn builder() -> crate::operation::create_project::builders::CreateProjectInputBuilder {
        crate::operation::create_project::builders::CreateProjectInputBuilder::default()
    }
}

/// A builder for [`CreateProjectInput`](crate::operation::create_project::CreateProjectInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateProjectInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) region: ::std::option::Option<::std::string::String>,
    pub(crate) contents: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) snapshot_id: ::std::option::Option<::std::string::String>,
}
impl CreateProjectInputBuilder {
    /// <p> Name of the project. </p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> Name of the project. </p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p> Default region where project resources should be created. </p>
    pub fn region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> Default region where project resources should be created. </p>
    pub fn set_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.region = input;
        self
    }
    /// <p> ZIP or YAML file which contains configuration settings to be used when creating the project. This may be the contents of the file downloaded from the URL provided in an export project operation. </p>
    pub fn contents(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.contents = ::std::option::Option::Some(input);
        self
    }
    /// <p> ZIP or YAML file which contains configuration settings to be used when creating the project. This may be the contents of the file downloaded from the URL provided in an export project operation. </p>
    pub fn set_contents(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.contents = input;
        self
    }
    /// <p> Unique identifier for an exported snapshot of project configuration. This snapshot identifier is included in the share URL when a project is exported. </p>
    pub fn snapshot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.snapshot_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> Unique identifier for an exported snapshot of project configuration. This snapshot identifier is included in the share URL when a project is exported. </p>
    pub fn set_snapshot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.snapshot_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateProjectInput`](crate::operation::create_project::CreateProjectInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_project::CreateProjectInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_project::CreateProjectInput {
            name: self.name,
            region: self.region,
            contents: self.contents,
            snapshot_id: self.snapshot_id,
        })
    }
}
