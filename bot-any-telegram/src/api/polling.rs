use std::{cmp, collections::VecDeque, pin::Pin, task::Poll};

use apid_telegram_bot::{
    calls::GetUpdates,
    types::{Update, UpdateEvent},
};
use futures_lite::Stream;
use std::future::Future;

use crate::bridge::telegram_client_request::TelegramClientRequest;

use super::TelegramClient;

pub struct LongPolling<'a, F, Fut, Error>
where
    F: Fn(TelegramClientRequest<GetUpdates>) -> Fut,
    F: Unpin,
    Fut: Future<Output = Result<Vec<Update>, Error>>,
{
    call: F,
    client: &'a TelegramClient<'a>,
    queue: VecDeque<Result<UpdateEvent, Error>>,
    fut: Option<Pin<Box<Fut>>>,
    offset: Option<i32>,
}

impl<'a, F, Fut, Error> LongPolling<'a, F, Fut, Error>
where
    F: Fn(TelegramClientRequest<GetUpdates>) -> Fut,
    F: Unpin,
    Fut: Future<Output = Result<Vec<Update>, Error>>,
{
    pub fn new(call: F, client: &'a TelegramClient<'a>) -> Self {
        Self {
            call,
            client,
            queue: VecDeque::new(),
            fut: None,
            offset: None,
        }
    }
}

impl<F, Fut, Error> Stream for LongPolling<'_, F, Fut, Error>
where
    F: Fn(TelegramClientRequest<GetUpdates>) -> Fut,
    F: Unpin,
    Fut: Future<Output = Result<Vec<Update>, Error>>,
    Error: Unpin,
{
    type Item = Result<UpdateEvent, Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match self.queue.pop_front() {
            Some(event) => Poll::Ready(Some(event)),
            None => match &mut self.fut {
                Some(fut) => match fut.as_mut().poll(cx) {
                    Poll::Ready(res) => {
                        self.fut = None;
                        match res {
                            Ok(updates) => {
                                for update in updates {
                                    if let Some(event) = update.event {
                                        self.queue.push_back(Ok(event));
                                    }
                                    self.offset = Some(cmp::max(
                                        self.offset.unwrap_or(i32::MIN),
                                        update.update_id + 1,
                                    ));
                                }
                            }
                            Err(err) => {
                                self.queue.push_back(Err(err));
                            }
                        }
                        self.poll_next(cx)
                    }
                    Poll::Pending => Poll::Pending,
                },
                None => {
                    let fut = Box::pin((self.call)(self.client.get_updates(self.offset)));
                    self.fut = Some(fut);
                    self.poll_next(cx)
                }
            },
        }
    }
}
