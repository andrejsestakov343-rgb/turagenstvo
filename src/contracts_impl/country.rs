use async_sqlite::Pool;

pub struct CountriesSqliteStorage {
    connection: Pool,
}

impl CountriesSqliteStorage {
    pub fn new(database_url: &str) -> Self {
        let path_buf: PathBuf = PathBuf:: from (database_url);
        let builder: PoolBuilder = PoolBuilder:: new()
        .path(path_buf)
        .journal_mode(async_sqlite::JournalMode::Wal)
        .num_conns(10);
    let connection: Pool = builder.open().await.expect("Database creation error");
    Self {connection}
    }
}

impl CountriesStorage for CountriesSqliteStorage {

    async fn add(&self, country: Country) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            INSERT INTO countries (id, name, visa, created_at, updated_at, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        ";
         
    //id                  
    //name               
    //visa                
    //created_at          
    //updated_at          
    //deleted_at          
        
        let mut parameters: Vec<Value> = Vec::with_capacity(6);
        parameters.push(country.id.into());
        parameters.push(country.name.into());
        parameters.push(country.visa.into());
        parameters.push(country.created_at.to_string().into());
        parameters.push(country.updated_at.map(|dt| dt.to_string()).into());
        parameters.push(country.deleted_at.map(|dt| dt.to_string()).into());

        self.connection
            .conn(move |connection| {
                let affected = connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(affected)
            })
            .await?;
        Ok(())
    }
    async fn get(&self, uuid: Uuid) -> Result<Country, Box<dyn std::error::Error>> {
        const QUERY: &str = "
            SELECT id, name, visa, created_at, updated_at, deleted_at
            FROM countries
            WHERE id = ?1
        ";
        let parameters: Vec<Value> = vec![uuid.into()];

        let result = self.connection
            .conn(move |connection| {
                connection.query_row(QUERY, params_from_iter(parameters), |row| {
                    let id: Uuid = row.get(0)?;
                    let name: String = row.get(1)?;
                    let visa: f32 = row.get(2)?;
                    let created_at: String = row.get(3)?;
                    let updated_at: Option<String> = row.get(4)?;
                    let deleted_at: Option<String> = row.get(5)?;
                    Ok((id, name, visa, created_at, updated_at, deleted_at))
                })
            })
            .await?;

        Ok(())
    }
    async fn remove(&self, country: Country) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM countries WHERE id = ?1";
        let parameters: Vec<Value> = vec![country.id.into()];

        self.connection
            .conn(move |connection| {
                let affected = connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(affected)
            })
            .await?;
        Ok(())
    }

    async fn update(&self, country: &Country) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            UPDATE countries
            SET name = ?1, visa = ?2, updated_at = ?3, deleted_at = ?4
            WHERE id = ?5
        ";
        let mut parameters: Vec<Value> = Vec::with_capacity(5);
        parameters.push(country.name.clone().into());
        parameters.push(country.visa.into());
        parameters.push(country.updated_at.map(|dt| dt.to_string()).into());
        parameters.push(country.deleted_at.map(|dt| dt.to_string()).into());
        parameters.push(country.id.into());

        self.connection
            .conn(move |connection| {
                let affected = connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(affected)
            })
            .await?;
        Ok(())
    }
}



