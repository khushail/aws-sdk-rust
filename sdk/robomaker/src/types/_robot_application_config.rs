// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Application configuration information for a robot.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RobotApplicationConfig {
    /// <p>The application information for the robot application.</p>
    #[doc(hidden)]
    pub application: ::std::option::Option<::std::string::String>,
    /// <p>The version of the robot application.</p>
    #[doc(hidden)]
    pub application_version: ::std::option::Option<::std::string::String>,
    /// <p>The launch configuration for the robot application.</p>
    #[doc(hidden)]
    pub launch_config: ::std::option::Option<crate::types::LaunchConfig>,
    /// <p>The upload configurations for the robot application.</p>
    #[doc(hidden)]
    pub upload_configurations:
        ::std::option::Option<::std::vec::Vec<crate::types::UploadConfiguration>>,
    /// <p>A Boolean indicating whether to use default upload configurations. By default, <code>.ros</code> and <code>.gazebo</code> files are uploaded when the application terminates and all ROS topics will be recorded.</p>
    /// <p>If you set this value, you must specify an <code>outputLocation</code>.</p> <important>
    /// <p>This API is no longer supported and will throw an error if used.</p>
    /// </important>
    #[deprecated(
        note = "AWS RoboMaker is ending support for ROS software suite. For additional information, see https://docs.aws.amazon.com/robomaker/latest/dg/software-support-policy.html."
    )]
    #[doc(hidden)]
    pub use_default_upload_configurations: ::std::option::Option<bool>,
    /// <p>Information about tools configured for the robot application.</p>
    #[doc(hidden)]
    pub tools: ::std::option::Option<::std::vec::Vec<crate::types::Tool>>,
    /// <p>A Boolean indicating whether to use default robot application tools. The default tools are rviz, rqt, terminal and rosbag record. The default is <code>False</code>.</p> <important>
    /// <p>This API is no longer supported and will throw an error if used.</p>
    /// </important>
    #[deprecated(
        note = "AWS RoboMaker is ending support for ROS software suite. For additional information, see https://docs.aws.amazon.com/robomaker/latest/dg/software-support-policy.html."
    )]
    #[doc(hidden)]
    pub use_default_tools: ::std::option::Option<bool>,
}
impl RobotApplicationConfig {
    /// <p>The application information for the robot application.</p>
    pub fn application(&self) -> ::std::option::Option<&str> {
        self.application.as_deref()
    }
    /// <p>The version of the robot application.</p>
    pub fn application_version(&self) -> ::std::option::Option<&str> {
        self.application_version.as_deref()
    }
    /// <p>The launch configuration for the robot application.</p>
    pub fn launch_config(&self) -> ::std::option::Option<&crate::types::LaunchConfig> {
        self.launch_config.as_ref()
    }
    /// <p>The upload configurations for the robot application.</p>
    pub fn upload_configurations(
        &self,
    ) -> ::std::option::Option<&[crate::types::UploadConfiguration]> {
        self.upload_configurations.as_deref()
    }
    /// <p>A Boolean indicating whether to use default upload configurations. By default, <code>.ros</code> and <code>.gazebo</code> files are uploaded when the application terminates and all ROS topics will be recorded.</p>
    /// <p>If you set this value, you must specify an <code>outputLocation</code>.</p> <important>
    /// <p>This API is no longer supported and will throw an error if used.</p>
    /// </important>
    #[deprecated(
        note = "AWS RoboMaker is ending support for ROS software suite. For additional information, see https://docs.aws.amazon.com/robomaker/latest/dg/software-support-policy.html."
    )]
    pub fn use_default_upload_configurations(&self) -> ::std::option::Option<bool> {
        self.use_default_upload_configurations
    }
    /// <p>Information about tools configured for the robot application.</p>
    pub fn tools(&self) -> ::std::option::Option<&[crate::types::Tool]> {
        self.tools.as_deref()
    }
    /// <p>A Boolean indicating whether to use default robot application tools. The default tools are rviz, rqt, terminal and rosbag record. The default is <code>False</code>.</p> <important>
    /// <p>This API is no longer supported and will throw an error if used.</p>
    /// </important>
    #[deprecated(
        note = "AWS RoboMaker is ending support for ROS software suite. For additional information, see https://docs.aws.amazon.com/robomaker/latest/dg/software-support-policy.html."
    )]
    pub fn use_default_tools(&self) -> ::std::option::Option<bool> {
        self.use_default_tools
    }
}
impl RobotApplicationConfig {
    /// Creates a new builder-style object to manufacture [`RobotApplicationConfig`](crate::types::RobotApplicationConfig).
    pub fn builder() -> crate::types::builders::RobotApplicationConfigBuilder {
        crate::types::builders::RobotApplicationConfigBuilder::default()
    }
}

