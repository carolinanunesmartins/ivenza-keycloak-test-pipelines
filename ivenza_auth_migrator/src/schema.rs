table! {
#[allow(non_snake_case)]
    UserRoles(id) {
        id -> Integer,
        name -> Text,
        #[sql_name = "displayName"]
        display_name -> Text,
    }
}

table! {
#[allow(non_snake_case)]
    UserRolePermissions(role) {
        role -> Text,
        permission -> Text,
    }
}