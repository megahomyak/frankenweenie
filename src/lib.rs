use frankenstein::{AsyncTelegramApi, GetUpdatesParams};

pub struct Listener {
    offset: Option<i64>,
}

pub enum Error {
    UpdatesGettingError { source: frankenstein::Error },
}

impl Listener {
    pub fn new() -> Self {
        Self { offset: None }
    }

    pub fn new_with_offset(offset: Option<i64>) -> Self {
        Self { offset }
    }

    pub async fn listen<H: FnMut(frankenstein::Update)>(
        client: &frankenstein::AsyncApi,
        params: &GetUpdatesParams,
        mut handler: H,
    ) -> Result<(), Error> {
        let mut offset = None;

        loop {
            let updates = client
                .get_updates(&GetUpdatesParams {
                    offset,
                    ..params.clone()
                })
                .await
                .map_err(|source| Error::UpdatesGettingError { source })?;

            offset = updates
                .result
                .iter()
                .map(|update| update.update_id)
                .max()
                .map(|max_update_id| (max_update_id + 1).into());

            for update in updates.result {
                handler(update);
            }
        }
    }

    pub fn offset(&self) -> Option<i64> {
        self.offset
    }
}
