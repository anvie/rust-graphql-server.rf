use crate::database::pool::PgPool;
use crate::errors::$name_pascal_case$Error;
use crate::graphql::model::Context;
use crate::jwt::manager::create_token;
use crate::jwt::model::{HumanClaims, Token};
use crate::user::model::{AuthData, SlimUser, User, UserData};
use crate::user::util::verify;
use actix_web::web;
use diesel::prelude::*;

pub fn user_manager_register(
    user_data: UserData,
    pool: web::Data<PgPool>,
) -> Result<SlimUser, $name_pascal_case$Error> {
    use crate::schema::users::dsl::users;

    let conn: &PgConnection = &pool.get().unwrap();
    let user = User::new(&user_data.email, &user_data.password, &user_data.name);

    let inserted_user: User = diesel::insert_into(users).values(&user).get_result(conn)?;
    return Ok(inserted_user.into());
}

pub fn user_manager_login(
    auth_data: AuthData,
    pool: web::Data<PgPool>,
) -> Result<SlimUser, $name_pascal_case$Error> {
    use crate::schema::users::dsl::{email, users};
    let conn: &PgConnection = &pool.get().unwrap();
    let mut items = users
        .filter(email.eq(&auth_data.email))
        .load::<User>(conn)?;

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.hash, &user.salt, &auth_data.password) {
            if matching {
                return Ok(user.into()); // convert into slimUser
            }
        }
    }
    Err($name_pascal_case$Error::Unauthorized)
}

pub fn user_manager_get_all(context: &Context) -> Result<Vec<User>, $name_pascal_case$Error> {
    use crate::schema::users::dsl::*;
    let conn: &PgConnection = &context.db;

    Ok(users.limit(100).load::<User>(conn)?)
}

pub fn user_manager_get_jwt(context: &Context) -> Result<Token, $name_pascal_case$Error> {
    match &context.user.email {
        None => return Err($name_pascal_case$Error::Unauthorized),
        Some(m) => match create_token(m.as_str()) {
            Ok(r) => return Ok(Token { bearer: Some(r) }),
            Err(e) => return Err(e),
        },
    }
}

pub fn user_manager_get_decode(context: &Context) -> Result<&HumanClaims, $name_pascal_case$Error> {
    match &context.token.jwt {
        None => return Err($name_pascal_case$Error::Unauthorized),
        Some(m) => return Ok(m as &HumanClaims),
    }
}