/// A builder for [`RobotApplicationConfig`](crate::types::RobotApplicationConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RobotApplicationConfigBuilder {
    pub(crate) application: ::std::option::Option<::std::string::String>,
    pub(crate) application_version: ::std::option::Option<::std::string::String>,
    pub(crate) launch_config: ::std::option::Option<crate::types::LaunchConfig>,
    pub(crate) upload_configurations:
        ::std::option::Option<::std::vec::Vec<crate::types::UploadConfiguration>>,
    pub(crate) use_default_upload_configurations: ::std::option::Option<bool>,
    pub(crate) tools: ::std::option::Option<::std::vec::Vec<crate::types::Tool>>,
    pub(crate) use_default_tools: ::std::option::Option<bool>,
}
impl RobotApplicationConfigBuilder {
    /// <p>The application information for the robot application.</p>
    pub fn application(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.application = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The application information for the robot application.</p>
    pub fn set_application(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.application = input;
        self
    }
    /// <p>The version of the robot application.</p>
    pub fn application_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the robot application.</p>
    pub fn set_application_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_version = input;
        self
    }
    /// <p>The launch configuration for the robot application.</p>
    pub fn launch_config(mut self, input: crate::types::LaunchConfig) -> Self {
        self.launch_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The launch configuration for the robot application.</p>
    pub fn set_launch_config(
        mut self,
        input: ::std::option::Option<crate::types::LaunchConfig>,
    ) -> Self {
        self.launch_config = input;
        self
    }
    /// Appends an item to `upload_configurations`.
    ///
    /// To override the contents of this collection use [`set_upload_configurations`](Self::set_upload_configurations).
    ///
    /// <p>The upload configurations for the robot application.</p>
    pub fn upload_configurations(mut self, input: crate::types::UploadConfiguration) -> Self {
        let mut v = self.upload_configurations.unwrap_or_default();
        v.push(input);
        self.upload_configurations = ::std::option::Option::Some(v);
        self
    }
    /// <p>The upload configurations for the robot application.</p>
    pub fn set_upload_configurations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::UploadConfiguration>>,
    ) -> Self {
        self.upload_configurations = input;
        self
    }
    /// <p>A Boolean indicating whether to use default upload configurations. By default, <code>.ros</code> and <code>.gazebo</code> files are uploaded when the application terminates and all ROS topics will be recorded.</p>
    /// <p>If you set this value, you must specify an <code>outputLocation</code>.</p> <important>
    /// <p>This API is no longer supported and will throw an error if used.</p>
    /// </important>
    #[deprecated(
        note = "AWS RoboMaker is ending support for ROS software suite. For additional information, see https://docs.aws.amazon.com/robomaker/latest/dg/software-support-policy.html."
    )]
    pub fn use_default_upload_configurations(mut self, input: bool) -> Self {
        self.use_default_upload_configurations = ::std::option::Option::Some(input);
        self
    }
    /// <p>A Boolean indicating whether to use default upload configurations. By default, <code>.ros</code> and <code>.gazebo</code> files are uploaded when the application terminates and all ROS topics will be recorded.</p>
    /// <p>If you set this value, you must specify an <code>outputLocation</code>.</p> <important>
    /// <p>This API is no longer supported and will throw an error if used.</p>
    /// </important>
    #[deprecated(
        note = "AWS RoboMaker is ending support for ROS software suite. For additional information, see https://docs.aws.amazon.com/robomaker/latest/dg/software-support-policy.html."
    )]
    pub fn set_use_default_upload_configurations(
        mut self,
        input: ::std::option::Option<bool>,
    ) -> Self {
        self.use_default_upload_configurations = input;
        self
    }
    /// Appends an item to `tools`.
    ///
    /// To override the contents of this collection use [`set_tools`](Self::set_tools).
    ///
    /// <p>Information about tools configured for the robot application.</p>
    pub fn tools(mut self, input: crate::types::Tool) -> Self {
        let mut v = self.tools.unwrap_or_default();
        v.push(input);
        self.tools = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about tools configured for the robot application.</p>
    pub fn set_tools(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tool>>,
    ) -> Self {
        self.tools = input;
        self
    }
    /// <p>A Boolean indicating whether to use default robot application tools. The default tools are rviz, rqt, terminal and rosbag record. The default is <code>False</code>.</p> <important>
    /// <p>This API is no longer supported and will throw an error if used.</p>
    /// </important>
    #[deprecated(
        note = "AWS RoboMaker is ending support for ROS software suite. For additional information, see https://docs.aws.amazon.com/robomaker/latest/dg/software-support-policy.html."
    )]
    pub fn use_default_tools(mut self, input: bool) -> Self {
        self.use_default_tools = ::std::option::Option::Some(input);
        self
    }
    /// <p>A Boolean indicating whether to use default robot application tools. The default tools are rviz, rqt, terminal and rosbag record. The default is <code>False</code>.</p> <important>
    /// <p>This API is no longer supported and will throw an error if used.</p>
    /// </important>
    #[deprecated(
        note = "AWS RoboMaker is ending support for ROS software suite. For additional information, see https://docs.aws.amazon.com/robomaker/latest/dg/software-support-policy.html."
    )]
    pub fn set_use_default_tools(mut self, input: ::std::option::Option<bool>) -> Self {
        self.use_default_tools = input;
        self
    }
    /// Consumes the builder and constructs a [`RobotApplicationConfig`](crate::types::RobotApplicationConfig).
    pub fn build(self) -> crate::types::RobotApplicationConfig {
        crate::types::RobotApplicationConfig {
            application: self.application,
            application_version: self.application_version,
            launch_config: self.launch_config,
            upload_configurations: self.upload_configurations,
            use_default_upload_configurations: self.use_default_upload_configurations,
            tools: self.tools,
            use_default_tools: self.use_default_tools,
        }
    }
}
