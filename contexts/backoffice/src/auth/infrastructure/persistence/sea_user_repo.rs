use sea_orm::{entity::prelude::*, sea_query::OnConflict};
use sea_orm::{DatabaseConnection, Set};
use shared::domain::base_errors::BaseRepositoryError;
use shared::domain::criteria::Criteria;
use shared::infrastructure::criteria::sea_criteria_converter::{
    convert_criteria_cursor, sea_convert_criteria,
};

use crate::auth::domain::user::{UserIsAdmin, UserLastLogin, UserProfilePicture, UserUsername};
use crate::auth::domain::{
    user::{User, UserCreatedAt, UserEmail, UserFullName, UserId, UserPassword, UserUpdatedAt},
    user_repository::UserRepository,
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub last_login: Option<TimeDateTimeWithTimeZone>,
    pub profile_picture: Option<String>,
    pub is_admin: bool,
    pub created_at: TimeDateTimeWithTimeZone,
    pub updated_at: TimeDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

fn from_model(model: Model) -> User {
    User::new(
        UserId::new(model.id.to_string()).unwrap(),
        UserUsername::new(model.username).unwrap(),
        UserEmail::new(model.email).unwrap(),
        UserPassword::new(model.password).unwrap(),
        UserFullName::new(model.full_name).unwrap(),
        UserLastLogin::new(model.last_login),
        UserProfilePicture::new(model.profile_picture).unwrap(),
        UserIsAdmin::new(model.is_admin),
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
        let user_query = sea_convert_criteria::<Column, Entity>(&mut user_query, criteria.clone())
            .map_err(|e| BaseRepositoryError::CriteriaCoverterError(e.to_string()))?;
        let mut cursor_query = user_query.cursor_by(Column::CreatedAt);
        let user_query =
            convert_criteria_cursor::<Column, Model>(criteria.cursor(), &mut cursor_query);

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
            username: Set(user.username().to_string()),
            email: Set(user.email().to_string()),
            password: Set(user.password().to_string()),
            full_name: Set(user.full_name().to_string()),
            last_login: Set(user.last_login().map(|d| d.to_owned())),
            profile_picture: Set(user.profile_picture().map(|p| p.to_owned())),
            is_admin: Set(user.is_admin().to_owned()),
            created_at: Set(user.created_at().to_owned()),
            updated_at: Set(user.updated_at().to_owned()),
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
    use shared::domain::criteria::{
        cursor::{Cursor, FirstField},
        filter::{Filter, FilterField, FilterOperator, FilterValue},
        order::{Order, OrderField, OrderType},
    };

    use crate::{
        auth::domain::user::tests::{UserIsAdminMother, UserMother},
        test_utils::get_db_image,
    };

    use super::*;

    #[tokio::test]
    async fn should_find_by_id() {
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
                r#"INSERT INTO users (id, username, email, password, full_name, last_login, profile_picture, is_admin, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', NULL, '{}', {}, '{}', '{}')"#,
                 user.id(), user.username(), user.email(), user.password(), user.full_name(), user.profile_picture().unwrap_or("NULL"), user.is_admin(), user.created_at(), user.updated_at()
                ).as_str()
        ).await.unwrap();

        let repo = SeaUserRepository::new(db);

        let user_result = repo
            .find_by_id(UserId::new(user.id().to_string()).unwrap())
            .await;

        assert!(user_result.is_ok());
        assert_eq!(user_result.unwrap(), user);
    }

    #[tokio::test]
    async fn should_find_by_criteria() {
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
                r#"INSERT INTO users (id, username, email, password, full_name, last_login, profile_picture, is_admin, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', NULL, '{}', {}, '{}', '{}')"#,
                user.id(), user.username(), user.email(), user.password(), user.full_name(), user.profile_picture().unwrap_or("NULL"), user.is_admin(), user.created_at(), user.updated_at()
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
            Some(Cursor::new(
                None,
                None,
                Some(FirstField::new(1).unwrap()),
                None,
            )),
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
                r#"INSERT INTO users (id, username, email, password, full_name, last_login, profile_picture, is_admin, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', NULL, '{}', {}, '{}', '{}')"#,
                user.id(), user.username(), user.email(), user.password(), user.full_name(), user.profile_picture().unwrap_or("NULL"), user.is_admin(), user.created_at(), user.updated_at()
                ).as_str()
        ).await.unwrap();

        let repo = SeaUserRepository::new(db);

        let delete_result = repo
            .delete(UserId::new(user.id().to_string()).unwrap())
            .await;

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
                r#"INSERT INTO users (id, username, email, password, full_name, last_login, profile_picture, is_admin, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', NULL, '{}', {}, '{}', '{}')"#,
                user.id(),
                user.username(),
                user.email(),
                user.password(),
                user.full_name(),
                user.profile_picture().unwrap_or("NULL"),
                user.is_admin(),
                user.created_at(),
                user.updated_at(),
                ).as_str()
        ).await.unwrap();

        let repo = SeaUserRepository::new(db);

        user.update(
            Some(UserUsername::new("new_email".to_string()).unwrap()),
            Some(UserPassword::new("new_password".to_string()).unwrap()),
            Some(UserFullName::new("new_name".to_string()).unwrap()),
            Some(UserProfilePicture::new(Some("new_picture.jpg".to_string())).unwrap()),
            Some(UserIsAdminMother::inverted(user.is_admin())),
            UserUpdatedAt::new("2021-01-01T00:00:00Z".to_string()).unwrap(),
        )
        .expect("Failed to update user aggregate");

        let save_result = repo.save(&user).await;

        assert!(save_result.is_ok());
    }
}
