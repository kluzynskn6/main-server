mod table_traits {    
    trait GenericTableFunctions {
      fn get(id: u64);
      fn delete(id: u64);
    }
    trait CreateTableFunctions {
      fn create(&self);
    }
    trait UpdateTableFunctions {
      fn update(&self);
    }
    trait SearchTableFunction {
      fn search(&self);
    }
}
