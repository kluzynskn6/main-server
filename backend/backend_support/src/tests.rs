#[cfg(test)]
mod tests {
    use crate::search::NullableSearch;
    use crate::search::Search;

    #[derive(GenericTableFunctions)]
    struct genstruct {
        id: u64,
        name: String,
        banner: u32,
    }
    #[derive(CreateTableFunctions)]
    struct createstruct {
        name: String,
        banner: u32,
    }
    #[derive(UpdateTableFunction)]
    struct upstruct {
        name: Option<String>,
        banner: Option<u32>,
    }
    #[derive(SearchTableFunction)]
    struct searchstruct {
        name: Search<String>,
        banner: Search<u32>,
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn function_test() {
        let gentest = genstruct {
            id: 1,
            name: "nick".to_string(),
            banner: 123456789,
        };
        gentest.get();
    }
}

