use worker::*;

/// Exceptions
///
/// see: https://github.com/beancount/fava/blob/main/src/fava/helpers.py
#[derive(Debug, PartialEq)]
enum Helpers {
    /// TODO: Option<Directive>, Option<Source>
    BeancountError(String),
    FavaError(String)
}

mod beans;
mod core;

mod util {
    pub(crate) mod date {

        /// Month and day that specify the end of the fiscal year
        pub(crate) struct FiscalYearEnd(u8, u8);

        impl FiscalYearEnd {
            /// Actual month of the year
            fn month_of_year(&self) -> u8 {
                (self.0 - 1) % 12 + 1
            }

            /// Number of years that this is offset into the future
            fn year_offset(&self) -> u8 {
                self.0 - 1 // 12
            }

            /// Whether this fiscal year end supports fiscal quarters
            fn has_quarters(&self) -> u8 {
                todo!();
                // (
                //     datetime.date(2001, self.month_of_year, self.day) + ONE_DAY
                // ).day == 1
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;


        }
    }
}

#[event(fetch)]
async fn fetch(
    _req: Request,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    console_error_panic_hook::set_once();
    Response::ok("Hello World!")
}