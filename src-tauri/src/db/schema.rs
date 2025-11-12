// db/schema.rs - Schéma Diesel minimal pour calendar_events

diesel::table! {
    calendar_events (id) {
        id -> Integer,
        symbol -> Text,
        event_time -> Timestamp,
        impact -> Text,
        description -> Text,
        actual -> Nullable<Double>,
        forecast -> Nullable<Double>,
        previous -> Nullable<Double>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    ohlc_data (id) {
        id -> Integer,
        symbol -> Text,
        timestamp -> BigInt,
        open -> Double,
        high -> Double,
        low -> Double,
        close -> Double,
        volume -> Double,
    }
}

diesel::allow_tables_to_appear_in_same_query!(calendar_events, ohlc_data,);
