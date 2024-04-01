use sea_orm::{entity::prelude::*, sea_query::OnConflict};
use sea_orm::{DatabaseConnection, Set};
use shared::domain::base_errors::BaseRepositoryError;
use shared::domain::criteria::Criteria;
use shared::infrastructure::criteria::sea_criteria_converter::{
    convert_criteria_cursor, sea_convert_criteria,
};

use crate::posts::domain::post::{Post, PostId};
use crate::posts::domain::post_repository::PostRepository;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub post_picture: Option<String>,
    pub is_nsfw: bool,
    pub created_at: TimeDateTimeWithTimeZone,
    pub updated_at: TimeDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

fn from_model(model: Model) -> Post {
    Post::new(
        model.id.to_string(),
        model.user_id.to_string(),
        model.content,
        model.post_picture,
        model.is_nsfw,
        model.created_at,
        model.updated_at,
    )
    .unwrap()
}

pub struct SeaPostRepo {
    db: DatabaseConnection,
}

impl SeaPostRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl PostRepository for SeaPostRepo {
    async fn find_by_id(&self, id: PostId) -> Result<Post, BaseRepositoryError> {
        Entity::find_by_id(Uuid::parse_str(&id.to_string()).unwrap())
            .one(&self.db)
            .await
            .map(|u| u.map(from_model))
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?
            .ok_or(BaseRepositoryError::NotFound)
    }

    async fn find_by_criteria(&self, criteria: Criteria) -> Result<Vec<Post>, BaseRepositoryError> {
        let mut query = Entity::find();
        let query = sea_convert_criteria::<Column, Entity>(&mut query, criteria.clone())
            .map_err(|e| BaseRepositoryError::CriteriaCoverterError(e.to_string()))?;
        let mut cursor_query = query.cursor_by(Column::CreatedAt);
        let query = convert_criteria_cursor::<Column, Model>(criteria.cursor(), &mut cursor_query);

        let posts = query
            .all(&self.db)
            .await
            .map(|u| u.into_iter().map(from_model).collect::<Vec<Post>>())
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        Ok(posts)
    }

    async fn find_all(&self) -> Result<Vec<Post>, BaseRepositoryError> {
        let posts = Entity::find()
            .all(&self.db)
            .await
            .map(|u| u.into_iter().map(from_model).collect::<Vec<Post>>())
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        Ok(posts)
    }

    async fn save(&self, post: &Post) -> Result<(), BaseRepositoryError> {
        let on_conflict = OnConflict::column(Column::Id)
            .update_columns(vec![
                Column::UserId,
                Column::Content,
                Column::PostPicture,
                Column::IsNsfw,
                Column::CreatedAt,
                Column::UpdatedAt,
            ])
            .to_owned();

        let post = ActiveModel {
            id: Set(Uuid::parse_str(&post.id()).unwrap()),
            user_id: Set(Uuid::parse_str(&post.user_id()).unwrap()),
            content: Set(post.content()),
            post_picture: Set(post.picture()),
            is_nsfw: Set(post.is_nsfw()),
            created_at: Set(post.created_at()),
            updated_at: Set(post.updated_at()),
        };

        Entity::insert(post)
            .on_conflict(on_conflict)
            .exec(&self.db)
            .await
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: PostId) -> Result<(), BaseRepositoryError> {
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

    use crate::{posts::domain::post::tests::PostMother, test_utils::get_db_image};

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

        let repo = SeaPostRepo::new(db);

        let post = PostMother::random();

        // Save
        repo.save(&post).await.expect("Error saving post");

        let post_id = PostId::new(post.id()).unwrap();

        // Find by id
        let found_post = repo
            .find_by_id(post_id.clone())
            .await
            .expect("Error finding post by id");
        assert_eq!(found_post, post);

        // Find all
        let posts = repo.find_all().await.expect("Error finding all posts");

        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0], post);

        // Find by criteria
        let criteria = Criteria::new(
            vec![Filter::new(
                FilterField::try_from("user_id".to_string()).unwrap(),
                FilterOperator::Equal,
                FilterValue::try_from(post.user_id()).unwrap(),
            )],
            None,
            Some(Cursor::new(
                None,
                None,
                Some(FirstField::new(1).unwrap()),
                None,
            )),
        );
        let posts = repo
            .find_by_criteria(criteria)
            .await
            .expect("Error finding post by criteria");

        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0], post);

        // Delete
        repo.delete(post_id)
            .await
            .expect("Error deleting post by id");

        let posts = repo.find_all().await.expect("Error finding all posts");

        assert_eq!(posts.len(), 0);
    }
}
