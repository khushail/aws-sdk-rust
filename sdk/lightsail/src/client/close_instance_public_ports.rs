// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CloseInstancePublicPorts`](crate::operation::close_instance_public_ports::builders::CloseInstancePublicPortsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`port_info(PortInfo)`](crate::operation::close_instance_public_ports::builders::CloseInstancePublicPortsFluentBuilder::port_info) / [`set_port_info(Option<PortInfo>)`](crate::operation::close_instance_public_ports::builders::CloseInstancePublicPortsFluentBuilder::set_port_info): <p>An object to describe the ports to close for the specified instance.</p>
    ///   - [`instance_name(impl ::std::convert::Into<String>)`](crate::operation::close_instance_public_ports::builders::CloseInstancePublicPortsFluentBuilder::instance_name) / [`set_instance_name(Option<String>)`](crate::operation::close_instance_public_ports::builders::CloseInstancePublicPortsFluentBuilder::set_instance_name): <p>The name of the instance for which to close ports.</p>
    /// - On success, responds with [`CloseInstancePublicPortsOutput`](crate::operation::close_instance_public_ports::CloseInstancePublicPortsOutput) with field(s):
    ///   - [`operation(Option<Operation>)`](crate::operation::close_instance_public_ports::CloseInstancePublicPortsOutput::operation): <p>An object that describes the result of the action, such as the status of the request, the timestamp of the request, and the resources affected by the request.</p>
    /// - On failure, responds with [`SdkError<CloseInstancePublicPortsError>`](crate::operation::close_instance_public_ports::CloseInstancePublicPortsError)
    pub fn close_instance_public_ports(&self) -> crate::operation::close_instance_public_ports::builders::CloseInstancePublicPortsFluentBuilder{
        crate::operation::close_instance_public_ports::builders::CloseInstancePublicPortsFluentBuilder::new(self.handle.clone())
    }
}
