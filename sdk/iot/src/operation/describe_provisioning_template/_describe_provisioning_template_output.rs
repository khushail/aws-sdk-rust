// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeProvisioningTemplateOutput {
    /// <p>The ARN of the provisioning template.</p>
    #[doc(hidden)]
    pub template_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the provisioning template.</p>
    #[doc(hidden)]
    pub template_name: ::std::option::Option<::std::string::String>,
    /// <p>The description of the provisioning template.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The date when the provisioning template was created.</p>
    #[doc(hidden)]
    pub creation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The date when the provisioning template was last modified.</p>
    #[doc(hidden)]
    pub last_modified_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The default fleet template version ID.</p>
    #[doc(hidden)]
    pub default_version_id: ::std::option::Option<i32>,
    /// <p>The JSON formatted contents of the provisioning template.</p>
    #[doc(hidden)]
    pub template_body: ::std::option::Option<::std::string::String>,
    /// <p>True if the provisioning template is enabled, otherwise false.</p>
    #[doc(hidden)]
    pub enabled: bool,
    /// <p>The ARN of the role associated with the provisioning template. This IoT role grants permission to provision a device.</p>
    #[doc(hidden)]
    pub provisioning_role_arn: ::std::option::Option<::std::string::String>,
    /// <p>Gets information about a pre-provisioned hook.</p>
    #[doc(hidden)]
    pub pre_provisioning_hook: ::std::option::Option<crate::types::ProvisioningHook>,
    /// <p>The type you define in a provisioning template. You can create a template with only one type. You can't change the template type after its creation. The default value is <code>FLEET_PROVISIONING</code>. For more information about provisioning template, see: <a href="https://docs.aws.amazon.com/iot/latest/developerguide/provision-template.html">Provisioning template</a>. </p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::TemplateType>,
    _request_id: Option<String>,
}
impl DescribeProvisioningTemplateOutput {
    /// <p>The ARN of the provisioning template.</p>
    pub fn template_arn(&self) -> ::std::option::Option<&str> {
        self.template_arn.as_deref()
    }
    /// <p>The name of the provisioning template.</p>
    pub fn template_name(&self) -> ::std::option::Option<&str> {
        self.template_name.as_deref()
    }
    /// <p>The description of the provisioning template.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The date when the provisioning template was created.</p>
    pub fn creation_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_date.as_ref()
    }
    /// <p>The date when the provisioning template was last modified.</p>
    pub fn last_modified_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_modified_date.as_ref()
    }
    /// <p>The default fleet template version ID.</p>
    pub fn default_version_id(&self) -> ::std::option::Option<i32> {
        self.default_version_id
    }
    /// <p>The JSON formatted contents of the provisioning template.</p>
    pub fn template_body(&self) -> ::std::option::Option<&str> {
        self.template_body.as_deref()
    }
    /// <p>True if the provisioning template is enabled, otherwise false.</p>
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    /// <p>The ARN of the role associated with the provisioning template. This IoT role grants permission to provision a device.</p>
    pub fn provisioning_role_arn(&self) -> ::std::option::Option<&str> {
        self.provisioning_role_arn.as_deref()
    }
    /// <p>Gets information about a pre-provisioned hook.</p>
    pub fn pre_provisioning_hook(&self) -> ::std::option::Option<&crate::types::ProvisioningHook> {
        self.pre_provisioning_hook.as_ref()
    }
    /// <p>The type you define in a provisioning template. You can create a template with only one type. You can't change the template type after its creation. The default value is <code>FLEET_PROVISIONING</code>. For more information about provisioning template, see: <a href="https://docs.aws.amazon.com/iot/latest/developerguide/provision-template.html">Provisioning template</a>. </p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::TemplateType> {
        self.r#type.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeProvisioningTemplateOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeProvisioningTemplateOutput {
    /// Creates a new builder-style object to manufacture [`DescribeProvisioningTemplateOutput`](crate::operation::describe_provisioning_template::DescribeProvisioningTemplateOutput).
    pub fn builder() -> crate::operation::describe_provisioning_template::builders::DescribeProvisioningTemplateOutputBuilder{
        crate::operation::describe_provisioning_template::builders::DescribeProvisioningTemplateOutputBuilder::default()
    }
}

