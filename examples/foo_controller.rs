use sai::{Component, Injected, async_trait};
#[cfg(not(test))]
use crate::db::Db;
#[cfg(test)]
use crate::db::MockDb as Db;

#[derive(Component)]
pub struct FooController {

    #[injected]
    db: Injected<Db>
}

impl FooController {
    pub fn index (&self) -> Result<String, tide::Error> {
        self.db.query();
        Ok("Hello Foo".to_string())
    }

    pub async fn async_index(&self) {
        self.db.query_async();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index () {
        let mut db = Db::new();
        db.expect_query()
            .returning(|| ())
            .times(1);

        let x = FooController {
            db: Injected::new(db)
        };


        assert_eq!(x.index().unwrap(), "Hello Foo");
    }


    #[tokio::test]
    async fn test_async_index () {
        let mut db = Db::new();

        let x = FooController {
            db: Injected::new(db)
        };

        x.async_index().await
    }
}
