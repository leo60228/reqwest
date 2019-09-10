/// dox
#[derive(Debug)]
pub struct Error(());

pub(crate) fn from<T>(_: T) -> Error {
    Error(())
}
