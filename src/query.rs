use crate::Context;

pub trait ExtractQuery {
    fn query_params<T: serde::de::DeserializeOwned>(&self) -> Option<T>;
}

impl<AppData: Send + Sync + 'static> ExtractQuery for Context<AppData> {
    fn query_params<T: serde::de::DeserializeOwned>(&self) -> Option<T> {
        self.uri()
            .query()
            .and_then(|q| serde_qs::from_str(q).ok())
    }
}
