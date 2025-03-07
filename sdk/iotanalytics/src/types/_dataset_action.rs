// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A <code>DatasetAction</code> object that specifies how dataset contents are automatically created.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DatasetAction {
    /// <p>The name of the dataset action by which dataset contents are automatically created.</p>
    #[doc(hidden)]
    pub action_name: ::std::option::Option<::std::string::String>,
    /// <p>An <code>SqlQueryDatasetAction</code> object that uses an SQL query to automatically create dataset contents.</p>
    #[doc(hidden)]
    pub query_action: ::std::option::Option<crate::types::SqlQueryDatasetAction>,
    /// <p>Information that allows the system to run a containerized application to create the dataset contents. The application must be in a Docker container along with any required support libraries.</p>
    #[doc(hidden)]
    pub container_action: ::std::option::Option<crate::types::ContainerDatasetAction>,
}
impl DatasetAction {
    /// <p>The name of the dataset action by which dataset contents are automatically created.</p>
    pub fn action_name(&self) -> ::std::option::Option<&str> {
        self.action_name.as_deref()
    }
    /// <p>An <code>SqlQueryDatasetAction</code> object that uses an SQL query to automatically create dataset contents.</p>
    pub fn query_action(&self) -> ::std::option::Option<&crate::types::SqlQueryDatasetAction> {
        self.query_action.as_ref()
    }
    /// <p>Information that allows the system to run a containerized application to create the dataset contents. The application must be in a Docker container along with any required support libraries.</p>
    pub fn container_action(&self) -> ::std::option::Option<&crate::types::ContainerDatasetAction> {
        self.container_action.as_ref()
    }
}
impl DatasetAction {
    /// Creates a new builder-style object to manufacture [`DatasetAction`](crate::types::DatasetAction).
    pub fn builder() -> crate::types::builders::DatasetActionBuilder {
        crate::types::builders::DatasetActionBuilder::default()
    }
}

/// A builder for [`DatasetAction`](crate::types::DatasetAction).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DatasetActionBuilder {
    pub(crate) action_name: ::std::option::Option<::std::string::String>,
    pub(crate) query_action: ::std::option::Option<crate::types::SqlQueryDatasetAction>,
    pub(crate) container_action: ::std::option::Option<crate::types::ContainerDatasetAction>,
}
impl DatasetActionBuilder {
    /// <p>The name of the dataset action by which dataset contents are automatically created.</p>
    pub fn action_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.action_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the dataset action by which dataset contents are automatically created.</p>
    pub fn set_action_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.action_name = input;
        self
    }
    /// <p>An <code>SqlQueryDatasetAction</code> object that uses an SQL query to automatically create dataset contents.</p>
    pub fn query_action(mut self, input: crate::types::SqlQueryDatasetAction) -> Self {
        self.query_action = ::std::option::Option::Some(input);
        self
    }
    /// <p>An <code>SqlQueryDatasetAction</code> object that uses an SQL query to automatically create dataset contents.</p>
    pub fn set_query_action(
        mut self,
        input: ::std::option::Option<crate::types::SqlQueryDatasetAction>,
    ) -> Self {
        self.query_action = input;
        self
    }
    /// <p>Information that allows the system to run a containerized application to create the dataset contents. The application must be in a Docker container along with any required support libraries.</p>
    pub fn container_action(mut self, input: crate::types::ContainerDatasetAction) -> Self {
        self.container_action = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information that allows the system to run a containerized application to create the dataset contents. The application must be in a Docker container along with any required support libraries.</p>
    pub fn set_container_action(
        mut self,
        input: ::std::option::Option<crate::types::ContainerDatasetAction>,
    ) -> Self {
        self.container_action = input;
        self
    }
    /// Consumes the builder and constructs a [`DatasetAction`](crate::types::DatasetAction).
    pub fn build(self) -> crate::types::DatasetAction {
        crate::types::DatasetAction {
            action_name: self.action_name,
            query_action: self.query_action,
            container_action: self.container_action,
        }
    }
}
