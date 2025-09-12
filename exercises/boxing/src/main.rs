trait TTrait {
    fn get(&self) -> &str;
}

struct TraitImpl {
    value: String
}

impl TTrait for TraitImpl {
    fn get(&self) -> &str {
        &self.value
    }
}

fn main() {
    
    let value = 42;

    let boxed_i32 = Box::new(value);

    let boxed_tuple = Box::new((1,2,3,4,5));

    let boxed_trait: Box<dyn TTrait> = Box::new(TraitImpl { value: "test".into() });

    println!("{:?}", boxed_tuple);
}