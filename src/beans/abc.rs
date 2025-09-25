//! Abstract base classes for Beancount types

use crate::beans::*;

trait Amount {
    /// Number of units in the amount
    fn get_value(&self) -> f32;
    fn get_currency(&self) -> &'static str;
}

/// an amount with date and label
trait Cost: Amount {
    fn get_date(&self) -> time::Date;
    fn get_label(&self) -> Option<String>;
}

#[derive(Debug, PartialEq)]
pub(crate) struct AAmount(pub f32, pub &'static str);

impl Amount for AAmount {
    fn get_value(&self) -> f32 {
        self.0
    }

    fn get_currency(&self) -> &'static str {
        self.1
    }
}

type DiffAmount = Option<AAmount>;

/// an Entry, must have a Date
/// 
/// see https://beancount.github.io/docs/beancount_language_syntax.html#directives
#[derive(Debug, PartialEq)]
pub(crate) enum Directive {
    Balance(time::Date, String, DiffAmount),
    /// This is a Custom type
    Budget,
    // time::Date, Account
    Close,
    Commodity,
    // Custom(Box<dyn Entry>),
    Document,
    Event,
    // time::Date, Account, Meta(String)
    Note,
    // time::Date, Account
    Open,
    Pad,
    Price,
    Query,
    Transactions(Transaction), // , Posting
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
pub(crate) trait Entry: PartialEq + std::fmt::Debug {
    fn get_date(&self) -> time::Date;
    fn get_meta(&self) {
        todo!()
    }
}

type Payee = String;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Transaction(pub flags::Flags, pub Option<Payee>);

impl Transaction {
    pub fn is_unrealized(&self) -> bool {
        self.0 == flags::Flags::Unrealized
    }
}

pub(crate) struct Posting(Box<dyn Position>);

/// cost and units
trait Position {
    fn get_units(&self) -> Box<dyn Amount>;
    fn get_cost(&self) -> Option<Box<dyn Cost>>;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn custom_directive() {
//         #[derive(Debug, PartialEq)]
//         struct MyCustomDirective(time::Date);

//         impl Default for MyCustomDirective {
//             fn default() -> Self {
//                 Self(time::OffsetDateTime::now_utc().date())
//             }
//         }

//         impl Entry for MyCustomDirective {
//             fn get_date(&self) -> time::Date {
//                 self.0
//             }
//         }
    
//         let entry = Directive::Custom(Box::new(MyCustomDirective::default()));
//         entry.get_date();
//     }
// }