/// A builder for [`DescribeProvisioningTemplateOutput`](crate::operation::describe_provisioning_template::DescribeProvisioningTemplateOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeProvisioningTemplateOutputBuilder {
    pub(crate) template_arn: ::std::option::Option<::std::string::String>,
    pub(crate) template_name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) creation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_modified_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) default_version_id: ::std::option::Option<i32>,
    pub(crate) template_body: ::std::option::Option<::std::string::String>,
    pub(crate) enabled: ::std::option::Option<bool>,
    pub(crate) provisioning_role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) pre_provisioning_hook: ::std::option::Option<crate::types::ProvisioningHook>,
    pub(crate) r#type: ::std::option::Option<crate::types::TemplateType>,
    _request_id: Option<String>,
}
impl DescribeProvisioningTemplateOutputBuilder {
    /// <p>The ARN of the provisioning template.</p>
    pub fn template_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.template_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the provisioning template.</p>
    pub fn set_template_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.template_arn = input;
        self
    }
    /// <p>The name of the provisioning template.</p>
    pub fn template_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.template_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the provisioning template.</p>
    pub fn set_template_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.template_name = input;
        self
    }
    /// <p>The description of the provisioning template.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the provisioning template.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The date when the provisioning template was created.</p>
    pub fn creation_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date when the provisioning template was created.</p>
    pub fn set_creation_date(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_date = input;
        self
    }
    /// <p>The date when the provisioning template was last modified.</p>
    pub fn last_modified_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modified_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date when the provisioning template was last modified.</p>
    pub fn set_last_modified_date(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_modified_date = input;
        self
    }
    /// <p>The default fleet template version ID.</p>
    pub fn default_version_id(mut self, input: i32) -> Self {
        self.default_version_id = ::std::option::Option::Some(input);
        self
    }
    /// <p>The default fleet template version ID.</p>
    pub fn set_default_version_id(mut self, input: ::std::option::Option<i32>) -> Self {
        self.default_version_id = input;
        self
    }
    /// <p>The JSON formatted contents of the provisioning template.</p>
    pub fn template_body(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.template_body = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The JSON formatted contents of the provisioning template.</p>
    pub fn set_template_body(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.template_body = input;
        self
    }
    /// <p>True if the provisioning template is enabled, otherwise false.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>True if the provisioning template is enabled, otherwise false.</p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// <p>The ARN of the role associated with the provisioning template. This IoT role grants permission to provision a device.</p>
    pub fn provisioning_role_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.provisioning_role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the role associated with the provisioning template. This IoT role grants permission to provision a device.</p>
    pub fn set_provisioning_role_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.provisioning_role_arn = input;
        self
    }
    /// <p>Gets information about a pre-provisioned hook.</p>
    pub fn pre_provisioning_hook(mut self, input: crate::types::ProvisioningHook) -> Self {
        self.pre_provisioning_hook = ::std::option::Option::Some(input);
        self
    }
    /// <p>Gets information about a pre-provisioned hook.</p>
    pub fn set_pre_provisioning_hook(
        mut self,
        input: ::std::option::Option<crate::types::ProvisioningHook>,
    ) -> Self {
        self.pre_provisioning_hook = input;
        self
    }
    /// <p>The type you define in a provisioning template. You can create a template with only one type. You can't change the template type after its creation. The default value is <code>FLEET_PROVISIONING</code>. For more information about provisioning template, see: <a href="https://docs.aws.amazon.com/iot/latest/developerguide/provision-template.html">Provisioning template</a>. </p>
    pub fn r#type(mut self, input: crate::types::TemplateType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type you define in a provisioning template. You can create a template with only one type. You can't change the template type after its creation. The default value is <code>FLEET_PROVISIONING</code>. For more information about provisioning template, see: <a href="https://docs.aws.amazon.com/iot/latest/developerguide/provision-template.html">Provisioning template</a>. </p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::TemplateType>) -> Self {
        self.r#type = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeProvisioningTemplateOutput`](crate::operation::describe_provisioning_template::DescribeProvisioningTemplateOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_provisioning_template::DescribeProvisioningTemplateOutput {
        crate::operation::describe_provisioning_template::DescribeProvisioningTemplateOutput {
            template_arn: self.template_arn,
            template_name: self.template_name,
            description: self.description,
            creation_date: self.creation_date,
            last_modified_date: self.last_modified_date,
            default_version_id: self.default_version_id,
            template_body: self.template_body,
            enabled: self.enabled.unwrap_or_default(),
            provisioning_role_arn: self.provisioning_role_arn,
            pre_provisioning_hook: self.pre_provisioning_hook,
            r#type: self.r#type,
            _request_id: self._request_id,
        }
    }
}
