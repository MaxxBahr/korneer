pub mod coffee{
    pub enum Taste{
        Chocolate,
        Fruity,
        Caramel,
        Default
    }

    pub enum FieldValue{
        name(String),
        rating(u8),
        url(String),
        grind_size(u16),
        grind_time(f32),
        extraction_time(f32),
        taste(Taste)
    }
    pub struct Coffee {
        name: String,
        rating: u8,
        url: String,
        grind_size: u16,
        grind_time: f32,
        extraction_time: f32,
        taste: Taste
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