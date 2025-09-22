mod accounts;

mod inventory {
    use std::collections::HashMap;

    /// A lightweight inventory
    pub(crate) type CounterInventory = HashMap<String, f32>;

    #[cfg(test)]
    mod tests {
        use super::*;
    }
}

mod tree {
    use crate::core::inventory::CounterInventory;

    /// name, balance
    pub(crate) struct TreeNode(pub String, pub CounterInventory);

    impl TreeNode {
        fn new(account_name: String) -> Self {
            Self(account_name, CounterInventory::new())
        }

        pub fn get_name(&self) -> String {
            self.0.clone()
        }

        pub fn get_balance(&self) -> CounterInventory {
            self.1.clone()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    }
}