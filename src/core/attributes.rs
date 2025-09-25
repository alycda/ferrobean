//! Attributes for auto-completion

use crate::{beans::abc::{Directive, Entry, Transaction}, core::ledger::FavaLedger, util::date::{FiscalYearEnd, END_OF_YEAR}, util::ranking::ExponentialDecayRanker, LoadFile};

/// Return active years, with support for fiscal years
fn get_active_years(entries: &Vec<Directive>, fye: FiscalYearEnd) -> Vec<String> {
    let mut years = Vec::new();
    
    if fye == END_OF_YEAR {
        let mut prev_year = None;
        for entry in entries {
            let year = entry.get_date().year();
            if Some(year) != prev_year {
                prev_year = Some(year);
                years.push(year);
            }
        }
        return years.into_iter()
            .rev()
            .map(|year| year.to_string())
            .collect();
    }
    
    let month = fye.0;
    let day = fye.1;
    let mut prev_year = None;
    
    for entry in entries {
        let date = entry.get_date();
        let year = if date.month() > month || (date.month() == month && date.day() > day) {
            date.year() + 1
        } else {
            date.year()
        };
        
        if Some(year) != prev_year {
            prev_year = Some(year);
            years.push(year);
        }
    }
    
    years.into_iter()
        .rev()
        .map(|year| format!("FY{}", year))
        .collect()
}

/// Some attributes of the ledger (mostly for auto-completion)
struct AttributesModule {
    ledger: FavaLedger,
    years: Vec<String>,
    payee_ranker: ExponentialDecayRanker,
}

impl Default for AttributesModule {
    fn default() -> Self {
        Self {
            ledger: FavaLedger::default(),
            years: Vec::new(),
            payee_ranker: ExponentialDecayRanker::new(),
        }
    }
}

impl LoadFile for AttributesModule {
    fn load_file(&mut self) {
        let all_entries = &self.ledger.all_entries;
        self.years = get_active_years(all_entries, END_OF_YEAR);

        // Build payee ranking
        let mut ranker = ExponentialDecayRanker::new();
        for entry in all_entries {
            if let Directive::Transactions(txn) = entry {
                if let Some(payee) = &txn.1 {
                    ranker.update(payee);
                }
            }
        }
        self.payee_ranker = ranker;
    }
}

impl AttributesModule {
    fn new(ledger: FavaLedger) -> Self {
        let mut s = Self {
            ledger,
            years: Vec::new(),
            payee_ranker: ExponentialDecayRanker::new(),
        };
        s.load_file();
        s
    }

    /// Return a list of all payee accounts (accounts that appear as payees in transactions), sorted by rank
    fn payee_accounts(&self, payee: &str) -> Vec<String> {
        self.payee_ranker
            .sort()
            .into_iter()
            .map(|(payee, _)| payee)
            .collect()
    }

    /// Get the last transaction for a payee
    fn payee_transaction(&self, payee: &str) -> Option<&Transaction> {
        let mut result: Option<&Transaction> = None;
        for entry in self.ledger.all_entries.iter().rev() {
            if let Directive::Transactions(txn) = entry {
                if let Some(txn_payee) = &txn.1 {
                    if txn_payee == payee {
                        result = Some(txn);
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}