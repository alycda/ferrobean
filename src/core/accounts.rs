// struct Accounts;

use crate::beans::abc::Directive;

// impl Accounts {
fn get_last_entry(postings: &Vec<Directive>) -> Option<&Directive> {
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

/// Balance directive for the given account for today
fn balance_string(tree_node: &super::tree::TreeNode) -> String {
    let today = time::OffsetDateTime::now_utc().date();
    let account = &tree_node.get_name();
    let mut res = String::new();
    
    for (currency, number) in &tree_node.get_balance() {
        res.push_str(&format!(
            "{} balance {:<28} {:>15} {}\n",
            today, account, number, currency
        ));
    }
    
    res
}
// }

/// Date and hash of the last entry for an account
struct LastEntry(time::Date, String);

/// Holds information about an account
struct AccountData {
    /// The date on which this account is closed 
    close_date: Option<time::Date>,
    // The metadata of the Open entry of this account
    // meta: 
    /// Uptodate status. Is only computed if the account has a "fava-uptodate-indication" meta attribute.
    uptodate_status: Option<Status>,
    /// Balance directive if this account has an uptodate status
    balance_string: String,
    /// The last entry of the account (unless it is a close Entry)
    last_entry: Option<LastEntry>,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    use crate::{beans::{abc::{AAmount, Directive, Transaction}, flags::Flags}, core::tree::TreeNode};

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
        let entries = vec![Directive::Open, Directive::Balance(time::OffsetDateTime::now_utc().date(), "Checking".into(), Some(AAmount(100., "USD")))];
        
        assert_eq!(uptodate_status(&entries), Some(Status::Fail));
    }
    
    #[test]
    fn multiple_valid_entries() {
        let entries = vec![
            Directive::Open,                                           // First valid
            Directive::Transactions(Transaction(Flags::Unrealized)),   // Unrealized (filtered out)
            Directive::Close,                                          // Last valid
        ];

        assert_eq!(get_last_entry(&entries), Some(&Directive::Close));
        assert_eq!(uptodate_status(&entries), None);
    }

    #[test]
    fn test_balance_string_single_currency() {
        let mut balance = HashMap::new();
        balance.insert("USD".to_string(), 1234.56);
        
        let tree_node = TreeNode("Assets:Cash".to_string(),
            balance);
        
        let result = balance_string(&tree_node);
        let today = time::OffsetDateTime::now_utc().date();
        let expected = format!("{} balance Assets:Cash                          1234.56 USD\n", today);
        
        assert_eq!(result, expected);
    }
}