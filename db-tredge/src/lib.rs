macro_rules! create_table {
    ($table_name:ident,$row_name: ident,$($field_name:ident: $tuple_field:ty),*) => {

        #[derive(Debug,Default)]
        pub struct $row_name {
                $($field_name: $tuple_field),*
            }
        pub struct $table_name {
            rows: Vec<$row_name>
        }
        impl $table_name {
            fn new() -> $table_name {
                $table_name {
                    rows: Vec::new(),
                }
            }
            fn insert_row(&mut self, row: $row_name) {
                self.rows.push(row);
            }
            fn print_table(&self) {
                for row in &self.rows {
                    println!("{:?}", row);
                }
            }
        }

    };
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        create_table!(TablePerson,RowPerson,first_name: String,last_name: String);
        let mut p = TablePerson::new();
        p.insert_row(RowPerson {
            first_name: "Alex".to_string(),
            last_name: "Tredgett".to_string(),
        });
        p.insert_row(RowPerson {
            first_name: "Brix".to_string(),
            last_name: "Tredgett".to_string(),
        });
        p.print_table();
    }
}
