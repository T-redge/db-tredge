#![feature(macro_metavar_expr_concat)]
macro_rules! create_table {
    ($struct_name:ident,$($field_name:ident: $tuple_field:ty),*) => {
        #[derive(Debug,Default)]
        pub struct ${concat(Row,$struct_name)} {
            $($field_name: $tuple_field),*
        }
        #[derive(Debug)]
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
            fn select<F>(&self, f: F) -> Option<Vec<${concat(Row,$struct_name)}>>
            where
                F: Fn(${concat(Row,$struct_name)}) -> ($($tuple_field),*)
            {
                let _x = f(${concat(Row,$struct_name)}::default());
                for row in &self.rows {
                    println!("{row:?}")

                }
                None
            }
            fn _print_table(&self) {
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
        create_table!(Person,person_id: u128,first_name: String,last_name: String);
        let mut p = TablePerson::new();
        p.insert_row(RowPerson {
            person_id: 1,
            first_name: "Alex".to_string(),
            last_name: "Tredgett".to_string(),
        });
        p.insert_row(RowPerson {
            person_id: 2,
            first_name: "Brix".to_string(),
            last_name: "Tredgett".to_string(),
        });

        p.select(|row| (row.person_id, row.first_name, _));
    }
}
