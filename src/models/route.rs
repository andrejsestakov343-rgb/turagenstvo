use time::UtcDateTime;
use uuid::Uuid;
use crate::models::Country;

#[doc = "Маршрут
         Поля:
         - идентификатор
         - код страны
         - код маршрута
         - наименование маршрута
         - дата создания
         - дата удаления
         - дата обновления"]
pub struct Route {
    pub(crate) id: Uuid,
    pub(crate) country_id: Uuid,
    pub(crate) name: String,
    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}

impl Route {
    pub fn create_new(country: &Country, name: String) -> Self {
        let id: Uuid = Uuid::new_v4();
        let created_at: UtcDateTime = UtcDateTime::now();
        Self {
            id,
            country_id: country.id,
            name,
            created_at,
            updated_at: None,
            deleted_at: None,
        }
    }

    pub fn create(
        id: Uuid,
        country: &Country,
        name: String,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Self {
        Self {
            id,
            country_id: country.id,
            name,
            created_at,
            updated_at,
            deleted_at,
        }
    }
}
