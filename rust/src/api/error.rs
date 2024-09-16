
use snafu::Snafu;

pub type Result<O, E = Error> = std::result::Result<O, E>;

#[derive(Debug, Snafu)]
#[snafu(module(builder), context(suffix(false)), visibility(pub(crate)))]
pub(crate) enum Error {

    #[snafu(display("Database error: {source}"))]
    DatabaseError {
        backtrace: snafu::Backtrace,
        source: sea_orm::DbErr,
    },


}




