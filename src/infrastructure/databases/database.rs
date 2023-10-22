pub trait Database<T> {
    fn connect() -> T;
}
