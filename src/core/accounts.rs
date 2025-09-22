// struct Accounts;

use crate::beans::abc::Directive;

// impl Accounts {
pub(crate) fn get_last_entry(postings: &Vec<Directive>) -> Option<&Directive> {
    postings.into_iter()
        .rev()                    // Start from the end  
        .find(|entry| {          // Find first match going backwards
            match entry {
                Directive::Transactions(t) => !t.is_unrealized(),  // Keep non-unrealized
                _ => true        // Keep all non-transaction directives
            }
        })
}

#[derive(Debug, PartialEq)]
enum Status {
    /// 'green':  A balance check that passed
    Pass,
    /// 'red':    A balance check that failed.
    Fail,
    /// 'yellow': Not a balance check.
    NotApplicable,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Pass => "green",
            Self::Fail => "red",
            Self::NotApplicable => "yellow",
        }
    }
}

/// Status of the last balance or transaction
fn uptodate_status(postings: &Vec<Directive>) -> Option<Status> {
    for entry in postings.into_iter().rev() {
        match entry {
            Directive::Balance(_, _, balance) => {
                if balance.is_some() {
                    return Some(Status::Fail);
                }
                return Some(Status::Pass);
            }
            Directive::Transactions(transaction) => {
                if !transaction.is_unrealized() {
                    return Some(Status::NotApplicable);
                }
            }
            _ => {
                // Continue to next entry for other directive types
            }
        }
    }
    None
}
// }

#[cfg(test)]
mod tests {
    use super::*;

    use crate::beans::{abc::{AAmount, Directive, Transaction}, flags::Flags};

    #[test]
    fn empty_list() {
        assert!(get_last_entry(&Vec::<Directive>::new()).is_none());
        assert!(uptodate_status(&Vec::<Directive>::new()).is_none());
    }

    #[test]
    fn single_directive() {
        assert!(get_last_entry(&vec![Directive::Open]).is_some());
        assert!(uptodate_status(&vec![Directive::Open]).is_none());
    }

    #[test]
    fn with_unrealized() {
        let entries = vec![Directive::Open, Directive::Transactions(Transaction(Flags::Unrealized))];

        assert_eq!(get_last_entry(&entries), Some(&Directive::Open));
        assert_eq!(uptodate_status(&entries), None);
    }

    #[test]
    fn with_balance() {
        let entries = vec![Directive::Open, Directive::Balance(time::OffsetDateTime::now_utc().date(), "Checking".into(), None)];

        assert_eq!(uptodate_status(&entries), Some(Status::Pass));
    }

    #[test]
    fn with_diff_balance() {
        let entries = vec![Directive::Open, Directive::Balance(time::OffsetDateTime::now_utc().date(), "Checking".into(), Some(AAmount(100, "USD")))];
        
        assert_eq!(uptodate_status(&entries), Some(Status::Fail));
    }
    
    #[test]
    fn multiple_valid_entries() {
        let entries = vec![
            Directive::Open,                                           // First valid
            Directive::Transactions(Transaction(Flags::Unrealized)),   // Unrealized (filtered out)
            Directive::Close,                                          // Last valid
        ];

        // Your original code: filter() removes unrealized, leaving [Open, Close]
        // Then rev() gives [Close, Open], then last() gives Open
        // So it returns Some(Directive::Open) - WRONG
        
        // Correct code should return Some(Directive::Close) - the last valid entry
        assert_eq!(get_last_entry(&entries), Some(&Directive::Close));
        assert_eq!(uptodate_status(&entries), None);
    }


}