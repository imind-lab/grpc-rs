pub mod micro;

use micro::{
    CreateMicroRequest, 
    Micro as ProtoMicro,
};

use serde::{Deserialize, Serialize};
use redis_derive::{FromRedisValue, ToRedisArgs};
use util::fmt_timestamp;

#[derive(Debug, Serialize, sqlx::FromRow, FromRedisValue, ToRedisArgs, Clone)]
pub struct Micro {
    id: u32,
    name: String,
    view_num: u32,
    typ: i8,
    update_ts: i64,
    create_ts: i64,
}

impl From<Micro> for ProtoMicro {
    fn from(model: Micro) -> Self {
        ProtoMicro {
            id: model.id,
            name: model.name,
            view_num: model.view_num,
            typ: model.typ as _,
            create_datetime: fmt_timestamp(model.create_ts, "%Y-%m-%d %H:%M:%S"),
            update_datetime: fmt_timestamp(model.update_ts, "%Y-%m-%d %H:%M:%S"),
        }
    }
}


#[derive(Deserialize, Debug)]
pub struct CreateMicro {
    pub name: String,
    pub typ: i8,
}

impl From<CreateMicroRequest> for CreateMicro {
    fn from(req: CreateMicroRequest) -> Self {
        CreateMicro {
            name: req.name,
            typ: req.typ as _,
        }
    }
}