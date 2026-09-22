use time::UtcDateTime;
use uuid::Uuid;
use crate::models::Route;

#[doc = "Продажа
         Поля:
         - идентификатор
         - код маршрута
         - цель путешествия
         - цена путевки (руб.)
         - количество проданных путевок по маршруту
         - дата продажи
         - дата создания
         - дата окончания"]
pub struct Sale {
    pub(crate) id: Uuid,
    pub(crate) route_id: Uuid,
    pub(crate) travel: String,
    pub(crate) price: f32,
    pub(crate) quantity: i32,
    pub(crate) sale_date: UtcDateTime,
    pub(crate) created_at: UtcDateTime,
    pub(crate) finished_at: Option<UtcDateTime>,
}

impl Sale {
    pub fn create_new(
        route: &Route,
        travel: String,
        price: f32,
        quantity: i32,
        sale_date: UtcDateTime,
    ) -> Self {
        let id: Uuid = Uuid::new_v4();
        let created_at: UtcDateTime = UtcDateTime::now();
        Self {
            id,
            route_id: route.id,
            travel,
            price,
            quantity,
            sale_date,
            created_at,
            finished_at: None,
        }
    }

    pub fn create(
        id: Uuid,
        route: &Route,
        travel: String,
        price: f32,
        quantity: i32,
        sale_date: UtcDateTime,
        created_at: UtcDateTime,
        finished_at: Option<UtcDateTime>,
    ) -> Self {
        Self {
            id,
            route_id: route.id,
            travel,
            price,
            quantity,
            sale_date,
            created_at,
            finished_at,
        }
    }
}


      
