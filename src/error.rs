use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T, E = Report> = color_eyre::Result<T, E>;

pub struct Report(color_eyre::Report);

impl std::fmt::Debug for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<E> From<E> for Report
where
    E: Into<color_eyre::Report>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for Report {
    fn into_response(self) -> Response {
        let err = self.0;
        let err_string = format!("{err:?}");

        tracing::error!("{err_string}");

        if let Some(err) = err.downcast_ref::<AppError>() {
            return err.response();
        }

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".to_string(),
        )
            .into_response()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("A spooky thing happened")]
    Spooky,
}

impl AppError {
    fn response(&self) -> Response {
        match self {
            Self::Spooky => (
                StatusCode::IM_A_TEAPOT,
                "A user-facing message about a Spooky".to_string(),
            )
                .into_response(),
        }
    }
}
