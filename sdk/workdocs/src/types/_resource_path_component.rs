// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the resource path.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ResourcePathComponent {
    /// <p>The ID of the resource path.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the resource path.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ResourcePathComponent {
    /// <p>The ID of the resource path.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The name of the resource path.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl ::std::fmt::Debug for ResourcePathComponent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ResourcePathComponent");
        formatter.field("id", &self.id);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl ResourcePathComponent {
    /// Creates a new builder-style object to manufacture [`ResourcePathComponent`](crate::types::ResourcePathComponent).
    pub fn builder() -> crate::types::builders::ResourcePathComponentBuilder {
        crate::types::builders::ResourcePathComponentBuilder::default()
    }
}

/// A builder for [`ResourcePathComponent`](crate::types::ResourcePathComponent).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct ResourcePathComponentBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl ResourcePathComponentBuilder {
    /// <p>The ID of the resource path.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the resource path.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The name of the resource path.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the resource path.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`ResourcePathComponent`](crate::types::ResourcePathComponent).
    pub fn build(self) -> crate::types::ResourcePathComponent {
        crate::types::ResourcePathComponent {
            id: self.id,
            name: self.name,
        }
    }
}
impl ::std::fmt::Debug for ResourcePathComponentBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ResourcePathComponentBuilder");
        formatter.field("id", &self.id);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
