use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    TableNotReady(String),
    Unhandled(Box<dyn StdError + Send + Sync + 'static>),
}

impl Error {
    pub fn table_not_ready(table_name: impl Into<String>) -> Self {
        Self::TableNotReady(table_name.into())
    }

    pub fn unhandled(source: impl Into<Box<dyn StdError + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(source.into())
    }
}

impl From<aws_sdk_dynamodb::Error> for Error {
    fn from(source: aws_sdk_dynamodb::Error) -> Self {
        Error::unhandled(source)
    }
}

impl<T> From<aws_sdk_dynamodb::types::SdkError<T>> for Error
where
    T: StdError + Send + Sync + 'static,
{
    fn from(source: aws_sdk_dynamodb::types::SdkError<T>) -> Self {
        Error::unhandled(source)
    }
}
