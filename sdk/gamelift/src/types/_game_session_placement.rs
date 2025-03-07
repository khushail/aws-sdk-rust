// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>This object includes the full details of the original request plus the current status and start/end time stamps.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GameSessionPlacement {
    /// <p>A unique identifier for a game session placement.</p>
    #[doc(hidden)]
    pub placement_id: ::std::option::Option<::std::string::String>,
    /// <p>A descriptive label that is associated with game session queue. Queue names must be unique within each Region.</p>
    #[doc(hidden)]
    pub game_session_queue_name: ::std::option::Option<::std::string::String>,
    /// <p>Current status of the game session placement request.</p>
    /// <ul>
    /// <li> <p> <b>PENDING</b> -- The placement request is currently in the queue waiting to be processed.</p> </li>
    /// <li> <p> <b>FULFILLED</b> -- A new game session and player sessions (if requested) have been successfully created. Values for <i>GameSessionArn</i> and <i>GameSessionRegion</i> are available. </p> </li>
    /// <li> <p> <b>CANCELLED</b> -- The placement request was canceled.</p> </li>
    /// <li> <p> <b>TIMED_OUT</b> -- A new game session was not successfully created before the time limit expired. You can resubmit the placement request as needed.</p> </li>
    /// <li> <p> <b>FAILED</b> -- Amazon GameLift is not able to complete the process of placing the game session. Common reasons are the game session terminated before the placement process was completed, or an unexpected internal error.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::GameSessionPlacementState>,
    /// <p>A set of custom properties for a game session, formatted as key:value pairs. These properties are passed to a game server process with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>).</p>
    #[doc(hidden)]
    pub game_properties: ::std::option::Option<::std::vec::Vec<crate::types::GameProperty>>,
    /// <p>The maximum number of players that can be connected simultaneously to the game session.</p>
    #[doc(hidden)]
    pub maximum_player_session_count: ::std::option::Option<i32>,
    /// <p>A descriptive label that is associated with a game session. Session names do not need to be unique.</p>
    #[doc(hidden)]
    pub game_session_name: ::std::option::Option<::std::string::String>,
    /// <p>A unique identifier for the game session. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>
    #[doc(hidden)]
    pub game_session_id: ::std::option::Option<::std::string::String>,
    /// <p>Identifier for the game session created by this placement request. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>). This identifier is unique across all Regions. You can use this value as a <code>GameSessionId</code> value as needed.</p>
    #[doc(hidden)]
    pub game_session_arn: ::std::option::Option<::std::string::String>,
    /// <p>Name of the Region where the game session created by this placement request is running. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>
    #[doc(hidden)]
    pub game_session_region: ::std::option::Option<::std::string::String>,
    /// <p>A set of values, expressed in milliseconds, that indicates the amount of latency that a player experiences when connected to Amazon Web Services Regions.</p>
    #[doc(hidden)]
    pub player_latencies: ::std::option::Option<::std::vec::Vec<crate::types::PlayerLatency>>,
    /// <p>Time stamp indicating when this request was placed in the queue. Format is a number expressed in Unix time as milliseconds (for example <code>"1469498468.057"</code>).</p>
    #[doc(hidden)]
    pub start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Time stamp indicating when this request was completed, canceled, or timed out.</p>
    #[doc(hidden)]
    pub end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The IP address of the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>). </p>
    #[doc(hidden)]
    pub ip_address: ::std::option::Option<::std::string::String>,
    /// <p>The DNS identifier assigned to the instance that is running the game session. Values have the following format:</p>
    /// <ul>
    /// <li> <p>TLS-enabled fleets: <code>
    /// <unique identifier>
    /// .
    /// <region identifier>
    /// .amazongamelift.com
    /// </region>
    /// </unique></code>.</p> </li>
    /// <li> <p>Non-TLS-enabled fleets: <code>ec2-
    /// <unique identifier>
    /// .compute.amazonaws.com
    /// </unique></code>. (See <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-instance-addressing.html#concepts-public-addresses">Amazon EC2 Instance IP Addressing</a>.)</p> </li>
    /// </ul>
    /// <p>When connecting to a game session that is running on a TLS-enabled fleet, you must use the DNS name, not the IP address.</p>
    #[doc(hidden)]
    pub dns_name: ::std::option::Option<::std::string::String>,
    /// <p>The port number for the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>
    #[doc(hidden)]
    pub port: ::std::option::Option<i32>,
    /// <p>A collection of information on player sessions created in response to the game session placement request. These player sessions are created only once a new game session is successfully placed (placement status is <code>FULFILLED</code>). This information includes the player ID (as provided in the placement request) and the corresponding player session ID.</p>
    #[doc(hidden)]
    pub placed_player_sessions:
        ::std::option::Option<::std::vec::Vec<crate::types::PlacedPlayerSession>>,
    /// <p>A set of custom game session properties, formatted as a single string value. This data is passed to a game server process in the <code>GameSession</code> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>).</p>
    #[doc(hidden)]
    pub game_session_data: ::std::option::Option<::std::string::String>,
    /// <p>Information on the matchmaking process for this game. Data is in JSON syntax, formatted as a string. It identifies the matchmaking configuration used to create the match, and contains data on all players assigned to the match, including player attributes and team assignments. For more details on matchmaker data, see <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-server.html#match-server-data">Match Data</a>.</p>
    #[doc(hidden)]
    pub matchmaker_data: ::std::option::Option<::std::string::String>,
}
impl GameSessionPlacement {
    /// <p>A unique identifier for a game session placement.</p>
    pub fn placement_id(&self) -> ::std::option::Option<&str> {
        self.placement_id.as_deref()
    }
    /// <p>A descriptive label that is associated with game session queue. Queue names must be unique within each Region.</p>
    pub fn game_session_queue_name(&self) -> ::std::option::Option<&str> {
        self.game_session_queue_name.as_deref()
    }
    /// <p>Current status of the game session placement request.</p>
    /// <ul>
    /// <li> <p> <b>PENDING</b> -- The placement request is currently in the queue waiting to be processed.</p> </li>
    /// <li> <p> <b>FULFILLED</b> -- A new game session and player sessions (if requested) have been successfully created. Values for <i>GameSessionArn</i> and <i>GameSessionRegion</i> are available. </p> </li>
    /// <li> <p> <b>CANCELLED</b> -- The placement request was canceled.</p> </li>
    /// <li> <p> <b>TIMED_OUT</b> -- A new game session was not successfully created before the time limit expired. You can resubmit the placement request as needed.</p> </li>
    /// <li> <p> <b>FAILED</b> -- Amazon GameLift is not able to complete the process of placing the game session. Common reasons are the game session terminated before the placement process was completed, or an unexpected internal error.</p> </li>
    /// </ul>
    pub fn status(&self) -> ::std::option::Option<&crate::types::GameSessionPlacementState> {
        self.status.as_ref()
    }
    /// <p>A set of custom properties for a game session, formatted as key:value pairs. These properties are passed to a game server process with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>).</p>
    pub fn game_properties(&self) -> ::std::option::Option<&[crate::types::GameProperty]> {
        self.game_properties.as_deref()
    }
    /// <p>The maximum number of players that can be connected simultaneously to the game session.</p>
    pub fn maximum_player_session_count(&self) -> ::std::option::Option<i32> {
        self.maximum_player_session_count
    }
    /// <p>A descriptive label that is associated with a game session. Session names do not need to be unique.</p>
    pub fn game_session_name(&self) -> ::std::option::Option<&str> {
        self.game_session_name.as_deref()
    }
    /// <p>A unique identifier for the game session. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>
    pub fn game_session_id(&self) -> ::std::option::Option<&str> {
        self.game_session_id.as_deref()
    }
    /// <p>Identifier for the game session created by this placement request. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>). This identifier is unique across all Regions. You can use this value as a <code>GameSessionId</code> value as needed.</p>
    pub fn game_session_arn(&self) -> ::std::option::Option<&str> {
        self.game_session_arn.as_deref()
    }
    /// <p>Name of the Region where the game session created by this placement request is running. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>
    pub fn game_session_region(&self) -> ::std::option::Option<&str> {
        self.game_session_region.as_deref()
    }
    /// <p>A set of values, expressed in milliseconds, that indicates the amount of latency that a player experiences when connected to Amazon Web Services Regions.</p>
    pub fn player_latencies(&self) -> ::std::option::Option<&[crate::types::PlayerLatency]> {
        self.player_latencies.as_deref()
    }
    /// <p>Time stamp indicating when this request was placed in the queue. Format is a number expressed in Unix time as milliseconds (for example <code>"1469498468.057"</code>).</p>
    pub fn start_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.start_time.as_ref()
    }
    /// <p>Time stamp indicating when this request was completed, canceled, or timed out.</p>
    pub fn end_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.end_time.as_ref()
    }
    /// <p>The IP address of the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>). </p>
    pub fn ip_address(&self) -> ::std::option::Option<&str> {
        self.ip_address.as_deref()
    }
    /// <p>The DNS identifier assigned to the instance that is running the game session. Values have the following format:</p>
    /// <ul>
    /// <li> <p>TLS-enabled fleets: <code>
    /// <unique identifier>
    /// .
    /// <region identifier>
    /// .amazongamelift.com
    /// </region>
    /// </unique></code>.</p> </li>
    /// <li> <p>Non-TLS-enabled fleets: <code>ec2-
    /// <unique identifier>
    /// .compute.amazonaws.com
    /// </unique></code>. (See <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-instance-addressing.html#concepts-public-addresses">Amazon EC2 Instance IP Addressing</a>.)</p> </li>
    /// </ul>
    /// <p>When connecting to a game session that is running on a TLS-enabled fleet, you must use the DNS name, not the IP address.</p>
    pub fn dns_name(&self) -> ::std::option::Option<&str> {
        self.dns_name.as_deref()
    }
    /// <p>The port number for the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>
    pub fn port(&self) -> ::std::option::Option<i32> {
        self.port
    }
    /// <p>A collection of information on player sessions created in response to the game session placement request. These player sessions are created only once a new game session is successfully placed (placement status is <code>FULFILLED</code>). This information includes the player ID (as provided in the placement request) and the corresponding player session ID.</p>
    pub fn placed_player_sessions(
        &self,
    ) -> ::std::option::Option<&[crate::types::PlacedPlayerSession]> {
        self.placed_player_sessions.as_deref()
    }
    /// <p>A set of custom game session properties, formatted as a single string value. This data is passed to a game server process in the <code>GameSession</code> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>).</p>
    pub fn game_session_data(&self) -> ::std::option::Option<&str> {
        self.game_session_data.as_deref()
    }
    /// <p>Information on the matchmaking process for this game. Data is in JSON syntax, formatted as a string. It identifies the matchmaking configuration used to create the match, and contains data on all players assigned to the match, including player attributes and team assignments. For more details on matchmaker data, see <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-server.html#match-server-data">Match Data</a>.</p>
    pub fn matchmaker_data(&self) -> ::std::option::Option<&str> {
        self.matchmaker_data.as_deref()
    }
}
impl GameSessionPlacement {
    /// Creates a new builder-style object to manufacture [`GameSessionPlacement`](crate::types::GameSessionPlacement).
    pub fn builder() -> crate::types::builders::GameSessionPlacementBuilder {
        crate::types::builders::GameSessionPlacementBuilder::default()
    }
}

