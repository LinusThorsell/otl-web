// @generated automatically by Diesel CLI.

diesel::table! {
    apikeys (id) {
        id -> Int4,
        apikey -> Text,
        expires -> Timestamp,
    }
}

diesel::table! {
    blog_posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    events (id) {
        id -> Int4,
        title -> Varchar,
        location -> Varchar,
        description -> Varchar,
        body -> Varchar,
        date -> Timestamp,
        url -> Varchar,
        image -> Varchar,
        tour_id -> Nullable<Int4>,
    }
}

diesel::table! {
    scores (id) {
        id -> Int4,
        score -> Int4,
        divcode -> Text,
        event_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    tours (id) {
        id -> Int4,
        title -> Varchar,
        location -> Varchar,
        description -> Varchar,
        body -> Varchar,
        start_date -> Timestamp,
        end_date -> Timestamp,
        url -> Varchar,
        score_count -> Int4,
        image -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        pdga -> Nullable<Int4>,
        firstname -> Text,
        lastname -> Text,
    }
}

diesel::joinable!(events -> tours (tour_id));
diesel::joinable!(scores -> events (event_id));
diesel::joinable!(scores -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    apikeys,
    blog_posts,
    events,
    scores,
    tours,
    users,
);
