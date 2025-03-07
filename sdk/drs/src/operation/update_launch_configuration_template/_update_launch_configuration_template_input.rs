// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateLaunchConfigurationTemplateInput {
    /// <p>Launch Configuration Template ID.</p>
    #[doc(hidden)]
    pub launch_configuration_template_id: ::std::option::Option<::std::string::String>,
    /// <p>Launch disposition.</p>
    #[doc(hidden)]
    pub launch_disposition: ::std::option::Option<crate::types::LaunchDisposition>,
    /// <p>Target instance type right-sizing method.</p>
    #[doc(hidden)]
    pub target_instance_type_right_sizing_method:
        ::std::option::Option<crate::types::TargetInstanceTypeRightSizingMethod>,
    /// <p>Copy private IP.</p>
    #[doc(hidden)]
    pub copy_private_ip: ::std::option::Option<bool>,
    /// <p>Copy tags.</p>
    #[doc(hidden)]
    pub copy_tags: ::std::option::Option<bool>,
    /// <p>Licensing.</p>
    #[doc(hidden)]
    pub licensing: ::std::option::Option<crate::types::Licensing>,
}
impl UpdateLaunchConfigurationTemplateInput {
    /// <p>Launch Configuration Template ID.</p>
    pub fn launch_configuration_template_id(&self) -> ::std::option::Option<&str> {
        self.launch_configuration_template_id.as_deref()
    }
    /// <p>Launch disposition.</p>
    pub fn launch_disposition(&self) -> ::std::option::Option<&crate::types::LaunchDisposition> {
        self.launch_disposition.as_ref()
    }
    /// <p>Target instance type right-sizing method.</p>
    pub fn target_instance_type_right_sizing_method(
        &self,
    ) -> ::std::option::Option<&crate::types::TargetInstanceTypeRightSizingMethod> {
        self.target_instance_type_right_sizing_method.as_ref()
    }
    /// <p>Copy private IP.</p>
    pub fn copy_private_ip(&self) -> ::std::option::Option<bool> {
        self.copy_private_ip
    }
    /// <p>Copy tags.</p>
    pub fn copy_tags(&self) -> ::std::option::Option<bool> {
        self.copy_tags
    }
    /// <p>Licensing.</p>
    pub fn licensing(&self) -> ::std::option::Option<&crate::types::Licensing> {
        self.licensing.as_ref()
    }
}
impl UpdateLaunchConfigurationTemplateInput {
    /// Creates a new builder-style object to manufacture [`UpdateLaunchConfigurationTemplateInput`](crate::operation::update_launch_configuration_template::UpdateLaunchConfigurationTemplateInput).
    pub fn builder() -> crate::operation::update_launch_configuration_template::builders::UpdateLaunchConfigurationTemplateInputBuilder{
        crate::operation::update_launch_configuration_template::builders::UpdateLaunchConfigurationTemplateInputBuilder::default()
    }
}

/// A builder for [`UpdateLaunchConfigurationTemplateInput`](crate::operation::update_launch_configuration_template::UpdateLaunchConfigurationTemplateInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateLaunchConfigurationTemplateInputBuilder {
    pub(crate) launch_configuration_template_id: ::std::option::Option<::std::string::String>,
    pub(crate) launch_disposition: ::std::option::Option<crate::types::LaunchDisposition>,
    pub(crate) target_instance_type_right_sizing_method:
        ::std::option::Option<crate::types::TargetInstanceTypeRightSizingMethod>,
    pub(crate) copy_private_ip: ::std::option::Option<bool>,
    pub(crate) copy_tags: ::std::option::Option<bool>,
    pub(crate) licensing: ::std::option::Option<crate::types::Licensing>,
}
impl UpdateLaunchConfigurationTemplateInputBuilder {
    /// <p>Launch Configuration Template ID.</p>
    pub fn launch_configuration_template_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.launch_configuration_template_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Launch Configuration Template ID.</p>
    pub fn set_launch_configuration_template_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.launch_configuration_template_id = input;
        self
    }
    /// <p>Launch disposition.</p>
    pub fn launch_disposition(mut self, input: crate::types::LaunchDisposition) -> Self {
        self.launch_disposition = ::std::option::Option::Some(input);
        self
    }
    /// <p>Launch disposition.</p>
    pub fn set_launch_disposition(
        mut self,
        input: ::std::option::Option<crate::types::LaunchDisposition>,
    ) -> Self {
        self.launch_disposition = input;
        self
    }
    /// <p>Target instance type right-sizing method.</p>
    pub fn target_instance_type_right_sizing_method(
        mut self,
        input: crate::types::TargetInstanceTypeRightSizingMethod,
    ) -> Self {
        self.target_instance_type_right_sizing_method = ::std::option::Option::Some(input);
        self
    }
    /// <p>Target instance type right-sizing method.</p>
    pub fn set_target_instance_type_right_sizing_method(
        mut self,
        input: ::std::option::Option<crate::types::TargetInstanceTypeRightSizingMethod>,
    ) -> Self {
        self.target_instance_type_right_sizing_method = input;
        self
    }
    /// <p>Copy private IP.</p>
    pub fn copy_private_ip(mut self, input: bool) -> Self {
        self.copy_private_ip = ::std::option::Option::Some(input);
        self
    }
    /// <p>Copy private IP.</p>
    pub fn set_copy_private_ip(mut self, input: ::std::option::Option<bool>) -> Self {
        self.copy_private_ip = input;
        self
    }
    /// <p>Copy tags.</p>
    pub fn copy_tags(mut self, input: bool) -> Self {
        self.copy_tags = ::std::option::Option::Some(input);
        self
    }
    /// <p>Copy tags.</p>
    pub fn set_copy_tags(mut self, input: ::std::option::Option<bool>) -> Self {
        self.copy_tags = input;
        self
    }
    /// <p>Licensing.</p>
    pub fn licensing(mut self, input: crate::types::Licensing) -> Self {
        self.licensing = ::std::option::Option::Some(input);
        self
    }
    /// <p>Licensing.</p>
    pub fn set_licensing(mut self, input: ::std::option::Option<crate::types::Licensing>) -> Self {
        self.licensing = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateLaunchConfigurationTemplateInput`](crate::operation::update_launch_configuration_template::UpdateLaunchConfigurationTemplateInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::update_launch_configuration_template::UpdateLaunchConfigurationTemplateInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::update_launch_configuration_template::UpdateLaunchConfigurationTemplateInput {
                launch_configuration_template_id: self.launch_configuration_template_id
                ,
                launch_disposition: self.launch_disposition
                ,
                target_instance_type_right_sizing_method: self.target_instance_type_right_sizing_method
                ,
                copy_private_ip: self.copy_private_ip
                ,
                copy_tags: self.copy_tags
                ,
                licensing: self.licensing
                ,
            }
        )
    }
}