/// A builder for [`GameSessionPlacement`](crate::types::GameSessionPlacement).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GameSessionPlacementBuilder {
    pub(crate) placement_id: ::std::option::Option<::std::string::String>,
    pub(crate) game_session_queue_name: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::GameSessionPlacementState>,
    pub(crate) game_properties: ::std::option::Option<::std::vec::Vec<crate::types::GameProperty>>,
    pub(crate) maximum_player_session_count: ::std::option::Option<i32>,
    pub(crate) game_session_name: ::std::option::Option<::std::string::String>,
    pub(crate) game_session_id: ::std::option::Option<::std::string::String>,
    pub(crate) game_session_arn: ::std::option::Option<::std::string::String>,
    pub(crate) game_session_region: ::std::option::Option<::std::string::String>,
    pub(crate) player_latencies:
        ::std::option::Option<::std::vec::Vec<crate::types::PlayerLatency>>,
    pub(crate) start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) ip_address: ::std::option::Option<::std::string::String>,
    pub(crate) dns_name: ::std::option::Option<::std::string::String>,
    pub(crate) port: ::std::option::Option<i32>,
    pub(crate) placed_player_sessions:
        ::std::option::Option<::std::vec::Vec<crate::types::PlacedPlayerSession>>,
    pub(crate) game_session_data: ::std::option::Option<::std::string::String>,
    pub(crate) matchmaker_data: ::std::option::Option<::std::string::String>,
}
impl GameSessionPlacementBuilder {
    /// <p>A unique identifier for a game session placement.</p>
    pub fn placement_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.placement_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for a game session placement.</p>
    pub fn set_placement_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.placement_id = input;
        self
    }
    /// <p>A descriptive label that is associated with game session queue. Queue names must be unique within each Region.</p>
    pub fn game_session_queue_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.game_session_queue_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A descriptive label that is associated with game session queue. Queue names must be unique within each Region.</p>
    pub fn set_game_session_queue_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.game_session_queue_name = input;
        self
    }
    /// <p>Current status of the game session placement request.</p>
    /// <ul>
    /// <li> <p> <b>PENDING</b> -- The placement request is currently in the queue waiting to be processed.</p> </li>
    /// <li> <p> <b>FULFILLED</b> -- A new game session and player sessions (if requested) have been successfully created. Values for <i>GameSessionArn</i> and <i>GameSessionRegion</i> are available. </p> </li>
    /// <li> <p> <b>CANCELLED</b> -- The placement request was canceled.</p> </li>
    /// <li> <p> <b>TIMED_OUT</b> -- A new game session was not successfully created before the time limit expired. You can resubmit the placement request as needed.</p> </li>
    /// <li> <p> <b>FAILED</b> -- Amazon GameLift is not able to complete the process of placing the game session. Common reasons are the game session terminated before the placement process was completed, or an unexpected internal error.</p> </li>
    /// </ul>
    pub fn status(mut self, input: crate::types::GameSessionPlacementState) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Current status of the game session placement request.</p>
    /// <ul>
    /// <li> <p> <b>PENDING</b> -- The placement request is currently in the queue waiting to be processed.</p> </li>
    /// <li> <p> <b>FULFILLED</b> -- A new game session and player sessions (if requested) have been successfully created. Values for <i>GameSessionArn</i> and <i>GameSessionRegion</i> are available. </p> </li>
    /// <li> <p> <b>CANCELLED</b> -- The placement request was canceled.</p> </li>
    /// <li> <p> <b>TIMED_OUT</b> -- A new game session was not successfully created before the time limit expired. You can resubmit the placement request as needed.</p> </li>
    /// <li> <p> <b>FAILED</b> -- Amazon GameLift is not able to complete the process of placing the game session. Common reasons are the game session terminated before the placement process was completed, or an unexpected internal error.</p> </li>
    /// </ul>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::GameSessionPlacementState>,
    ) -> Self {
        self.status = input;
        self
    }
    /// Appends an item to `game_properties`.
    ///
    /// To override the contents of this collection use [`set_game_properties`](Self::set_game_properties).
    ///
    /// <p>A set of custom properties for a game session, formatted as key:value pairs. These properties are passed to a game server process with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>).</p>
    pub fn game_properties(mut self, input: crate::types::GameProperty) -> Self {
        let mut v = self.game_properties.unwrap_or_default();
        v.push(input);
        self.game_properties = ::std::option::Option::Some(v);
        self
    }
    /// <p>A set of custom properties for a game session, formatted as key:value pairs. These properties are passed to a game server process with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>).</p>
    pub fn set_game_properties(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::GameProperty>>,
    ) -> Self {
        self.game_properties = input;
        self
    }
    /// <p>The maximum number of players that can be connected simultaneously to the game session.</p>
    pub fn maximum_player_session_count(mut self, input: i32) -> Self {
        self.maximum_player_session_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of players that can be connected simultaneously to the game session.</p>
    pub fn set_maximum_player_session_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.maximum_player_session_count = input;
        self
    }
    /// <p>A descriptive label that is associated with a game session. Session names do not need to be unique.</p>
    pub fn game_session_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.game_session_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A descriptive label that is associated with a game session. Session names do not need to be unique.</p>
    pub fn set_game_session_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.game_session_name = input;
        self
    }
    /// <p>A unique identifier for the game session. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>
    pub fn game_session_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.game_session_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the game session. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>
    pub fn set_game_session_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.game_session_id = input;
        self
    }
    /// <p>Identifier for the game session created by this placement request. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>). This identifier is unique across all Regions. You can use this value as a <code>GameSessionId</code> value as needed.</p>
    pub fn game_session_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.game_session_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Identifier for the game session created by this placement request. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>). This identifier is unique across all Regions. You can use this value as a <code>GameSessionId</code> value as needed.</p>
    pub fn set_game_session_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.game_session_arn = input;
        self
    }
    /// <p>Name of the Region where the game session created by this placement request is running. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>
    pub fn game_session_region(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.game_session_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name of the Region where the game session created by this placement request is running. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>
    pub fn set_game_session_region(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.game_session_region = input;
        self
    }
    /// Appends an item to `player_latencies`.
    ///
    /// To override the contents of this collection use [`set_player_latencies`](Self::set_player_latencies).
    ///
    /// <p>A set of values, expressed in milliseconds, that indicates the amount of latency that a player experiences when connected to Amazon Web Services Regions.</p>
    pub fn player_latencies(mut self, input: crate::types::PlayerLatency) -> Self {
        let mut v = self.player_latencies.unwrap_or_default();
        v.push(input);
        self.player_latencies = ::std::option::Option::Some(v);
        self
    }
    /// <p>A set of values, expressed in milliseconds, that indicates the amount of latency that a player experiences when connected to Amazon Web Services Regions.</p>
    pub fn set_player_latencies(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PlayerLatency>>,
    ) -> Self {
        self.player_latencies = input;
        self
    }
    /// <p>Time stamp indicating when this request was placed in the queue. Format is a number expressed in Unix time as milliseconds (for example <code>"1469498468.057"</code>).</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>Time stamp indicating when this request was placed in the queue. Format is a number expressed in Unix time as milliseconds (for example <code>"1469498468.057"</code>).</p>
    pub fn set_start_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.start_time = input;
        self
    }
    /// <p>Time stamp indicating when this request was completed, canceled, or timed out.</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.end_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>Time stamp indicating when this request was completed, canceled, or timed out.</p>
    pub fn set_end_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.end_time = input;
        self
    }
    /// <p>The IP address of the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>). </p>
    pub fn ip_address(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ip_address = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IP address of the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>). </p>
    pub fn set_ip_address(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ip_address = input;
        self
    }
    /// <p>The DNS identifier assigned to the instance that is running the game session. Values have the following format:</p>
    /// <ul>
    /// <li> <p>TLS-enabled fleets: <code>
    /// <unique identifier>
    /// .
    /// <region identifier>
    /// .amazongamelift.com
    /// </region>
    /// </unique></code>.</p> </li>
    /// <li> <p>Non-TLS-enabled fleets: <code>ec2-
    /// <unique identifier>
    /// .compute.amazonaws.com
    /// </unique></code>. (See <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-instance-addressing.html#concepts-public-addresses">Amazon EC2 Instance IP Addressing</a>.)</p> </li>
    /// </ul>
    /// <p>When connecting to a game session that is running on a TLS-enabled fleet, you must use the DNS name, not the IP address.</p>
    pub fn dns_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.dns_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The DNS identifier assigned to the instance that is running the game session. Values have the following format:</p>
    /// <ul>
    /// <li> <p>TLS-enabled fleets: <code>
    /// <unique identifier>
    /// .
    /// <region identifier>
    /// .amazongamelift.com
    /// </region>
    /// </unique></code>.</p> </li>
    /// <li> <p>Non-TLS-enabled fleets: <code>ec2-
    /// <unique identifier>
    /// .compute.amazonaws.com
    /// </unique></code>. (See <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-instance-addressing.html#concepts-public-addresses">Amazon EC2 Instance IP Addressing</a>.)</p> </li>
    /// </ul>
    /// <p>When connecting to a game session that is running on a TLS-enabled fleet, you must use the DNS name, not the IP address.</p>
    pub fn set_dns_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.dns_name = input;
        self
    }
    /// <p>The port number for the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>
    pub fn port(mut self, input: i32) -> Self {
        self.port = ::std::option::Option::Some(input);
        self
    }
    /// <p>The port number for the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>
    pub fn set_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.port = input;
        self
    }
    /// Appends an item to `placed_player_sessions`.
    ///
    /// To override the contents of this collection use [`set_placed_player_sessions`](Self::set_placed_player_sessions).
    ///
    /// <p>A collection of information on player sessions created in response to the game session placement request. These player sessions are created only once a new game session is successfully placed (placement status is <code>FULFILLED</code>). This information includes the player ID (as provided in the placement request) and the corresponding player session ID.</p>
    pub fn placed_player_sessions(mut self, input: crate::types::PlacedPlayerSession) -> Self {
        let mut v = self.placed_player_sessions.unwrap_or_default();
        v.push(input);
        self.placed_player_sessions = ::std::option::Option::Some(v);
        self
    }
    /// <p>A collection of information on player sessions created in response to the game session placement request. These player sessions are created only once a new game session is successfully placed (placement status is <code>FULFILLED</code>). This information includes the player ID (as provided in the placement request) and the corresponding player session ID.</p>
    pub fn set_placed_player_sessions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PlacedPlayerSession>>,
    ) -> Self {
        self.placed_player_sessions = input;
        self
    }
    /// <p>A set of custom game session properties, formatted as a single string value. This data is passed to a game server process in the <code>GameSession</code> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>).</p>
    pub fn game_session_data(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.game_session_data = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A set of custom game session properties, formatted as a single string value. This data is passed to a game server process in the <code>GameSession</code> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>).</p>
    pub fn set_game_session_data(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.game_session_data = input;
        self
    }
    /// <p>Information on the matchmaking process for this game. Data is in JSON syntax, formatted as a string. It identifies the matchmaking configuration used to create the match, and contains data on all players assigned to the match, including player attributes and team assignments. For more details on matchmaker data, see <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-server.html#match-server-data">Match Data</a>.</p>
    pub fn matchmaker_data(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.matchmaker_data = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Information on the matchmaking process for this game. Data is in JSON syntax, formatted as a string. It identifies the matchmaking configuration used to create the match, and contains data on all players assigned to the match, including player attributes and team assignments. For more details on matchmaker data, see <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-server.html#match-server-data">Match Data</a>.</p>
    pub fn set_matchmaker_data(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.matchmaker_data = input;
        self
    }
    /// Consumes the builder and constructs a [`GameSessionPlacement`](crate::types::GameSessionPlacement).
    pub fn build(self) -> crate::types::GameSessionPlacement {
        crate::types::GameSessionPlacement {
            placement_id: self.placement_id,
            game_session_queue_name: self.game_session_queue_name,
            status: self.status,
            game_properties: self.game_properties,
            maximum_player_session_count: self.maximum_player_session_count,
            game_session_name: self.game_session_name,
            game_session_id: self.game_session_id,
            game_session_arn: self.game_session_arn,
            game_session_region: self.game_session_region,
            player_latencies: self.player_latencies,
            start_time: self.start_time,
            end_time: self.end_time,
            ip_address: self.ip_address,
            dns_name: self.dns_name,
            port: self.port,
            placed_player_sessions: self.placed_player_sessions,
            game_session_data: self.game_session_data,
            matchmaker_data: self.matchmaker_data,
        }
    }
}
