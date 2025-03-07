// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetLayerVersion`](crate::operation::get_layer_version::builders::GetLayerVersionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`layer_name(impl ::std::convert::Into<String>)`](crate::operation::get_layer_version::builders::GetLayerVersionFluentBuilder::layer_name) / [`set_layer_name(Option<String>)`](crate::operation::get_layer_version::builders::GetLayerVersionFluentBuilder::set_layer_name): <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    ///   - [`version_number(i64)`](crate::operation::get_layer_version::builders::GetLayerVersionFluentBuilder::version_number) / [`set_version_number(Option<i64>)`](crate::operation::get_layer_version::builders::GetLayerVersionFluentBuilder::set_version_number): <p>The version number.</p>
    /// - On success, responds with [`GetLayerVersionOutput`](crate::operation::get_layer_version::GetLayerVersionOutput) with field(s):
    ///   - [`content(Option<LayerVersionContentOutput>)`](crate::operation::get_layer_version::GetLayerVersionOutput::content): <p>Details about the layer version.</p>
    ///   - [`layer_arn(Option<String>)`](crate::operation::get_layer_version::GetLayerVersionOutput::layer_arn): <p>The ARN of the layer.</p>
    ///   - [`layer_version_arn(Option<String>)`](crate::operation::get_layer_version::GetLayerVersionOutput::layer_version_arn): <p>The ARN of the layer version.</p>
    ///   - [`description(Option<String>)`](crate::operation::get_layer_version::GetLayerVersionOutput::description): <p>The description of the version.</p>
    ///   - [`created_date(Option<String>)`](crate::operation::get_layer_version::GetLayerVersionOutput::created_date): <p>The date that the layer version was created, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    ///   - [`version(i64)`](crate::operation::get_layer_version::GetLayerVersionOutput::version): <p>The version number.</p>
    ///   - [`compatible_runtimes(Option<Vec<Runtime>>)`](crate::operation::get_layer_version::GetLayerVersionOutput::compatible_runtimes): <p>The layer's compatible runtimes.</p>
    ///   - [`license_info(Option<String>)`](crate::operation::get_layer_version::GetLayerVersionOutput::license_info): <p>The layer's software license.</p>
    ///   - [`compatible_architectures(Option<Vec<Architecture>>)`](crate::operation::get_layer_version::GetLayerVersionOutput::compatible_architectures): <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
    /// - On failure, responds with [`SdkError<GetLayerVersionError>`](crate::operation::get_layer_version::GetLayerVersionError)
    pub fn get_layer_version(
        &self,
    ) -> crate::operation::get_layer_version::builders::GetLayerVersionFluentBuilder {
        crate::operation::get_layer_version::builders::GetLayerVersionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
