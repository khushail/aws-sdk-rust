// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateBlueprint`](crate::operation::update_blueprint::builders::UpdateBlueprintFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_blueprint::builders::UpdateBlueprintFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_blueprint::builders::UpdateBlueprintFluentBuilder::set_name): <p>The name of the blueprint.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_blueprint::builders::UpdateBlueprintFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_blueprint::builders::UpdateBlueprintFluentBuilder::set_description): <p>A description of the blueprint.</p>
    ///   - [`blueprint_location(impl ::std::convert::Into<String>)`](crate::operation::update_blueprint::builders::UpdateBlueprintFluentBuilder::blueprint_location) / [`set_blueprint_location(Option<String>)`](crate::operation::update_blueprint::builders::UpdateBlueprintFluentBuilder::set_blueprint_location): <p>Specifies a path in Amazon S3 where the blueprint is published.</p>
    /// - On success, responds with [`UpdateBlueprintOutput`](crate::operation::update_blueprint::UpdateBlueprintOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::operation::update_blueprint::UpdateBlueprintOutput::name): <p>Returns the name of the blueprint that was updated.</p>
    /// - On failure, responds with [`SdkError<UpdateBlueprintError>`](crate::operation::update_blueprint::UpdateBlueprintError)
    pub fn update_blueprint(
        &self,
    ) -> crate::operation::update_blueprint::builders::UpdateBlueprintFluentBuilder {
        crate::operation::update_blueprint::builders::UpdateBlueprintFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
