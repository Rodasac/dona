use sea_orm::{entity::prelude::*, sea_query::OnConflict};
use sea_orm::{DatabaseConnection, Set};
use shared::common::domain::base_errors::BaseRepositoryError;

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
    async fn find_by_email(&self, email: UserEmail) -> Result<User, BaseRepositoryError> {
        let user = Entity::find()
            .filter(Column::Email.contains(email.to_string()))
            .one(&self.db)
            .await
            .map(|u| u.map(from_model))
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?
            .ok_or(BaseRepositoryError::NotFound)?;

        Ok(user)
    }

    async fn find_by_id(&self, id: UserId) -> Result<User, BaseRepositoryError> {
        let user = Entity::find_by_id(Uuid::parse_str(&id.to_string()).unwrap())
            .one(&self.db)
            .await
            .map(|u| u.map(from_model))
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?
            .ok_or(BaseRepositoryError::NotFound)?;

        Ok(user)
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
