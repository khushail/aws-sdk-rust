// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateRule`](crate::operation::create_rule::builders::CreateRuleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`listener_arn(impl ::std::convert::Into<String>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::listener_arn) / [`set_listener_arn(Option<String>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::set_listener_arn): <p>The Amazon Resource Name (ARN) of the listener.</p>
    ///   - [`conditions(Vec<RuleCondition>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::conditions) / [`set_conditions(Option<Vec<RuleCondition>>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::set_conditions): <p>The conditions.</p>
    ///   - [`priority(i32)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::priority) / [`set_priority(Option<i32>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::set_priority): <p>The rule priority. A listener can't have multiple rules with the same priority.</p>
    ///   - [`actions(Vec<Action>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::actions) / [`set_actions(Option<Vec<Action>>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::set_actions): <p>The actions.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::set_tags): <p>The tags to assign to the rule.</p>
    /// - On success, responds with [`CreateRuleOutput`](crate::operation::create_rule::CreateRuleOutput) with field(s):
    ///   - [`rules(Option<Vec<Rule>>)`](crate::operation::create_rule::CreateRuleOutput::rules): <p>Information about the rule.</p>
    /// - On failure, responds with [`SdkError<CreateRuleError>`](crate::operation::create_rule::CreateRuleError)
    pub fn create_rule(&self) -> crate::operation::create_rule::builders::CreateRuleFluentBuilder {
        crate::operation::create_rule::builders::CreateRuleFluentBuilder::new(self.handle.clone())
    }
}
