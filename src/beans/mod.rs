mod abc {
    /// required behavior for a Directive
    trait Entry {
        fn get_date(&self) -> time::Date;
        fn get_meta(&self) {
            todo!()
        }
    }
}

mod flags;