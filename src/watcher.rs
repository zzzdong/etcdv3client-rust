use tokio::sync::mpsc::UnboundedSender;
use tonic::codec::Streaming;

use crate::pb::{watch_request, WatchCancelRequest, WatchRequest, WatchResponse};
use crate::{error::WatchError, EtcdClientError};

pub struct Watcher {
    watch_id: i64,
    req_tx: UnboundedSender<WatchRequest>,
    inbound: Streaming<crate::pb::WatchResponse>,
}

impl Watcher {
    pub(crate) fn new(
        watch_id: i64,
        req_tx: UnboundedSender<WatchRequest>,
        inbound: Streaming<crate::pb::WatchResponse>,
    ) -> Watcher {
        Watcher {
            watch_id,
            req_tx,
            inbound,
        }
    }

    pub async fn cancel(&mut self) -> Result<(), EtcdClientError> {
        let cancel_watch = watch_request::RequestUnion::CancelRequest(WatchCancelRequest {
            watch_id: self.watch_id,
        });
        let cancel_req = WatchRequest {
            request_union: Some(cancel_watch),
        };

        self.req_tx
            .send(cancel_req)
            .map_err(WatchError::WatchRequestError)?;

        Ok(())
    }

    pub async fn message(&mut self) -> Result<Option<WatchResponse>, EtcdClientError> {
        match self.inbound.message().await? {
            Some(resp) => Ok(Some(resp)),
            None => Ok(None),
        }
    }
}
