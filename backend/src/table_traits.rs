
trait generic_table_functions {
  fn get(id: u64);
  fn delete(id: u64);
}

trait create_table_functions {
  fn create(&self);
}
trait update_table_functions {
  fn update(&self);
}

trait search_table_function {
  fn search(&self);
}
