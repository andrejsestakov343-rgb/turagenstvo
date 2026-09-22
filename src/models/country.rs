use time::UtcDateTime;
use uuid::Uuid;

#[doc = "Страна
         Поля:
         - идентификатор
         - код страны
         - название страны
         - стоимость визы (руб.)
         - дата создания
         - дата удаления
         - дата обновления"]
pub struct Country {
    // - Код страны (UUID)
    pub(crate) id: Uuid,
    // - Название страны (String)
    pub(crate) name: String,
    // - Стоимость визы (руб.) (float)
    pub(crate) visa: f32,


    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}

impl Country {
    pub fn create_new(name: String, visa: f32) -> Self {
        let id: Uuid = Uuid::new_v4();
        let created_at: UtcDateTime = UtcDateTime::now();
        Self {
            id,
            name,
            visa,
            created_at,
            updated_at: None,
            deleted_at: None,
        }
    }

    pub fn create(
        id: Uuid,
        name: String,
        visa: f32,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Self {
        let result: Country = Country { 
            id,
            name,
            visa,
            created_at,
            updated_at,
            deleted_at,
        };
        result
    }
}
