//! a Beancount ledger

use std::path::PathBuf;

use crate::{beans::{abc::Directive, BeancountOptions}, LoadFile};

struct FilteredLedger(PathBuf);

impl FilteredLedger {
    fn new(file_path: PathBuf) -> Self {
        Self(file_path)
    }

    /// The date to use for prices
    fn end_date(&self) {

    }

    fn root_tree(&self) {

    }

    /// A root tree for the balance sheet
    fn root_tree_closed(&self) {

    }

    /// Yield date ranges corresponding to interval boundaries
    fn interval_ranges(&self) {

    }

    /// List all prices
    fn prices(&self) {

    }

    fn account_is_closed(&self) -> bool {
        todo!()
    }
}

/// an interface for a Beancount ledger
#[derive(Default)]
pub(crate) struct FavaLedger {
    beancount_file_path: PathBuf,
    pub all_entries: Vec<Directive>,
    /// should limit to Helpers::BeancountError
    errors: Vec<String>,
    option: BeancountOptions,
}

impl LoadFile for FavaLedger {
    fn load_file(&mut self) {
        todo!()
    }
}

impl FavaLedger {
    fn new(beancount_file_path: PathBuf) -> Self {
        let mut default = Self::default();

        default.beancount_file_path = beancount_file_path;

        default
    }

    fn get_filtered(&self) -> FilteredLedger {
        todo!()
    }

    /// The timestamp to the latest change of the underlying files.
    fn mtime(&self) {
        todo!()
    }

    /// The five root accounts
    fn root_accounts(&self) -> [String; 5] {
        todo!()
    }

    /// Path relative to the directory of the ledger
    fn join_path(&self) {

    }

    /// Get paths to included files and document directories
    fn paths_to_watch(&self) {

    }

    /// Check if the file needs to be reloaded
    fn changed(&self) -> bool {
        todo!()
    }

    /// Balances by interval
    fn interval_balances(&self) {

    }

    /// Journal for an account
    fn account_journal(&self) {

    }

    /// Find an entry
    fn get_entry(&self) -> Option<Directive> {
        todo!()
    }

    /// Context for an entry
    fn context(&self) {

    }

    /// List pairs of commodities
    fn commodity_pairs(&self) {

    }

    /// Get the path for a statement found in the specified entry
    fn statement_path(&self) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;
}