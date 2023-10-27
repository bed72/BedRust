use async_trait::async_trait;

#[async_trait(?Send)]
pub trait DatabaseClient<T> {
    async fn connect(&self) -> T;
}
