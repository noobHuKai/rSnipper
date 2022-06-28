use sled::{Db, Error};

pub struct Tag;

const TAGSKEY: &str = "$tags";
impl Tag {
    pub fn insert(db: &Db, tag: &str) -> Result<(), Error> {
        match db.get(TAGSKEY)? {
            Some(value) => {
                for s in value.split(|c| c == &b',') {
                    // is exist
                    if s == tag.as_bytes() {
                        return Err(Error::Unsupported("Key Is Exist".to_string()));
                    }
                }
                // not exit insert
                let mut ret = value.to_vec();
                ret.extend(b",");
                ret.extend(tag.as_bytes());
                db.insert(TAGSKEY, ret)?;
            }
            None => {
                // key is not exist
                db.insert(TAGSKEY, tag)?;
            }
        }
        Ok(())
    }

    pub fn get_all(db: &Db) -> Result<Vec<String>, Error> {
        // get value
        let tags = db.get(TAGSKEY)?;

        let res: Vec<String> = match tags {
            Some(t) => {
                let mut a = vec![];
                for s in t.split(|c| c == &b',') {
                    // &[u8]  -> String
                    a.push(String::from_utf8(s.to_vec()).unwrap())
                }
                a
            }
            // not exist
            None => vec![],
        };
        Ok(res)
    }
}
