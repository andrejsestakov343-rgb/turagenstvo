pub trait SalesStorage {
    fn add(&self, sale: Sale) -> 
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn remove(&self, sale: Sale) -> 
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn get(&self, uuid: Uuid) -> 
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn update(&self, sale: &Sale) -> 
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
}
