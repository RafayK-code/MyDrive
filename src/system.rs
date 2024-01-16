use crate::models;
use crate::database;

/* is this function even needed??????????????
pub async fn get_absolute_path(new_file: models::File, db: &database::Database) -> String {
    let mut absolute_path = new_file.name.to_owned();
    let mut file = file;

    while let Some(ref parent) = dummy.parent {
        absolute_path = parent.to_owned() + "/" + &absolute_path;
        dummy = db.get_file(parent.as_str()).await.unwrap();
    }

    absolute_path
}
*/