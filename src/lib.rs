use frankenstein::{AsyncTelegramApi, GetUpdatesParams};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct Listener<'a, H: FnMut(frankenstein::Update)> {
    params: GetUpdatesParams,
    client: &'a frankenstein::AsyncApi,
    handler: H,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("an error occured while getting updates: {source}")]
    UpdatesGettingError { source: frankenstein::Error },
}

impl<'a, H: FnMut(frankenstein::Update)> Listener<'a, H> {
    pub fn new(params: GetUpdatesParams, client: &'a frankenstein::AsyncApi, handler: H) -> Self {
        Self {
            params,
            client,
            handler,
        }
    }

    pub async fn listen(&mut self) -> Result<(), Error> {
        loop {
            let updates = self
                .client
                .get_updates(&self.params)
                .await
                .map_err(|source| Error::UpdatesGettingError { source })?;

            if let Some(new_offset) = updates
                .result
                .iter()
                .map(|update| update.update_id)
                .max()
                .map(|max_update_id| (max_update_id + 1).into())
            {
                self.params.offset = Some(new_offset);
            }

            for update in updates.result {
                (self.handler)(update);
            }
        }
    }

    pub fn params(&self) -> &GetUpdatesParams {
        &self.params
    }
}
