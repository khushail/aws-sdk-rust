// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateRecommendationTemplate`](crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`recommendation_ids(Vec<String>)`](crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder::recommendation_ids) / [`set_recommendation_ids(Option<Vec<String>>)`](crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder::set_recommendation_ids): <p>Identifiers for the recommendations used to create a recommendation template.</p>
    ///   - [`format(TemplateFormat)`](crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder::format) / [`set_format(Option<TemplateFormat>)`](crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder::set_format): <p>The format for the recommendation template.</p>  <dl>   <dt>   CfnJson  </dt>   <dd>    <p>The template is CloudFormation JSON.</p>   </dd>   <dt>   CfnYaml  </dt>   <dd>    <p>The template is CloudFormation YAML.</p>   </dd>  </dl>
    ///   - [`recommendation_types(Vec<RenderRecommendationType>)`](crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder::recommendation_types) / [`set_recommendation_types(Option<Vec<RenderRecommendationType>>)`](crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder::set_recommendation_types): <p>An array of strings that specify the recommendation template type or types.</p>  <dl>   <dt>   Alarm  </dt>   <dd>    <p>The template is an <code>AlarmRecommendation</code> template.</p>   </dd>   <dt>   Sop  </dt>   <dd>    <p>The template is a <code>SopRecommendation</code> template.</p>   </dd>   <dt>   Test  </dt>   <dd>    <p>The template is a <code>TestRecommendation</code> template.</p>   </dd>  </dl>
    ///   - [`assessment_arn(impl ::std::convert::Into<String>)`](crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder::assessment_arn) / [`set_assessment_arn(Option<String>)`](crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder::set_assessment_arn): <p>The Amazon Resource Name (ARN) of the assessment. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app-assessment/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder::set_name): <p>The name for the recommendation template.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder::set_client_token): <p>Used for an idempotency token. A client token is a unique, case-sensitive string of up to 64 ASCII characters. You should not reuse the same client token for other API requests.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder::set_tags): <p>The tags assigned to the resource. A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key/value pair.</p>
    ///   - [`bucket_name(impl ::std::convert::Into<String>)`](crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder::bucket_name) / [`set_bucket_name(Option<String>)`](crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder::set_bucket_name): <p>The name of the Amazon S3 bucket that will contain the recommendation template.</p>
    /// - On success, responds with [`CreateRecommendationTemplateOutput`](crate::operation::create_recommendation_template::CreateRecommendationTemplateOutput) with field(s):
    ///   - [`recommendation_template(Option<RecommendationTemplate>)`](crate::operation::create_recommendation_template::CreateRecommendationTemplateOutput::recommendation_template): <p>The newly created recommendation template, returned as an object. This object includes the template's name, format, status, tags, Amazon S3 bucket location, and more.</p>
    /// - On failure, responds with [`SdkError<CreateRecommendationTemplateError>`](crate::operation::create_recommendation_template::CreateRecommendationTemplateError)
    pub fn create_recommendation_template(&self) -> crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder{
        crate::operation::create_recommendation_template::builders::CreateRecommendationTemplateFluentBuilder::new(self.handle.clone())
    }
}
