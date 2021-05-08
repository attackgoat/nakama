use super::sys::NkDb;

pub struct Db(NkDb);

impl From<NkDb> for Db {
    fn from(db: NkDb) -> Self {
        Self(db)
    }
}
