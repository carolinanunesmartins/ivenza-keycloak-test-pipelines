table! {
#[allow(non_snake_case)]
    UserRoles(id) {
        id -> Integer,
        name -> Text,
        #[sql_name = "displayName"]
        display_name -> Nullable<Text>,
    }
}

table! {
#[allow(non_snake_case)]
    UserRolePermissions(role) {
        role -> Text,
        permission -> Text,
    }
}

table! {
#[allow(non_snake_case)]
    Users(id) {
        id -> Integer,
        #[sql_name = "loginName"]
        login_name -> Text,
        role -> Text,
        email -> Text,
        password -> Nullable<Text>,
        domain -> Nullable<Text>,
        state -> Text,
    }
}
