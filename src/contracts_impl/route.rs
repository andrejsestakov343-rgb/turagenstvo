use async_sqlite::Pool;

pub struct RoutesSqliteStorage {
    connection: Pool,
}

impl RoutesSqliteStorage {
    pub async fn new(database_url: &str) -> Self {
    let path_buf: PathBuf = PathBuf::from(database_url);
    let builder: PoolBuilder = PoolBuilder::new()
        .path(path_buf)
        .journal_mode(async_sqlite::JournalMode::Wal)
        .num_conns(10);
    let connection: Pool = builder.open().await.expect("Database creation error");
    Self { connection }
    }
}

impl RoutesStorage for RoutesSqliteStorage {

   
    async fn add(&self, route: Route) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            INSERT INTO routes (id, country_id, name, created_at, updated_at, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        ";
         
     //id               
     //country_id       
     //name             
     // created_at      
     // updated_at      
     // deleted_at      
       
        let mut parameters: Vec<Value> = Vec::with_capacity(6);
        parameters.push(route.id.into());
        parameters.push(route.country_id.into());
        parameters.push(route.name.into());
        parameters.push(route.created_at.to_string().into());
        parameters.push(route.updated_at.map(|dt| dt.to_string()).into());
        parameters.push(route.deleted_at.map(|dt| dt.to_string()).into());

        self.connection
            .conn(move |connection| {
                let affected = connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(affected)
            })
            .await?;
        Ok(())
    }
    async fn get(&self, uuid: Uuid) -> Result<Route, Box<dyn std::error::Error>> {
        const QUERY: &str = "
            SELECT id, country_id, name, created_at, updated_at, deleted_at
            FROM routes
            WHERE id = ?1
        ";
        let parameters: Vec<Value> = vec![uuid.into()];

        let result = self.connection
            .conn(move |connection| {
                connection.query_row(QUERY, params_from_iter(parameters), |row| {
                    let id: Uuid = row.get(0)?;
                    let country_id: Uuid = row.get(1)?;
                    let name: String = row.get(2)?;
                    let created_at: String = row.get(3)?;
                    let updated_at: Option<String> = row.get(4)?;
                    let deleted_at: Option<String> = row.get(5)?;
                    Ok((id, country_id, name, created_at, updated_at, deleted_at))
                })
            })
            .await?;

        Ok(())
    }
    async fn remove(
    &self,
    route: crate::entities::Route,
) -> Result<(), Box<dyn std::error::Error>> {
    const QUERY: &str = "DELETE FROM contracts WHERE id = ?1";

    let parameters: Vec<Value> = vec![contract.id.into()];

    self.connection
        .conn(move |connection| {
            let affected = connection.execute(QUERY, params_from_iter(parameters))?;
            Ok(affected)
        })
        .await?;

    Ok(())
}
    async fn update(&self, route: &Route) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            UPDATE routes
            SET country_id = ?1, name = ?2, updated_at = ?3, deleted_at = ?4
            WHERE id = ?5
        ";
        let mut parameters: Vec<Value> = Vec::with_capacity(5);
        parameters.push(route.country_id.into());
        parameters.push(route.name.clone().into());
        parameters.push(route.updated_at.map(|dt| dt.to_string()).into());
        parameters.push(route.deleted_at.map(|dt| dt.to_string()).into());
        parameters.push(route.id.into());

        self.connection
            .conn(move |connection| {
                let affected = connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(affected)
            })
            .await?;
        Ok(())
    }
}





