use diesel::mysql::MysqlConnection;

pub trait GenericTableFunctions {
  fn get(&self,id: u64,con: &MysqlConnection);
  fn delete(&self,id: u64,con: &MysqlConnection );
}
pub trait CreateTableFunctions {
  fn create<T>(&self,con: &MysqlConnection);
}
pub trait UpdateTableFunctions {
  fn update<T>(&self,con: &MysqlConnection);
}
pub trait SearchTableFunctions {
  fn search<T>(&self,con: &MysqlConnection);
}
