// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the connections used by a job.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ConnectionsList {
    /// <p>A list of connections used by the job.</p>
    #[doc(hidden)]
    pub connections: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ConnectionsList {
    /// <p>A list of connections used by the job.</p>
    pub fn connections(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.connections.as_deref()
    }
}
impl ConnectionsList {
    /// Creates a new builder-style object to manufacture [`ConnectionsList`](crate::types::ConnectionsList).
    pub fn builder() -> crate::types::builders::ConnectionsListBuilder {
        crate::types::builders::ConnectionsListBuilder::default()
    }
}

/// A builder for [`ConnectionsList`](crate::types::ConnectionsList).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ConnectionsListBuilder {
    pub(crate) connections: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ConnectionsListBuilder {
    /// Appends an item to `connections`.
    ///
    /// To override the contents of this collection use [`set_connections`](Self::set_connections).
    ///
    /// <p>A list of connections used by the job.</p>
    pub fn connections(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.connections.unwrap_or_default();
        v.push(input.into());
        self.connections = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of connections used by the job.</p>
    pub fn set_connections(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.connections = input;
        self
    }
    /// Consumes the builder and constructs a [`ConnectionsList`](crate::types::ConnectionsList).
    pub fn build(self) -> crate::types::ConnectionsList {
        crate::types::ConnectionsList {
            connections: self.connections,
        }
    }
}
