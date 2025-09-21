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

mod core {
    mod accounts {
        // struct Accounts;

        // impl Accounts {
        // fn get_last_entry(postings: Vec<beans::abc::Directive>) {
        pub(crate) fn get_last_entry(postings: Vec<impl crate::beans::abc::Entry>) -> Option<impl crate::beans::abc::Entry> {
            postings.into_iter()//.filter(|t| !t.is_unrealized())
                .rev().last()
        }
        // }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn empty_list() {
            assert!(accounts::get_last_entry(Vec::<crate::beans::abc::Directive>::new()).is_none())
        }

        #[test]
        fn single_directive() {
            assert!(accounts::get_last_entry(vec![crate::beans::abc::Directive::Open]).is_some())
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