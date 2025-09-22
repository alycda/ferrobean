//! a Beancount ledger

use crate::beans::abc::Directive;

struct FilteredLedger;

struct FavaLedger {

}

impl FavaLedger {
    fn new() -> Self {
        todo!()
    }

    fn load_file() {

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