use sea_orm::{entity::prelude::*, sea_query::OnConflict};
use sea_orm::{DatabaseConnection, Set};
use shared::domain::base_errors::BaseRepositoryError;
use shared::domain::criteria::Criteria;
use shared::infrastructure::criteria::sea_criteria_converter::{
    convert_criteria_cursor, sea_convert_criteria,
};

use crate::dona::domain::dona::{Dona, DonaId};
use crate::dona::domain::dona_repository::DonaRepository;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "donas")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub msg: String,
    pub amount: Decimal,
    pub status: String,
    pub option_method: String,
    pub user_id: String,
    pub sender_id: String,
    pub created_at: TimeDateTimeWithTimeZone,
    pub updated_at: TimeDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

fn from_model(model: Model) -> Dona {
    Dona::new(
        model.id.to_string(),
        model.msg,
        model.amount,
        model.status,
        model.option_method,
        model.user_id,
        model.sender_id,
        model.created_at,
        model.updated_at,
    )
    .unwrap()
}

pub struct SeaDonaRepo {
    db: DatabaseConnection,
}

impl SeaDonaRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl DonaRepository for SeaDonaRepo {
    async fn find_by_id(&self, id: DonaId) -> Result<Dona, BaseRepositoryError> {
        let dona = Entity::find_by_id(Uuid::parse_str(&id.to_string()).unwrap())
            .one(&self.db)
            .await
            .map(|u| u.map(from_model))
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?
            .ok_or(BaseRepositoryError::NotFound)?;

        Ok(dona)
    }

    async fn find_by_criteria(&self, criteria: Criteria) -> Result<Vec<Dona>, BaseRepositoryError> {
        let mut dona_query = Entity::find();
        let dona_query = sea_convert_criteria::<Column, Entity>(&mut dona_query, criteria.clone())
            .map_err(|e| BaseRepositoryError::CriteriaCoverterError(e.to_string()))?;
        let mut cursor_query = dona_query.cursor_by(Column::CreatedAt);
        let dona_query =
            convert_criteria_cursor::<Column, Model>(criteria.cursor(), &mut cursor_query);

        let donas = dona_query
            .all(&self.db)
            .await
            .map(|u| u.into_iter().map(from_model).collect::<Vec<Dona>>())
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        Ok(donas)
    }

    async fn find_all(&self) -> Result<Vec<Dona>, BaseRepositoryError> {
        let donas = Entity::find()
            .all(&self.db)
            .await
            .map(|u| u.into_iter().map(from_model).collect::<Vec<Dona>>())
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;
        Ok(donas)
    }

    async fn save(&self, dona: &Dona) -> Result<(), BaseRepositoryError> {
        let on_conflict = OnConflict::column(Column::Id)
            .update_columns(vec![
                Column::Msg,
                Column::Amount,
                Column::Status,
                Column::OptionMethod,
                Column::UserId,
                Column::SenderId,
                Column::CreatedAt,
                Column::UpdatedAt,
            ])
            .to_owned();
        let dona = ActiveModel {
            id: Set(Uuid::parse_str(&dona.id()).unwrap()),
            msg: Set(dona.msg()),
            amount: Set(dona.amount()),
            status: Set(dona.status()),
            option_method: Set(dona.method()),
            user_id: Set(dona.user_id()),
            sender_id: Set(dona.sender_id()),
            created_at: Set(dona.created_at()),
            updated_at: Set(dona.updated_at()),
        };

        Entity::insert(dona)
            .on_conflict(on_conflict)
            .exec(&self.db)
            .await
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: DonaId) -> Result<(), BaseRepositoryError> {
        Entity::delete_by_id(Uuid::parse_str(&id.to_string()).unwrap())
            .exec(&self.db)
            .await
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use migration::{Migrator, MigratorTrait};
    use sea_orm::Database;
    use shared::domain::criteria::{
        cursor::{Cursor, FirstField},
        filter::{Filter, FilterField, FilterOperator, FilterValue},
    };

    use crate::{dona::domain::dona::tests::DonaMother, test_utils::get_db_image};

    use super::*;

    #[tokio::test]
    async fn it_should_save_find_and_delete_dona() {
        let docker = testcontainers::clients::Cli::default();
        let dbimage = docker.run(get_db_image());
        let port = dbimage.get_host_port_ipv4(5432);

        let db = Database::connect(format!("postgres://dona:dona@localhost:{}/dona_test", port))
            .await
            .unwrap();
        Migrator::up(&db, None).await.unwrap();

        let repo = SeaDonaRepo::new(db);

        let dona = DonaMother::random();
        repo.save(&dona).await.unwrap();

        let dona_id = DonaId::new(dona.id()).unwrap();
        let dona_found = repo.find_by_id(dona_id.clone()).await.unwrap();
        assert_eq!(dona, dona_found);

        let donas = repo.find_all().await.unwrap();
        assert_eq!(1, donas.len());

        let criteria = Criteria::new(
            vec![Filter::new(
                FilterField::try_from("msg".to_string()).unwrap(),
                FilterOperator::Equal,
                FilterValue::try_from(dona.msg().to_string()).unwrap(),
            )],
            None,
            Some(Cursor::new(
                None,
                None,
                Some(FirstField::new(1).unwrap()),
                None,
            )),
        );

        let donas = repo.find_by_criteria(criteria).await.unwrap();
        assert_eq!(1, donas.len());

        repo.delete(dona_id).await.unwrap();
        let donas = repo.find_all().await.unwrap();
        assert_eq!(0, donas.len());
    }
}
