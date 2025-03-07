// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct DescribeStateMachineOutput {
    /// <p>The Amazon Resource Name (ARN) that identifies the state machine.</p>
    #[doc(hidden)]
    pub state_machine_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the state machine.</p>
    /// <p>A name must <i>not</i> contain:</p>
    /// <ul>
    /// <li> <p>white space</p> </li>
    /// <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li>
    /// <li> <p>wildcard characters <code>? *</code> </p> </li>
    /// <li> <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li>
    /// <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li>
    /// </ul>
    /// <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The current status of the state machine.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::StateMachineStatus>,
    /// <p>The Amazon States Language definition of the state machine. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html">Amazon States Language</a>.</p>
    #[doc(hidden)]
    pub definition: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role used when creating this state machine. (The IAM role maintains security by granting Step Functions access to Amazon Web Services resources.)</p>
    #[doc(hidden)]
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>The <code>type</code> of the state machine (<code>STANDARD</code> or <code>EXPRESS</code>).</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::StateMachineType>,
    /// <p>The date the state machine is created.</p>
    #[doc(hidden)]
    pub creation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The <code>LoggingConfiguration</code> data type is used to set CloudWatch Logs options.</p>
    #[doc(hidden)]
    pub logging_configuration: ::std::option::Option<crate::types::LoggingConfiguration>,
    /// <p>Selects whether X-Ray tracing is enabled.</p>
    #[doc(hidden)]
    pub tracing_configuration: ::std::option::Option<crate::types::TracingConfiguration>,
    /// <p>A user-defined or an auto-generated string that identifies a <code>Map</code> state. This parameter is present only if the <code>stateMachineArn</code> specified in input is a qualified state machine ARN.</p>
    #[doc(hidden)]
    pub label: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeStateMachineOutput {
    /// <p>The Amazon Resource Name (ARN) that identifies the state machine.</p>
    pub fn state_machine_arn(&self) -> ::std::option::Option<&str> {
        self.state_machine_arn.as_deref()
    }
    /// <p>The name of the state machine.</p>
    /// <p>A name must <i>not</i> contain:</p>
    /// <ul>
    /// <li> <p>white space</p> </li>
    /// <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li>
    /// <li> <p>wildcard characters <code>? *</code> </p> </li>
    /// <li> <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li>
    /// <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li>
    /// </ul>
    /// <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The current status of the state machine.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::StateMachineStatus> {
        self.status.as_ref()
    }
    /// <p>The Amazon States Language definition of the state machine. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html">Amazon States Language</a>.</p>
    pub fn definition(&self) -> ::std::option::Option<&str> {
        self.definition.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role used when creating this state machine. (The IAM role maintains security by granting Step Functions access to Amazon Web Services resources.)</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>The <code>type</code> of the state machine (<code>STANDARD</code> or <code>EXPRESS</code>).</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::StateMachineType> {
        self.r#type.as_ref()
    }
    /// <p>The date the state machine is created.</p>
    pub fn creation_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_date.as_ref()
    }
    /// <p>The <code>LoggingConfiguration</code> data type is used to set CloudWatch Logs options.</p>
    pub fn logging_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::LoggingConfiguration> {
        self.logging_configuration.as_ref()
    }
    /// <p>Selects whether X-Ray tracing is enabled.</p>
    pub fn tracing_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::TracingConfiguration> {
        self.tracing_configuration.as_ref()
    }
    /// <p>A user-defined or an auto-generated string that identifies a <code>Map</code> state. This parameter is present only if the <code>stateMachineArn</code> specified in input is a qualified state machine ARN.</p>
    pub fn label(&self) -> ::std::option::Option<&str> {
        self.label.as_deref()
    }
}
impl ::std::fmt::Debug for DescribeStateMachineOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeStateMachineOutput");
        formatter.field("state_machine_arn", &self.state_machine_arn);
        formatter.field("name", &self.name);
        formatter.field("status", &self.status);
        formatter.field("definition", &"*** Sensitive Data Redacted ***");
        formatter.field("role_arn", &self.role_arn);
        formatter.field("r#type", &self.r#type);
        formatter.field("creation_date", &self.creation_date);
        formatter.field("logging_configuration", &self.logging_configuration);
        formatter.field("tracing_configuration", &self.tracing_configuration);
        formatter.field("label", &self.label);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl ::aws_http::request_id::RequestId for DescribeStateMachineOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeStateMachineOutput {
    /// Creates a new builder-style object to manufacture [`DescribeStateMachineOutput`](crate::operation::describe_state_machine::DescribeStateMachineOutput).
    pub fn builder(
    ) -> crate::operation::describe_state_machine::builders::DescribeStateMachineOutputBuilder {
        crate::operation::describe_state_machine::builders::DescribeStateMachineOutputBuilder::default()
    }
}

/// A builder for [`DescribeStateMachineOutput`](crate::operation::describe_state_machine::DescribeStateMachineOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct DescribeStateMachineOutputBuilder {
    pub(crate) state_machine_arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::StateMachineStatus>,
    pub(crate) definition: ::std::option::Option<::std::string::String>,
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<crate::types::StateMachineType>,
    pub(crate) creation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) logging_configuration: ::std::option::Option<crate::types::LoggingConfiguration>,
    pub(crate) tracing_configuration: ::std::option::Option<crate::types::TracingConfiguration>,
    pub(crate) label: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeStateMachineOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) that identifies the state machine.</p>
    pub fn state_machine_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.state_machine_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that identifies the state machine.</p>
    pub fn set_state_machine_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.state_machine_arn = input;
        self
    }
    /// <p>The name of the state machine.</p>
    /// <p>A name must <i>not</i> contain:</p>
    /// <ul>
    /// <li> <p>white space</p> </li>
    /// <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li>
    /// <li> <p>wildcard characters <code>? *</code> </p> </li>
    /// <li> <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li>
    /// <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li>
    /// </ul>
    /// <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the state machine.</p>
    /// <p>A name must <i>not</i> contain:</p>
    /// <ul>
    /// <li> <p>white space</p> </li>
    /// <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li>
    /// <li> <p>wildcard characters <code>? *</code> </p> </li>
    /// <li> <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li>
    /// <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li>
    /// </ul>
    /// <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The current status of the state machine.</p>
    pub fn status(mut self, input: crate::types::StateMachineStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of the state machine.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::StateMachineStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>The Amazon States Language definition of the state machine. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html">Amazon States Language</a>.</p>
    pub fn definition(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.definition = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon States Language definition of the state machine. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html">Amazon States Language</a>.</p>
    pub fn set_definition(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.definition = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role used when creating this state machine. (The IAM role maintains security by granting Step Functions access to Amazon Web Services resources.)</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role used when creating this state machine. (The IAM role maintains security by granting Step Functions access to Amazon Web Services resources.)</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>The <code>type</code> of the state machine (<code>STANDARD</code> or <code>EXPRESS</code>).</p>
    pub fn r#type(mut self, input: crate::types::StateMachineType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The <code>type</code> of the state machine (<code>STANDARD</code> or <code>EXPRESS</code>).</p>
    pub fn set_type(
        mut self,
        input: ::std::option::Option<crate::types::StateMachineType>,
    ) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The date the state machine is created.</p>
    pub fn creation_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date the state machine is created.</p>
    pub fn set_creation_date(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_date = input;
        self
    }
    /// <p>The <code>LoggingConfiguration</code> data type is used to set CloudWatch Logs options.</p>
    pub fn logging_configuration(mut self, input: crate::types::LoggingConfiguration) -> Self {
        self.logging_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The <code>LoggingConfiguration</code> data type is used to set CloudWatch Logs options.</p>
    pub fn set_logging_configuration(
        mut self,
        input: ::std::option::Option<crate::types::LoggingConfiguration>,
    ) -> Self {
        self.logging_configuration = input;
        self
    }
    /// <p>Selects whether X-Ray tracing is enabled.</p>
    pub fn tracing_configuration(mut self, input: crate::types::TracingConfiguration) -> Self {
        self.tracing_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Selects whether X-Ray tracing is enabled.</p>
    pub fn set_tracing_configuration(
        mut self,
        input: ::std::option::Option<crate::types::TracingConfiguration>,
    ) -> Self {
        self.tracing_configuration = input;
        self
    }
    /// <p>A user-defined or an auto-generated string that identifies a <code>Map</code> state. This parameter is present only if the <code>stateMachineArn</code> specified in input is a qualified state machine ARN.</p>
    pub fn label(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.label = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A user-defined or an auto-generated string that identifies a <code>Map</code> state. This parameter is present only if the <code>stateMachineArn</code> specified in input is a qualified state machine ARN.</p>
    pub fn set_label(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.label = input;
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
    /// Consumes the builder and constructs a [`DescribeStateMachineOutput`](crate::operation::describe_state_machine::DescribeStateMachineOutput).
    pub fn build(self) -> crate::operation::describe_state_machine::DescribeStateMachineOutput {
        crate::operation::describe_state_machine::DescribeStateMachineOutput {
            state_machine_arn: self.state_machine_arn,
            name: self.name,
            status: self.status,
            definition: self.definition,
            role_arn: self.role_arn,
            r#type: self.r#type,
            creation_date: self.creation_date,
            logging_configuration: self.logging_configuration,
            tracing_configuration: self.tracing_configuration,
            label: self.label,
            _request_id: self._request_id,
        }
    }
}
impl ::std::fmt::Debug for DescribeStateMachineOutputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeStateMachineOutputBuilder");
        formatter.field("state_machine_arn", &self.state_machine_arn);
        formatter.field("name", &self.name);
        formatter.field("status", &self.status);
        formatter.field("definition", &"*** Sensitive Data Redacted ***");
        formatter.field("role_arn", &self.role_arn);
        formatter.field("r#type", &self.r#type);
        formatter.field("creation_date", &self.creation_date);
        formatter.field("logging_configuration", &self.logging_configuration);
        formatter.field("tracing_configuration", &self.tracing_configuration);
        formatter.field("label", &self.label);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
