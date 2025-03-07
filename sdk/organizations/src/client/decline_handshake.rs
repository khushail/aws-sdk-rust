// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeclineHandshake`](crate::operation::decline_handshake::builders::DeclineHandshakeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`handshake_id(impl ::std::convert::Into<String>)`](crate::operation::decline_handshake::builders::DeclineHandshakeFluentBuilder::handshake_id) / [`set_handshake_id(Option<String>)`](crate::operation::decline_handshake::builders::DeclineHandshakeFluentBuilder::set_handshake_id): <p>The unique identifier (ID) of the handshake that you want to decline. You can get the ID from the <code>ListHandshakesForAccount</code> operation.</p>  <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lowercase letters or digits.</p>
    /// - On success, responds with [`DeclineHandshakeOutput`](crate::operation::decline_handshake::DeclineHandshakeOutput) with field(s):
    ///   - [`handshake(Option<Handshake>)`](crate::operation::decline_handshake::DeclineHandshakeOutput::handshake): <p>A structure that contains details about the declined handshake. The state is updated to show the value <code>DECLINED</code>.</p>
    /// - On failure, responds with [`SdkError<DeclineHandshakeError>`](crate::operation::decline_handshake::DeclineHandshakeError)
    pub fn decline_handshake(
        &self,
    ) -> crate::operation::decline_handshake::builders::DeclineHandshakeFluentBuilder {
        crate::operation::decline_handshake::builders::DeclineHandshakeFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
