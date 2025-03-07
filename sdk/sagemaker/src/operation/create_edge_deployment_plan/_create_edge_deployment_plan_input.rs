// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateEdgeDeploymentPlanInput {
    /// <p>The name of the edge deployment plan.</p>
    #[doc(hidden)]
    pub edge_deployment_plan_name: ::std::option::Option<::std::string::String>,
    /// <p>List of models associated with the edge deployment plan.</p>
    #[doc(hidden)]
    pub model_configs:
        ::std::option::Option<::std::vec::Vec<crate::types::EdgeDeploymentModelConfig>>,
    /// <p>The device fleet used for this edge deployment plan.</p>
    #[doc(hidden)]
    pub device_fleet_name: ::std::option::Option<::std::string::String>,
    /// <p>List of stages of the edge deployment plan. The number of stages is limited to 10 per deployment.</p>
    #[doc(hidden)]
    pub stages: ::std::option::Option<::std::vec::Vec<crate::types::DeploymentStage>>,
    /// <p>List of tags with which to tag the edge deployment plan.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateEdgeDeploymentPlanInput {
    /// <p>The name of the edge deployment plan.</p>
    pub fn edge_deployment_plan_name(&self) -> ::std::option::Option<&str> {
        self.edge_deployment_plan_name.as_deref()
    }
    /// <p>List of models associated with the edge deployment plan.</p>
    pub fn model_configs(
        &self,
    ) -> ::std::option::Option<&[crate::types::EdgeDeploymentModelConfig]> {
        self.model_configs.as_deref()
    }
    /// <p>The device fleet used for this edge deployment plan.</p>
    pub fn device_fleet_name(&self) -> ::std::option::Option<&str> {
        self.device_fleet_name.as_deref()
    }
    /// <p>List of stages of the edge deployment plan. The number of stages is limited to 10 per deployment.</p>
    pub fn stages(&self) -> ::std::option::Option<&[crate::types::DeploymentStage]> {
        self.stages.as_deref()
    }
    /// <p>List of tags with which to tag the edge deployment plan.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl CreateEdgeDeploymentPlanInput {
    /// Creates a new builder-style object to manufacture [`CreateEdgeDeploymentPlanInput`](crate::operation::create_edge_deployment_plan::CreateEdgeDeploymentPlanInput).
    pub fn builder(
    ) -> crate::operation::create_edge_deployment_plan::builders::CreateEdgeDeploymentPlanInputBuilder
    {
        crate::operation::create_edge_deployment_plan::builders::CreateEdgeDeploymentPlanInputBuilder::default()
    }
}

/// A builder for [`CreateEdgeDeploymentPlanInput`](crate::operation::create_edge_deployment_plan::CreateEdgeDeploymentPlanInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateEdgeDeploymentPlanInputBuilder {
    pub(crate) edge_deployment_plan_name: ::std::option::Option<::std::string::String>,
    pub(crate) model_configs:
        ::std::option::Option<::std::vec::Vec<crate::types::EdgeDeploymentModelConfig>>,
    pub(crate) device_fleet_name: ::std::option::Option<::std::string::String>,
    pub(crate) stages: ::std::option::Option<::std::vec::Vec<crate::types::DeploymentStage>>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateEdgeDeploymentPlanInputBuilder {
    /// <p>The name of the edge deployment plan.</p>
    pub fn edge_deployment_plan_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.edge_deployment_plan_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the edge deployment plan.</p>
    pub fn set_edge_deployment_plan_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.edge_deployment_plan_name = input;
        self
    }
    /// Appends an item to `model_configs`.
    ///
    /// To override the contents of this collection use [`set_model_configs`](Self::set_model_configs).
    ///
    /// <p>List of models associated with the edge deployment plan.</p>
    pub fn model_configs(mut self, input: crate::types::EdgeDeploymentModelConfig) -> Self {
        let mut v = self.model_configs.unwrap_or_default();
        v.push(input);
        self.model_configs = ::std::option::Option::Some(v);
        self
    }
    /// <p>List of models associated with the edge deployment plan.</p>
    pub fn set_model_configs(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::EdgeDeploymentModelConfig>>,
    ) -> Self {
        self.model_configs = input;
        self
    }
    /// <p>The device fleet used for this edge deployment plan.</p>
    pub fn device_fleet_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.device_fleet_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The device fleet used for this edge deployment plan.</p>
    pub fn set_device_fleet_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.device_fleet_name = input;
        self
    }
    /// Appends an item to `stages`.
    ///
    /// To override the contents of this collection use [`set_stages`](Self::set_stages).
    ///
    /// <p>List of stages of the edge deployment plan. The number of stages is limited to 10 per deployment.</p>
    pub fn stages(mut self, input: crate::types::DeploymentStage) -> Self {
        let mut v = self.stages.unwrap_or_default();
        v.push(input);
        self.stages = ::std::option::Option::Some(v);
        self
    }
    /// <p>List of stages of the edge deployment plan. The number of stages is limited to 10 per deployment.</p>
    pub fn set_stages(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DeploymentStage>>,
    ) -> Self {
        self.stages = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>List of tags with which to tag the edge deployment plan.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>List of tags with which to tag the edge deployment plan.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateEdgeDeploymentPlanInput`](crate::operation::create_edge_deployment_plan::CreateEdgeDeploymentPlanInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_edge_deployment_plan::CreateEdgeDeploymentPlanInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_edge_deployment_plan::CreateEdgeDeploymentPlanInput {
                edge_deployment_plan_name: self.edge_deployment_plan_name,
                model_configs: self.model_configs,
                device_fleet_name: self.device_fleet_name,
                stages: self.stages,
                tags: self.tags,
            },
        )
    }
}
