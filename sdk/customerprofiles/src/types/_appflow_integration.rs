// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details for workflow of type <code>APPFLOW_INTEGRATION</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AppflowIntegration {
    /// <p>The configurations that control how Customer Profiles retrieves data from the source, Amazon AppFlow. Customer Profiles uses this information to create an AppFlow flow on behalf of customers.</p>
    #[doc(hidden)]
    pub flow_definition: ::std::option::Option<crate::types::FlowDefinition>,
    /// <p>Batches in workflow of type <code>APPFLOW_INTEGRATION</code>.</p>
    #[doc(hidden)]
    pub batches: ::std::option::Option<::std::vec::Vec<crate::types::Batch>>,
}
impl AppflowIntegration {
    /// <p>The configurations that control how Customer Profiles retrieves data from the source, Amazon AppFlow. Customer Profiles uses this information to create an AppFlow flow on behalf of customers.</p>
    pub fn flow_definition(&self) -> ::std::option::Option<&crate::types::FlowDefinition> {
        self.flow_definition.as_ref()
    }
    /// <p>Batches in workflow of type <code>APPFLOW_INTEGRATION</code>.</p>
    pub fn batches(&self) -> ::std::option::Option<&[crate::types::Batch]> {
        self.batches.as_deref()
    }
}
impl AppflowIntegration {
    /// Creates a new builder-style object to manufacture [`AppflowIntegration`](crate::types::AppflowIntegration).
    pub fn builder() -> crate::types::builders::AppflowIntegrationBuilder {
        crate::types::builders::AppflowIntegrationBuilder::default()
    }
}

/// A builder for [`AppflowIntegration`](crate::types::AppflowIntegration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AppflowIntegrationBuilder {
    pub(crate) flow_definition: ::std::option::Option<crate::types::FlowDefinition>,
    pub(crate) batches: ::std::option::Option<::std::vec::Vec<crate::types::Batch>>,
}
impl AppflowIntegrationBuilder {
    /// <p>The configurations that control how Customer Profiles retrieves data from the source, Amazon AppFlow. Customer Profiles uses this information to create an AppFlow flow on behalf of customers.</p>
    pub fn flow_definition(mut self, input: crate::types::FlowDefinition) -> Self {
        self.flow_definition = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configurations that control how Customer Profiles retrieves data from the source, Amazon AppFlow. Customer Profiles uses this information to create an AppFlow flow on behalf of customers.</p>
    pub fn set_flow_definition(
        mut self,
        input: ::std::option::Option<crate::types::FlowDefinition>,
    ) -> Self {
        self.flow_definition = input;
        self
    }
    /// Appends an item to `batches`.
    ///
    /// To override the contents of this collection use [`set_batches`](Self::set_batches).
    ///
    /// <p>Batches in workflow of type <code>APPFLOW_INTEGRATION</code>.</p>
    pub fn batches(mut self, input: crate::types::Batch) -> Self {
        let mut v = self.batches.unwrap_or_default();
        v.push(input);
        self.batches = ::std::option::Option::Some(v);
        self
    }
    /// <p>Batches in workflow of type <code>APPFLOW_INTEGRATION</code>.</p>
    pub fn set_batches(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Batch>>,
    ) -> Self {
        self.batches = input;
        self
    }
    /// Consumes the builder and constructs a [`AppflowIntegration`](crate::types::AppflowIntegration).
    pub fn build(self) -> crate::types::AppflowIntegration {
        crate::types::AppflowIntegration {
            flow_definition: self.flow_definition,
            batches: self.batches,
        }
    }
}
