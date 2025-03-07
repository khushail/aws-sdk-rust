// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetDigest`](crate::operation::get_digest::builders::GetDigestFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::get_digest::builders::GetDigestFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::get_digest::builders::GetDigestFluentBuilder::set_name): <p>The name of the ledger.</p>
    /// - On success, responds with [`GetDigestOutput`](crate::operation::get_digest::GetDigestOutput) with field(s):
    ///   - [`digest(Option<Blob>)`](crate::operation::get_digest::GetDigestOutput::digest): <p>The 256-bit hash value representing the digest returned by a <code>GetDigest</code> request.</p>
    ///   - [`digest_tip_address(Option<ValueHolder>)`](crate::operation::get_digest::GetDigestOutput::digest_tip_address): <p>The latest block location covered by the digest that you requested. An address is an Amazon Ion structure that has two fields: <code>strandId</code> and <code>sequenceNo</code>.</p>
    /// - On failure, responds with [`SdkError<GetDigestError>`](crate::operation::get_digest::GetDigestError)
    pub fn get_digest(&self) -> crate::operation::get_digest::builders::GetDigestFluentBuilder {
        crate::operation::get_digest::builders::GetDigestFluentBuilder::new(self.handle.clone())
    }
}
