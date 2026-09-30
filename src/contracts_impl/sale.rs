use async_sqlite::Pool;

pub struct ContractsSqliteStorage {
    connection: Pool,
}

impl SalesSqliteStorage {
    pub fn new(database_url: &str) -> Self {
        let path_buf: PathBuf = PathBuf::from(database_url);
        let builder: PoolBuilder = PoolBuilder::new()
            .path(path_buf)
            .journal_mode(async_sqlite::JournalMode::Wal)
            .num_conns(10);
        let connection: Pool = builder.open().await.expect("Database creation error");
        Self { connection }
    }
}

impl SalesStorage for SalesSqliteStorage {
    
    async fn add(
    &self,
    sale: crate::entities::Sale,
) -> Result<(), Box<dyn std::error::Error>> {
    const QUERY: &str = "
        INSERT INTO sales (id, route_id, travel, price, quantity, sale_date, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
    ";
    // id              
    // route_id        
    // travel          
    // price           
    // quantity        
    // sale_date       
    // created_at      
    // finished_at     



    let mut parameters: Vec<Value> = Vec::with_capacity(8);
        parameters.push(sale.id.into());
        parameters.push(sale.route_id.into());
        parameters.push(sale.travel.into());
        parameters.push(sale.price.into());
        parameters.push(sale.quantity.into());
        parameters.push(sale.sale_date.to_string().into());
        parameters.push(sale.created_at.to_string().into());
        parameters.push(sale.finished_at.map(|dt| dt.to_string()).into());

    self.connection
        .conn(move |connection| {
            let affected = connection.execute(QUERY, params_from_iter(parameters))?;
            Ok(affected)
        })
        .await?;

    Ok(())
}

    async fn get(&self, uuid: Uuid) -> Result<Sale, Box<dyn std::error::Error>> {
        const QUERY: &str = "
            SELECT id, route_id, travel, price, quantity, sale_date, created_at, finished_at
            FROM sales
            WHERE id = ?1
        ";
        let parameters: Vec<Value> = vec![uuid.into()];

        let result = self.connection
            .conn(move |connection| {
                connection.query_row(QUERY, params_from_iter(parameters), |row| {
                    let id: Uuid = row.get(0)?;
                    let route_id: Uuid = row.get(1)?;
                    let travel: String = row.get(2)?;
                    let price: f32 = row.get(3)?;
                    let quantity: i32 = row.get(4)?;
                    let sale_date: String = row.get(5)?;
                    let created_at: String = row.get(6)?;
                    let finished_at: Option<String> = row.get(7)?;
                    Ok((id, route_id, travel, price, quantity, sale_date, created_at, finished_at))
                })
            })
            .await?;

        Ok(())
    }
    async fn remote (
        &self,
        sale: crate::entities:: Sale,
    ) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM sales WHERE id = ?1";
        let parameters: Vec<Value> = vec![contract.id.into()];

    self.connection
        .conn(move |connection| {
            let affected = connection.execute(QUERY, params_from_iter(parameters))?;
            Ok(affected)
        })
        .await?;

    Ok(())
    }



    async fn update(&self, sale: &Sale) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            UPDATE sales
            SET route_id = ?1, travel = ?2, price = ?3, quantity = ?4, 
                sale_date = ?5, finished_at = ?6
            WHERE id = ?7
        ";
        let mut parameters: Vec<Value> = Vec::with_capacity(7);
        parameters.push(sale.route_id.into());
        parameters.push(sale.travel.clone().into());
        parameters.push(sale.price.into());
        parameters.push(sale.quantity.into());
        parameters.push(sale.sale_date.to_string().into());
        parameters.push(sale.finished_at.map(|dt| dt.to_string()).into());
        parameters.push(sale.id.into());

        self.connection
            .conn(move |connection| {
                let affected = connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(affected)
            })
            .await?;
        Ok(())
    }
}


