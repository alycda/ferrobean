//! Types, functions and wrappers for Beancount

pub(crate) mod abc;
pub(crate) mod flags;

#[derive(Default)]
pub(crate) struct BeancountOptions {
    title: String,
    filename: String,
    name_assets: String,
    name_liabilities: String,
    name_equity: String,
    name_income: String,
    name_expenses: String,
}