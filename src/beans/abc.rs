//! Abstract base classes for Beancount types

/// an Entry, must have a Date
/// 
/// see https://beancount.github.io/docs/beancount_language_syntax.html#directives
pub(crate) enum Directive {
    // time::Date, Account
    Open,
    // time::Date, Account
    Close,
    Commodity,
    Transactions,
    
    // time::Date, Account, Meta(String)
    Note,
    // time::Date, Account, Meta(Amount)
    Balance,

    Document,
    Custom(Box<dyn Entry>),
    Budget,
}

impl Entry for Directive {
    fn get_date(&self) -> time::Date {
        // match self {
        //     (date, _) => date
        // }

        time::OffsetDateTime::now_utc().date() // temporary
    }
}

/// required behavior for a Directive
pub(crate) trait Entry {
    fn get_date(&self) -> time::Date;
    fn get_meta(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_directive() {
        struct MyCustomDirective(time::Date);

        impl Default for MyCustomDirective {
            fn default() -> Self {
                Self(time::OffsetDateTime::now_utc().date())
            }
        }

        impl Entry for MyCustomDirective {
            fn get_date(&self) -> time::Date {
                self.0
            }
        }
    
        let entry = Directive::Custom(Box::new(MyCustomDirective::default()));
        entry.get_date();
    }
}