use thiserror::Error;

#[derive(Error,Debug)]
pub enum BrowserError {
    #[error("unable to find any running browser")]
    FailedFindBrowser,
    #[error("there's no url")]
    FailedExtractUrl,
    #[error("failed to find the url UI")]
    FailedFindUrlUI,
    #[error("failed to enum visible window")]
    FailedEnumWindow,
}