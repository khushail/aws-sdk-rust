// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeGameSessionPlacement`](crate::operation::describe_game_session_placement::builders::DescribeGameSessionPlacementFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`placement_id(impl ::std::convert::Into<String>)`](crate::operation::describe_game_session_placement::builders::DescribeGameSessionPlacementFluentBuilder::placement_id) / [`set_placement_id(Option<String>)`](crate::operation::describe_game_session_placement::builders::DescribeGameSessionPlacementFluentBuilder::set_placement_id): <p>A unique identifier for a game session placement to retrieve.</p>
    /// - On success, responds with [`DescribeGameSessionPlacementOutput`](crate::operation::describe_game_session_placement::DescribeGameSessionPlacementOutput) with field(s):
    ///   - [`game_session_placement(Option<GameSessionPlacement>)`](crate::operation::describe_game_session_placement::DescribeGameSessionPlacementOutput::game_session_placement): <p>Object that describes the requested game session placement.</p>
    /// - On failure, responds with [`SdkError<DescribeGameSessionPlacementError>`](crate::operation::describe_game_session_placement::DescribeGameSessionPlacementError)
    pub fn describe_game_session_placement(&self) -> crate::operation::describe_game_session_placement::builders::DescribeGameSessionPlacementFluentBuilder{
        crate::operation::describe_game_session_placement::builders::DescribeGameSessionPlacementFluentBuilder::new(self.handle.clone())
    }
}
