pub trait CountriesStorage {
    fn add(&self, country: Country) ->
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn remove(&self, country: Country) ->
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn get(&self, uuid: Uuid) ->
        impl Future<Output = Result<Country, Box<dyn std::error::Error>>>;

    fn update(&self, country: &Country) ->
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
}