pub trait RoutesStorage {
    fn add(&self, route: Route) -> 
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn remove(&self, route: route) -> 
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn get(&self, uuid: Uuid) -> 
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn update(&self, route: &Route) -> 
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
}

