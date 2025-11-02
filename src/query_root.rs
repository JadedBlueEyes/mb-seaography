use async_graphql::dynamic::{Field, FieldFuture, FieldValue, Object, Schema, SchemaError, TypeRef};
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder, QuerySelect};
use entity::prelude::*;

pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    // Define Artist type
    let artist_type = Object::new("Artist")
        .field(Field::new("id", TypeRef::named_nn(TypeRef::INT), |ctx| {
            FieldFuture::new(async move {
                let artist = ctx.parent_value.try_downcast_ref::<entity::artist::Model>()?;
                Ok(Some(FieldValue::value(artist.id)))
            })
        }))
        .field(Field::new("gid", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let artist = ctx.parent_value.try_downcast_ref::<entity::artist::Model>()?;
                Ok(Some(FieldValue::value(artist.gid.to_string())))
            })
        }))
        .field(Field::new("name", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let artist = ctx.parent_value.try_downcast_ref::<entity::artist::Model>()?;
                Ok(Some(FieldValue::value(artist.name.clone())))
            })
        }))
        .field(Field::new("sort_name", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let artist = ctx.parent_value.try_downcast_ref::<entity::artist::Model>()?;
                Ok(Some(FieldValue::value(artist.sort_name.clone())))
            })
        }))
        .field(Field::new("comment", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let artist = ctx.parent_value.try_downcast_ref::<entity::artist::Model>()?;
                Ok(Some(FieldValue::value(artist.comment.clone())))
            })
        }))
        .field(Field::new("type", TypeRef::named(TypeRef::INT), |ctx| {
            FieldFuture::new(async move {
                let artist = ctx.parent_value.try_downcast_ref::<entity::artist::Model>()?;
                Ok(artist.r#type.map(FieldValue::value))
            })
        }))
        .field(Field::new("area", TypeRef::named(TypeRef::INT), |ctx| {
            FieldFuture::new(async move {
                let artist = ctx.parent_value.try_downcast_ref::<entity::artist::Model>()?;
                Ok(artist.area.map(FieldValue::value))
            })
        }))
        .field(Field::new("gender", TypeRef::named(TypeRef::INT), |ctx| {
            FieldFuture::new(async move {
                let artist = ctx.parent_value.try_downcast_ref::<entity::artist::Model>()?;
                Ok(artist.gender.map(FieldValue::value))
            })
        }))
        .field(Field::new("begin_date_year", TypeRef::named(TypeRef::INT), |ctx| {
            FieldFuture::new(async move {
                let artist = ctx.parent_value.try_downcast_ref::<entity::artist::Model>()?;
                Ok(artist.begin_date_year.map(|y| FieldValue::value(y as i32)))
            })
        }))
        .field(Field::new("end_date_year", TypeRef::named(TypeRef::INT), |ctx| {
            FieldFuture::new(async move {
                let artist = ctx.parent_value.try_downcast_ref::<entity::artist::Model>()?;
                Ok(artist.end_date_year.map(|y| FieldValue::value(y as i32)))
            })
        }))
        .field(Field::new("ended", TypeRef::named_nn(TypeRef::BOOLEAN), |ctx| {
            FieldFuture::new(async move {
                let artist = ctx.parent_value.try_downcast_ref::<entity::artist::Model>()?;
                Ok(Some(FieldValue::value(artist.ended)))
            })
        }));

    // Define Area type
    let area_type = Object::new("Area")
        .field(Field::new("id", TypeRef::named_nn(TypeRef::INT), |ctx| {
            FieldFuture::new(async move {
                let area = ctx.parent_value.try_downcast_ref::<entity::area::Model>()?;
                Ok(Some(FieldValue::value(area.id)))
            })
        }))
        .field(Field::new("gid", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let area = ctx.parent_value.try_downcast_ref::<entity::area::Model>()?;
                Ok(Some(FieldValue::value(area.gid.to_string())))
            })
        }))
        .field(Field::new("name", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let area = ctx.parent_value.try_downcast_ref::<entity::area::Model>()?;
                Ok(Some(FieldValue::value(area.name.clone())))
            })
        }))
        .field(Field::new("type", TypeRef::named(TypeRef::INT), |ctx| {
            FieldFuture::new(async move {
                let area = ctx.parent_value.try_downcast_ref::<entity::area::Model>()?;
                Ok(area.r#type.map(FieldValue::value))
            })
        }));

    // Define Query type
    let query = Object::new("Query")
        .field(Field::new(
            "health",
            TypeRef::named_nn(TypeRef::STRING),
            |_ctx| {
                FieldFuture::new(async move {
                    Ok(Some(FieldValue::value("OK")))
                })
            },
        ))
        .field(Field::new(
            "database_connected",
            TypeRef::named_nn(TypeRef::BOOLEAN),
            |ctx| {
                FieldFuture::new(async move {
                    let db = ctx.data::<DatabaseConnection>()?;
                    match sea_orm::DatabaseConnection::ping(db).await {
                        Ok(_) => Ok(Some(FieldValue::value(true))),
                        Err(_) => Ok(Some(FieldValue::value(false))),
                    }
                })
            },
        ))
        .field(Field::new(
            "artist",
            TypeRef::named("Artist"),
            |ctx| {
                FieldFuture::new(async move {
                    let db = ctx.data::<DatabaseConnection>()?;
                    let id = ctx.args.try_get("id")?.i64()? as i32;
                    
                    match Artist::find_by_id(id).one(db).await {
                        Ok(Some(artist)) => Ok(Some(FieldValue::owned_any(artist))),
                        Ok(None) => Ok(None),
                        Err(e) => Err(format!("Database error: {}", e).into()),
                    }
                })
            },
        ).argument(async_graphql::dynamic::InputValue::new("id", TypeRef::named_nn(TypeRef::INT))))
        .field(Field::new(
            "artists",
            TypeRef::named_nn_list_nn("Artist"),
            |ctx| {
                FieldFuture::new(async move {
                    let db = ctx.data::<DatabaseConnection>()?;
                    let limit = ctx.args.try_get("limit").ok().and_then(|v| v.u64().ok()).unwrap_or(10) as u64;
                    let offset = ctx.args.try_get("offset").ok().and_then(|v| v.u64().ok()).unwrap_or(0) as u64;
                    
                    match Artist::find()
                        .order_by_asc(entity::artist::Column::Id)
                        .offset(offset)
                        .limit(limit)
                        .all(db)
                        .await
                    {
                        Ok(artists) => {
                            let values: Vec<FieldValue> = artists
                                .into_iter()
                                .map(FieldValue::owned_any)
                                .collect();
                            Ok(Some(FieldValue::list(values)))
                        }
                        Err(e) => Err(format!("Database error: {}", e).into()),
                    }
                })
            },
        )
        .argument(async_graphql::dynamic::InputValue::new("limit", TypeRef::named(TypeRef::INT)))
        .argument(async_graphql::dynamic::InputValue::new("offset", TypeRef::named(TypeRef::INT))))
        .field(Field::new(
            "artistCount",
            TypeRef::named_nn(TypeRef::INT),
            |ctx| {
                FieldFuture::new(async move {
                    let db = ctx.data::<DatabaseConnection>()?;
                    match Artist::find().count(db).await {
                        Ok(count) => Ok(Some(FieldValue::value(count as i32))),
                        Err(e) => Err(format!("Database error: {}", e).into()),
                    }
                })
            },
        ))
        .field(Field::new(
            "area",
            TypeRef::named("Area"),
            |ctx| {
                FieldFuture::new(async move {
                    let db = ctx.data::<DatabaseConnection>()?;
                    let id = ctx.args.try_get("id")?.i64()? as i32;
                    
                    match Area::find_by_id(id).one(db).await {
                        Ok(Some(area)) => Ok(Some(FieldValue::owned_any(area))),
                        Ok(None) => Ok(None),
                        Err(e) => Err(format!("Database error: {}", e).into()),
                    }
                })
            },
        ).argument(async_graphql::dynamic::InputValue::new("id", TypeRef::named_nn(TypeRef::INT))))
        .field(Field::new(
            "areas",
            TypeRef::named_nn_list_nn("Area"),
            |ctx| {
                FieldFuture::new(async move {
                    let db = ctx.data::<DatabaseConnection>()?;
                    let limit = ctx.args.try_get("limit").ok().and_then(|v| v.u64().ok()).unwrap_or(10) as u64;
                    let offset = ctx.args.try_get("offset").ok().and_then(|v| v.u64().ok()).unwrap_or(0) as u64;
                    
                    match Area::find()
                        .order_by_asc(entity::area::Column::Id)
                        .offset(offset)
                        .limit(limit)
                        .all(db)
                        .await
                    {
                        Ok(areas) => {
                            let values: Vec<FieldValue> = areas
                                .into_iter()
                                .map(FieldValue::owned_any)
                                .collect();
                            Ok(Some(FieldValue::list(values)))
                        }
                        Err(e) => Err(format!("Database error: {}", e).into()),
                    }
                })
            },
        )
        .argument(async_graphql::dynamic::InputValue::new("limit", TypeRef::named(TypeRef::INT)))
        .argument(async_graphql::dynamic::InputValue::new("offset", TypeRef::named(TypeRef::INT))));

    let mut schema_builder = Schema::build(query.type_name(), None, None)
        .register(query)
        .register(artist_type)
        .register(area_type);

    if let Some(depth) = depth {
        schema_builder = schema_builder.limit_depth(depth);
    }

    if let Some(complexity) = complexity {
        schema_builder = schema_builder.limit_complexity(complexity);
    }

    schema_builder.data(database).finish()
}