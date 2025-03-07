// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The database details of an Amazon RDS database.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RdsDatabase {
    /// <p>The ID of an RDS DB instance.</p>
    #[doc(hidden)]
    pub instance_identifier: ::std::option::Option<::std::string::String>,
    /// <p>The name of a database hosted on an RDS DB instance.</p>
    #[doc(hidden)]
    pub database_name: ::std::option::Option<::std::string::String>,
}
impl RdsDatabase {
    /// <p>The ID of an RDS DB instance.</p>
    pub fn instance_identifier(&self) -> ::std::option::Option<&str> {
        self.instance_identifier.as_deref()
    }
    /// <p>The name of a database hosted on an RDS DB instance.</p>
    pub fn database_name(&self) -> ::std::option::Option<&str> {
        self.database_name.as_deref()
    }
}
impl RdsDatabase {
    /// Creates a new builder-style object to manufacture [`RdsDatabase`](crate::types::RdsDatabase).
    pub fn builder() -> crate::types::builders::RdsDatabaseBuilder {
        crate::types::builders::RdsDatabaseBuilder::default()
    }
}

/// A builder for [`RdsDatabase`](crate::types::RdsDatabase).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RdsDatabaseBuilder {
    pub(crate) instance_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) database_name: ::std::option::Option<::std::string::String>,
}
impl RdsDatabaseBuilder {
    /// <p>The ID of an RDS DB instance.</p>
    pub fn instance_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.instance_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of an RDS DB instance.</p>
    pub fn set_instance_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.instance_identifier = input;
        self
    }
    /// <p>The name of a database hosted on an RDS DB instance.</p>
    pub fn database_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.database_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of a database hosted on an RDS DB instance.</p>
    pub fn set_database_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.database_name = input;
        self
    }
    /// Consumes the builder and constructs a [`RdsDatabase`](crate::types::RdsDatabase).
    pub fn build(self) -> crate::types::RdsDatabase {
        crate::types::RdsDatabase {
            instance_identifier: self.instance_identifier,
            database_name: self.database_name,
        }
    }
}
