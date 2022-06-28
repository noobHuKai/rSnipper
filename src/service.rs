use sled::{Db, Error};

pub struct Tag;

impl Tag {
    pub fn insert(db: &Db, key: &str) -> Result<(), Error> {
        db.insert(key, key)?;
        db.set_merge_operator(concatenate_merge);
        db.merge("$tags", key)?;
        Ok(())
    }

    pub fn get_all(db: &Db) -> Result<(), Error> {
        Ok(())
    }
}

fn concatenate_merge(
    _key: &[u8],              // the key being merged
    old_value: Option<&[u8]>, // the previous value, if one existed
    merged_bytes: &[u8],      // the new bytes being merged in
) -> Option<Vec<u8>> {
    // set the new value, return None to delete
    let mut ret = old_value.map(|ov| ov.to_vec()).unwrap_or_else(|| vec![]);

    let a = ",";
    ret.extend_from_slice(b"a" + merged_bytes);

    Some(ret)
}
