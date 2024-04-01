use sea_orm::{entity::prelude::*, sea_query::OnConflict};
use sea_orm::{DatabaseConnection, Set};
use shared::domain::base_errors::BaseRepositoryError;
use shared::domain::criteria::Criteria;
use shared::infrastructure::criteria::sea_criteria_converter::{
    convert_criteria_cursor, sea_convert_criteria,
};

use crate::user_payment_method::domain::user_payment_method::{
    UserPaymentMethod, UserPaymentMethodId,
};
use crate::user_payment_method::domain::user_payment_method_repository::UserPaymentMethodRepository;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_payment_methods")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub payment_method: String,
    pub instructions: String,
    pub created_at: TimeDateTimeWithTimeZone,
    pub updated_at: TimeDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

fn from_model(model: Model) -> UserPaymentMethod {
    UserPaymentMethod::new(
        model.id.to_string(),
        model.user_id.to_string(),
        model.payment_method,
        model.instructions,
        model.created_at,
        model.updated_at,
    )
    .unwrap()
}

pub struct SeaUserPaymentMethodRepo {
    db: DatabaseConnection,
}

impl SeaUserPaymentMethodRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl UserPaymentMethodRepository for SeaUserPaymentMethodRepo {
    async fn find_by_id(
        &self,
        id: UserPaymentMethodId,
    ) -> Result<UserPaymentMethod, BaseRepositoryError> {
        Entity::find_by_id(Uuid::parse_str(&id.to_string()).unwrap())
            .one(&self.db)
            .await
            .map(|u| u.map(from_model))
            .map_err(|_| BaseRepositoryError::NotFound)?
            .ok_or(BaseRepositoryError::NotFound)
    }

    async fn find_by_criteria(
        &self,
        criteria: Criteria,
    ) -> Result<Vec<UserPaymentMethod>, BaseRepositoryError> {
        let mut query = Entity::find();
        let query = sea_convert_criteria::<Column, Entity>(&mut query, criteria.clone())
            .map_err(|e| BaseRepositoryError::CriteriaCoverterError(e.to_string()))?;
        let mut cursor_query = query.cursor_by(Column::CreatedAt);
        let query = convert_criteria_cursor::<Column, Model>(criteria.cursor(), &mut cursor_query);

        let user_payment_methods = query
            .all(&self.db)
            .await
            .map(|u| {
                u.into_iter()
                    .map(from_model)
                    .collect::<Vec<UserPaymentMethod>>()
            })
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        Ok(user_payment_methods)
    }

    async fn find_all(&self) -> Result<Vec<UserPaymentMethod>, BaseRepositoryError> {
        let user_payment_methods = Entity::find()
            .all(&self.db)
            .await
            .map(|u| {
                u.into_iter()
                    .map(from_model)
                    .collect::<Vec<UserPaymentMethod>>()
            })
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        Ok(user_payment_methods)
    }

    async fn save(
        &self,
        user_payment_method: &UserPaymentMethod,
    ) -> Result<(), BaseRepositoryError> {
        let on_conflict = OnConflict::column(Column::Id)
            .update_columns(vec![
                Column::UserId,
                Column::PaymentMethod,
                Column::Instructions,
                Column::CreatedAt,
                Column::UpdatedAt,
            ])
            .to_owned();

        let user_payment_method = ActiveModel {
            id: Set(Uuid::parse_str(&user_payment_method.id()).unwrap()),
            user_id: Set(Uuid::parse_str(&user_payment_method.user_id()).unwrap()),
            payment_method: Set(user_payment_method.payment_method().to_string()),
            instructions: Set(user_payment_method.instructions().to_string()),
            created_at: Set(user_payment_method.created_at()),
            updated_at: Set(user_payment_method.updated_at()),
        };

        Entity::insert(user_payment_method)
            .on_conflict(on_conflict)
            .exec(&self.db)
            .await
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: UserPaymentMethodId) -> Result<(), BaseRepositoryError> {
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

    use crate::{
        test_utils::get_db_image,
        user_payment_method::domain::user_payment_method::tests::UserPaymentMethodMother,
    };

    use super::*;

    #[tokio::test]
    async fn it_should_save_find_and_delete_user_payment_method() {
        let docker = testcontainers::clients::Cli::default();
        let dbimage = docker.run(get_db_image());
        let port = dbimage.get_host_port_ipv4(5432);

        let db = Database::connect(format!("postgres://dona:dona@localhost:{}/dona_test", port))
            .await
            .unwrap();
        Migrator::up(&db, None).await.unwrap();

        let repo = SeaUserPaymentMethodRepo::new(db);

        let method = UserPaymentMethodMother::random();

        // Save
        repo.save(&method)
            .await
            .expect("Error saving user payment method");

        let method_id = UserPaymentMethodId::new(method.id()).unwrap();

        // Find by id
        let found_method = repo
            .find_by_id(method_id.clone())
            .await
            .expect("Error finding user payment method");

        assert_eq!(method, found_method);

        // Find all
        let methods = repo
            .find_all()
            .await
            .expect("Error finding all user payment methods");
        assert_eq!(1, methods.len());
        assert_eq!(method, methods.first().unwrap().clone());

        // Find by criteria
        let criteria = Criteria::new(
            vec![Filter::new(
                FilterField::try_from("user_id".to_string()).unwrap(),
                FilterOperator::Equal,
                FilterValue::try_from(method.user_id()).unwrap(),
            )],
            None,
            Some(Cursor::new(
                None,
                None,
                Some(FirstField::new(1).unwrap()),
                None,
            )),
        );
        let methods = repo
            .find_by_criteria(criteria)
            .await
            .expect("Error finding user payment methods by criteria");
        assert_eq!(1, methods.len());
        assert_eq!(method, methods.first().unwrap().clone());

        // Delete
        repo.delete(method_id.clone())
            .await
            .expect("Error deleting user payment method");
        repo.find_by_id(method_id)
            .await
            .expect_err("Error deleting user payment method");
    }
}
