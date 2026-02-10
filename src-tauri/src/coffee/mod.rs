// Entities of coffee
pub mod coffee{
    use rusqlite::{ToSql, types::FromSql};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub enum Taste{
        Chocolate,
        Fruity,
        Caramel,
        Default
    }

    impl ToSql for Taste{
        fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
            match self {
                Taste::Chocolate => Ok("Chocolate".to_string().into()),
                Taste::Fruity => Ok("Fruity".to_string().into()),
                Taste::Caramel => Ok("Caramel".to_string().into()),
                Taste::Default => Ok("Default".to_string().into()), 
                _ => Ok("".to_string().into())
            }
        }
    }

    impl FromSql for Taste{
        fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
            match value.as_str()? {
                "Chocolate" => Ok(Taste::Chocolate),
                "Fruity" => Ok(Taste::Fruity),
                "Caramel" => Ok(Taste::Caramel),
                "Default" => Ok(Taste::Default), 
                _ => Ok(Taste::Default)
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub enum FieldValue{
        name(String),
        rating(u8),
        url(String),
        grind_size(u16),
        grind_time(f32),
        extraction_time(f32),
        taste(Taste)
    }

    #[derive(Serialize, Deserialize)]
    pub struct Coffee {
        pub name: String,
        pub rating: u8,
        pub url: String,
        pub grind_size: u16,
        pub grind_time: f32,
        pub extraction_time: f32,
        pub taste: Taste
    }

    impl Coffee {
        pub fn new(name: String, url: String) -> Coffee{
            // call database with same values
            Coffee{
                name,
                rating: 0,
                url,
                grind_size: 0,
                grind_time: 0.0,
                extraction_time: 0.0,
                taste: Taste::Default,
            }
        }

        pub fn edit_value(&mut self, value: FieldValue)-> bool{
            match value{
                FieldValue::name(v) => self.name = v,
                FieldValue::rating(v) => self.rating = v,
                FieldValue::url(v) => self.url = v,
                FieldValue::grind_size(v) => self.grind_size = v,
                FieldValue::grind_time(v) => self.grind_time = v,
                FieldValue::extraction_time(v) => self.extraction_time = v,
                FieldValue::taste(v) => self.taste = v,
                _ => {}
            }
            // call change in database
            true
        }
    }

}