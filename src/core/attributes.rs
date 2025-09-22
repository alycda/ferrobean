//! Attributes for auto-completion

use crate::{beans::abc::{Directive, Entry, Transaction}, util::date::{FiscalYearEnd, END_OF_YEAR}};

/// Return active years, with support for fiscal years
fn get_active_years(entries: Vec<Directive>, fye: FiscalYearEnd) -> Vec<String> {
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
#[derive(Default)]
struct AttributesModule;

impl AttributesModule {
    fn load_file(&self) {

    }

    fn payee_accounts(&self) -> Vec<String> {
        todo!()
    }

    fn payee_transaction(&self) -> Option<Transaction> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}