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
        use time::Month;

        pub(crate) const END_OF_YEAR: FiscalYearEnd = FiscalYearEnd(Month::December, 31);

        /// Month and day that specify the end of the fiscal year
        #[derive(PartialEq)]
        pub(crate) struct FiscalYearEnd(pub Month, pub u8);

        impl FiscalYearEnd {
            /// Actual month of the year
            fn month_of_year(&self) -> u8 {
                self.0.previous() as u8 % 12 + 1
            }

            /// Number of years that this is offset into the future
            fn year_offset(&self) -> u8 {
                self.0.previous() as u8 // 12
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

trait LoadFile {
    fn load_file(&mut self);
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