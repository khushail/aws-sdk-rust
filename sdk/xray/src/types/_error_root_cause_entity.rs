// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A collection of segments and corresponding subsegments associated to a trace summary error.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ErrorRootCauseEntity {
    /// <p>The name of the entity.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The types and messages of the exceptions.</p>
    #[doc(hidden)]
    pub exceptions: ::std::option::Option<::std::vec::Vec<crate::types::RootCauseException>>,
    /// <p>A flag that denotes a remote subsegment.</p>
    #[doc(hidden)]
    pub remote: ::std::option::Option<bool>,
}
impl ErrorRootCauseEntity {
    /// <p>The name of the entity.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The types and messages of the exceptions.</p>
    pub fn exceptions(&self) -> ::std::option::Option<&[crate::types::RootCauseException]> {
        self.exceptions.as_deref()
    }
    /// <p>A flag that denotes a remote subsegment.</p>
    pub fn remote(&self) -> ::std::option::Option<bool> {
        self.remote
    }
}
impl ErrorRootCauseEntity {
    /// Creates a new builder-style object to manufacture [`ErrorRootCauseEntity`](crate::types::ErrorRootCauseEntity).
    pub fn builder() -> crate::types::builders::ErrorRootCauseEntityBuilder {
        crate::types::builders::ErrorRootCauseEntityBuilder::default()
    }
}

/// A builder for [`ErrorRootCauseEntity`](crate::types::ErrorRootCauseEntity).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ErrorRootCauseEntityBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) exceptions: ::std::option::Option<::std::vec::Vec<crate::types::RootCauseException>>,
    pub(crate) remote: ::std::option::Option<bool>,
}
impl ErrorRootCauseEntityBuilder {
    /// <p>The name of the entity.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the entity.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Appends an item to `exceptions`.
    ///
    /// To override the contents of this collection use [`set_exceptions`](Self::set_exceptions).
    ///
    /// <p>The types and messages of the exceptions.</p>
    pub fn exceptions(mut self, input: crate::types::RootCauseException) -> Self {
        let mut v = self.exceptions.unwrap_or_default();
        v.push(input);
        self.exceptions = ::std::option::Option::Some(v);
        self
    }
    /// <p>The types and messages of the exceptions.</p>
    pub fn set_exceptions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RootCauseException>>,
    ) -> Self {
        self.exceptions = input;
        self
    }
    /// <p>A flag that denotes a remote subsegment.</p>
    pub fn remote(mut self, input: bool) -> Self {
        self.remote = ::std::option::Option::Some(input);
        self
    }
    /// <p>A flag that denotes a remote subsegment.</p>
    pub fn set_remote(mut self, input: ::std::option::Option<bool>) -> Self {
        self.remote = input;
        self
    }
    /// Consumes the builder and constructs a [`ErrorRootCauseEntity`](crate::types::ErrorRootCauseEntity).
    pub fn build(self) -> crate::types::ErrorRootCauseEntity {
        crate::types::ErrorRootCauseEntity {
            name: self.name,
            exceptions: self.exceptions,
            remote: self.remote,
        }
    }
}
