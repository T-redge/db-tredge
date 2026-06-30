#![feature(macro_metavar_expr_concat)]
macro_rules! create_table {
    ($struct_name:ident,$($field_name:ident: $tuple_field:ty),*) => {

        #[derive(Debug,Default)]
        pub struct ${concat(Row,$struct_name)} {
            $($field_name: $tuple_field),*
        }
        pub struct ${concat(Table,$struct_name)} {
            rows: Vec<${concat(Row,$struct_name)}>
        }
        impl ${concat(Table,$struct_name)} {
            fn new() -> Self {
                Self {
                    rows: Vec::new(),
                }
            }
            fn insert_row(&mut self, row: ${concat(Row,$struct_name)}) {
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
        create_table!(Person,first_name: String,last_name: String);
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
