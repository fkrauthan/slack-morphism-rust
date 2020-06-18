//!
//! Support for Slack Chat API methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::scroller::*;
use crate::ClientResult;
use crate::SlackClientSession;
use futures::future::{BoxFuture, FutureExt};
use slack_morphism_models::*;

impl<'a> SlackClientSession<'a> {
    ///
    /// https://api.slack.com/methods/chat.delete
    ///
    pub async fn chat_delete(
        &self,
        req: &SlackApiChatDeleteRequest,
    ) -> ClientResult<SlackApiChatDeleteResponse> {
        self.http_api.http_post("chat.delete", req).await
    }

    ///
    /// https://api.slack.com/methods/chat.deleteScheduledMessage
    ///
    pub async fn chat_delete_scheduled_message(
        &self,
        req: &SlackApiChatDeleteScheduledMessageRequest,
    ) -> ClientResult<SlackApiChatDeleteScheduledMessageResponse> {
        self.http_api
            .http_post("chat.deleteScheduledMessage", req)
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatDeleteRequest {
    pub channel: SlackChannelId,
    pub ts: SlackTs,
    pub as_user: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatDeleteResponse {
    pub channel: SlackChannelId,
    pub ts: SlackTs,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatDeleteScheduledMessageRequest {
    pub channel: SlackChannelId,
    pub scheduled_message: SlackTs,
    pub as_user: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatDeleteScheduledMessageResponse {}
