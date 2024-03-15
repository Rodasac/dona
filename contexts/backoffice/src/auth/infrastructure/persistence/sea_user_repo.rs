use sea_orm::{entity::prelude::*, sea_query::OnConflict};
use sea_orm::{DatabaseConnection, Set};
use shared::common::domain::base_errors::BaseRepositoryError;
use shared::common::domain::criteria::Criteria;
use shared::common::infrastructure::criteria::sea_criteria_converter::sea_convert_criteria;

use crate::auth::domain::{
    user::{User, UserCreatedAt, UserEmail, UserFullName, UserId, UserPassword, UserUpdatedAt},
    user_repository::UserRepository,
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub created_at: TimeDateTimeWithTimeZone,
    pub updated_at: TimeDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

fn from_model(model: Model) -> User {
    User::new(
        UserId::new(model.id.to_string()).unwrap(),
        UserEmail::new(model.email).unwrap(),
        UserPassword::new(model.password).unwrap(),
        UserFullName::new(model.full_name).unwrap(),
        UserCreatedAt::from_offset(model.created_at),
        UserUpdatedAt::from_offset(model.updated_at),
    )
}

pub struct SeaUserRepository {
    db: DatabaseConnection,
}

impl SeaUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl UserRepository for SeaUserRepository {
    async fn find_by_id(&self, id: UserId) -> Result<User, BaseRepositoryError> {
        let user = Entity::find_by_id(Uuid::parse_str(&id.to_string()).unwrap())
            .one(&self.db)
            .await
            .map(|u| u.map(from_model))
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?
            .ok_or(BaseRepositoryError::NotFound)?;

        Ok(user)
    }

    async fn find_by_criteria(&self, criteria: Criteria) -> Result<Vec<User>, BaseRepositoryError> {
        let mut user_query = Entity::find();
        let user_query = sea_convert_criteria("users", &mut user_query, criteria);

        let users = user_query
            .all(&self.db)
            .await
            .map(|u| u.into_iter().map(from_model).collect::<Vec<User>>())
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        Ok(users)
    }

    async fn find_all(&self) -> Result<Vec<User>, BaseRepositoryError> {
        let users = Entity::find()
            .all(&self.db)
            .await
            .map(|users| users.into_iter().map(from_model).collect())
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        Ok(users)
    }

    async fn save(&self, user: &User) -> Result<(), BaseRepositoryError> {
        let on_conflict = OnConflict::column(Column::Id)
            .update_columns(vec![Column::Password, Column::FullName, Column::UpdatedAt])
            .to_owned();
        let user = ActiveModel {
            id: Set(Uuid::parse_str(&user.id().to_string()).unwrap()),
            email: Set(user.email().to_string()),
            password: Set(user.password().to_string()),
            full_name: Set(user.full_name().to_string()),
            created_at: Set(user.created_at().value().to_owned()),
            updated_at: Set(user.updated_at().value().to_owned()),
        };

        Entity::insert(user)
            .on_conflict(on_conflict)
            .exec(&self.db)
            .await
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: UserId) -> Result<(), BaseRepositoryError> {
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
    use shared::common::domain::criteria::{
        filter::{Filter, FilterField, FilterOperator, FilterValue},
        order::{Order, OrderField, OrderType},
    };

    use crate::{auth::domain::user::tests::UserMother, test_utils::get_db_image};

    use super::*;

    #[tokio::test]
    async fn test_find_by_id() {
        let user = UserMother::random();

        let docker = testcontainers::clients::Cli::default();
        let dbimage = docker.run(get_db_image());
        let port = dbimage.get_host_port_ipv4(5432);

        let db = Database::connect(format!("postgres://dona:dona@localhost:{}/dona_test", port))
            .await
            .unwrap();
        Migrator::up(&db, None).await.unwrap();

        db.execute_unprepared(
            format!(
                "INSERT INTO users (id, email, password, full_name, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')",
                 user.id().to_string(), user.email().to_string(), user.password().to_string(), user.full_name().to_string(), user.created_at().to_string(), user.updated_at().to_string()
                ).as_str()
        ).await.unwrap();

        let repo = SeaUserRepository::new(db);

        let user_result = repo.find_by_id(user.id().to_owned()).await;

        assert!(user_result.is_ok());
        assert_eq!(user_result.unwrap(), user);
    }

    #[tokio::test]
    async fn test_find_by_criteria() {
        let user = UserMother::random();

        let docker = testcontainers::clients::Cli::default();
        let dbimage = docker.run(get_db_image());
        let port = dbimage.get_host_port_ipv4(5432);

        let db = Database::connect(format!("postgres://dona:dona@localhost:{}/dona_test", port))
            .await
            .unwrap();
        Migrator::up(&db, None).await.unwrap();

        db.execute_unprepared(
            format!(
                "INSERT INTO users (id, email, password, full_name, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')",
                 user.id().to_string(), user.email().to_string(), user.password().to_string(), user.full_name().to_string(), user.created_at().to_string(), user.updated_at().to_string()
                ).as_str()
        ).await.unwrap();

        let repo = SeaUserRepository::new(db);

        let criteria = Criteria::new(
            vec![Filter::new(
                FilterField::try_from("email".to_string()).unwrap(),
                FilterOperator::Equal,
                FilterValue::try_from(user.email().to_string()).unwrap(),
            )],
            Some(Order::new(
                OrderField::try_from("email".to_string()).unwrap(),
                OrderType::Asc,
            )),
            None,
        );
        let user_result = repo.find_by_criteria(criteria).await;

        assert!(user_result.is_ok());
        assert_eq!(user_result.unwrap(), vec![user]);
    }

    #[tokio::test]
    async fn should_save_user() {
        let user = UserMother::random();

        let docker = testcontainers::clients::Cli::default();
        let dbimage = docker.run(get_db_image());
        let port = dbimage.get_host_port_ipv4(5432);

        let db = Database::connect(format!("postgres://dona:dona@localhost:{}/dona_test", port))
            .await
            .unwrap();
        Migrator::up(&db, None).await.unwrap();

        let repo = SeaUserRepository::new(db);

        let save_result = repo.save(&user).await;

        assert!(save_result.is_ok());
    }

    #[tokio::test]
    async fn should_delete_user() {
        let user = UserMother::random();

        let docker = testcontainers::clients::Cli::default();
        let dbimage = docker.run(get_db_image());
        let port = dbimage.get_host_port_ipv4(5432);

        let db = Database::connect(format!("postgres://dona:dona@localhost:{}/dona_test", port))
            .await
            .unwrap();
        Migrator::up(&db, None).await.unwrap();

        db.execute_unprepared(
            format!(
                "INSERT INTO users (id, email, password, full_name, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')",
                 user.id().to_string(), user.email().to_string(), user.password().to_string(), user.full_name().to_string(), user.created_at().to_string(), user.updated_at().to_string()
                ).as_str()
        ).await.unwrap();

        let repo = SeaUserRepository::new(db);

        let delete_result = repo.delete(user.id().to_owned()).await;

        assert!(delete_result.is_ok());
    }

    #[tokio::test]
    async fn should_update_user() {
        let mut user = UserMother::random();

        let docker = testcontainers::clients::Cli::default();
        let dbimage = docker.run(get_db_image());
        let port = dbimage.get_host_port_ipv4(5432);

        let db = Database::connect(format!("postgres://dona:dona@localhost:{}/dona_test", port))
            .await
            .unwrap();
        Migrator::up(&db, None).await.unwrap();

        db.execute_unprepared(
            format!(
                "INSERT INTO users (id, email, password, full_name, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')",
                 user.id().to_string(), user.email().to_string(), user.password().to_string(), user.full_name().to_string(), user.created_at().to_string(), user.updated_at().to_string()
                ).as_str()
        ).await.unwrap();

        let repo = SeaUserRepository::new(db);

        user.update(
            Some(UserPassword::new("new_password".to_string()).unwrap()),
            Some(UserFullName::new("new_name".to_string()).unwrap()),
            UserUpdatedAt::new("2021-01-01T00:00:00Z".to_string()).unwrap(),
        )
        .expect("Failed to update user aggregate");

        let save_result = repo.save(&user).await;

        assert!(save_result.is_ok());
    }
}
