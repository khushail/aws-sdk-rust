// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Input information used for an action execution.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ActionExecutionInput {
    /// <p>Represents information about an action type.</p>
    #[doc(hidden)]
    pub action_type_id: ::std::option::Option<crate::types::ActionTypeId>,
    /// <p>Configuration data for an action execution.</p>
    #[doc(hidden)]
    pub configuration: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// <p>Configuration data for an action execution with all variable references replaced with their real values for the execution.</p>
    #[doc(hidden)]
    pub resolved_configuration: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// <p>The ARN of the IAM service role that performs the declared action. This is assumed through the roleArn for the pipeline. </p>
    #[doc(hidden)]
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>The AWS Region for the action, such as us-east-1.</p>
    #[doc(hidden)]
    pub region: ::std::option::Option<::std::string::String>,
    /// <p>Details of input artifacts of the action that correspond to the action execution.</p>
    #[doc(hidden)]
    pub input_artifacts: ::std::option::Option<::std::vec::Vec<crate::types::ArtifactDetail>>,
    /// <p>The variable namespace associated with the action. All variables produced as output by this action fall under this namespace.</p>
    #[doc(hidden)]
    pub namespace: ::std::option::Option<::std::string::String>,
}
impl ActionExecutionInput {
    /// <p>Represents information about an action type.</p>
    pub fn action_type_id(&self) -> ::std::option::Option<&crate::types::ActionTypeId> {
        self.action_type_id.as_ref()
    }
    /// <p>Configuration data for an action execution.</p>
    pub fn configuration(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.configuration.as_ref()
    }
    /// <p>Configuration data for an action execution with all variable references replaced with their real values for the execution.</p>
    pub fn resolved_configuration(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.resolved_configuration.as_ref()
    }
    /// <p>The ARN of the IAM service role that performs the declared action. This is assumed through the roleArn for the pipeline. </p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>The AWS Region for the action, such as us-east-1.</p>
    pub fn region(&self) -> ::std::option::Option<&str> {
        self.region.as_deref()
    }
    /// <p>Details of input artifacts of the action that correspond to the action execution.</p>
    pub fn input_artifacts(&self) -> ::std::option::Option<&[crate::types::ArtifactDetail]> {
        self.input_artifacts.as_deref()
    }
    /// <p>The variable namespace associated with the action. All variables produced as output by this action fall under this namespace.</p>
    pub fn namespace(&self) -> ::std::option::Option<&str> {
        self.namespace.as_deref()
    }
}
impl ActionExecutionInput {
    /// Creates a new builder-style object to manufacture [`ActionExecutionInput`](crate::types::ActionExecutionInput).
    pub fn builder() -> crate::types::builders::ActionExecutionInputBuilder {
        crate::types::builders::ActionExecutionInputBuilder::default()
    }
}

/// A builder for [`ActionExecutionInput`](crate::types::ActionExecutionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ActionExecutionInputBuilder {
    pub(crate) action_type_id: ::std::option::Option<crate::types::ActionTypeId>,
    pub(crate) configuration: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) resolved_configuration: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) region: ::std::option::Option<::std::string::String>,
    pub(crate) input_artifacts:
        ::std::option::Option<::std::vec::Vec<crate::types::ArtifactDetail>>,
    pub(crate) namespace: ::std::option::Option<::std::string::String>,
}
impl ActionExecutionInputBuilder {
    /// <p>Represents information about an action type.</p>
    pub fn action_type_id(mut self, input: crate::types::ActionTypeId) -> Self {
        self.action_type_id = ::std::option::Option::Some(input);
        self
    }
    /// <p>Represents information about an action type.</p>
    pub fn set_action_type_id(
        mut self,
        input: ::std::option::Option<crate::types::ActionTypeId>,
    ) -> Self {
        self.action_type_id = input;
        self
    }
    /// Adds a key-value pair to `configuration`.
    ///
    /// To override the contents of this collection use [`set_configuration`](Self::set_configuration).
    ///
    /// <p>Configuration data for an action execution.</p>
    pub fn configuration(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.configuration.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.configuration = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Configuration data for an action execution.</p>
    pub fn set_configuration(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.configuration = input;
        self
    }
    /// Adds a key-value pair to `resolved_configuration`.
    ///
    /// To override the contents of this collection use [`set_resolved_configuration`](Self::set_resolved_configuration).
    ///
    /// <p>Configuration data for an action execution with all variable references replaced with their real values for the execution.</p>
    pub fn resolved_configuration(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.resolved_configuration.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.resolved_configuration = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Configuration data for an action execution with all variable references replaced with their real values for the execution.</p>
    pub fn set_resolved_configuration(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.resolved_configuration = input;
        self
    }
    /// <p>The ARN of the IAM service role that performs the declared action. This is assumed through the roleArn for the pipeline. </p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the IAM service role that performs the declared action. This is assumed through the roleArn for the pipeline. </p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>The AWS Region for the action, such as us-east-1.</p>
    pub fn region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The AWS Region for the action, such as us-east-1.</p>
    pub fn set_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.region = input;
        self
    }
    /// Appends an item to `input_artifacts`.
    ///
    /// To override the contents of this collection use [`set_input_artifacts`](Self::set_input_artifacts).
    ///
    /// <p>Details of input artifacts of the action that correspond to the action execution.</p>
    pub fn input_artifacts(mut self, input: crate::types::ArtifactDetail) -> Self {
        let mut v = self.input_artifacts.unwrap_or_default();
        v.push(input);
        self.input_artifacts = ::std::option::Option::Some(v);
        self
    }
    /// <p>Details of input artifacts of the action that correspond to the action execution.</p>
    pub fn set_input_artifacts(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ArtifactDetail>>,
    ) -> Self {
        self.input_artifacts = input;
        self
    }
    /// <p>The variable namespace associated with the action. All variables produced as output by this action fall under this namespace.</p>
    pub fn namespace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.namespace = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The variable namespace associated with the action. All variables produced as output by this action fall under this namespace.</p>
    pub fn set_namespace(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.namespace = input;
        self
    }
    /// Consumes the builder and constructs a [`ActionExecutionInput`](crate::types::ActionExecutionInput).
    pub fn build(self) -> crate::types::ActionExecutionInput {
        crate::types::ActionExecutionInput {
            action_type_id: self.action_type_id,
            configuration: self.configuration,
            resolved_configuration: self.resolved_configuration,
            role_arn: self.role_arn,
            region: self.region,
            input_artifacts: self.input_artifacts,
            namespace: self.namespace,
        }
    }
}
