pub trait Io: parity_scale_codec::Encode + parity_scale_codec::Decode {}
pub trait IntoContract {}
pub trait IntoQuery {}

#[macro_export]
macro_rules! iter_log_error(($results: expr, $msgf: expr) => ({
    $results.iter()
            .filter(|result| result.is_err())
            .for_each(|error_result| {
                log::error!($msgf, error_result)
            })
}));

