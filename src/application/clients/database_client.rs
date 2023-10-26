pub trait DatabaseClient<T>
where
    T: Sync + Send,
{
    fn connect(&self) -> T;
}
