use crate::db;
use crate::error_handler::CustomError;
use crate::schema::dogs;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "dogs"]
pub struct Dog {
    pub name: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "dogs"]
pub struct Dogs {
    pub id: i32,
    pub name: String,
}

impl Dogs {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let dogs = dogs::table.load::<Dogs>(&conn)?;
        Ok(dogs)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let dog = dogs::table.filter(dogs::id.eq(id)).first(&conn)?;
        Ok(dog)
    }

    pub fn create(dog: Dog) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let dog = Dog::from(dog);
        let dog = diesel::insert_into(dogs::table)
            .values(dog)
            .get_result(&conn)?;
        Ok(dog)
    }

    pub fn update(id: i32, dog: Dog) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let dog = diesel::update(dogs::table)
            .filter(dogs::id.eq(id))
            .set(dog)
            .get_result(&conn)?;
        Ok(dog)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(dogs::table.filter(dogs::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl Dog {
    fn from(dog: Dog) -> Dog {
        Dog {
            name: dog.name,
        }
    }
}