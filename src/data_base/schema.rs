table! {
    customers (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    stores (id) {
        id -> Int4,
        name -> Varchar,
        clients -> Nullable<Int4>,
    }
}

joinable!(stores -> customers (clients));

allow_tables_to_appear_in_same_query!(
    customers,
    stores,
);
