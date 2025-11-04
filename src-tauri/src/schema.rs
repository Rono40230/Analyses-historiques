// @generated automatically by Diesel CLI.

diesel::table! {
    calendar_events (id) {
        id -> Integer,
        symbol -> Text,
        event_time -> Timestamp,
        event_type -> Text,
        impact_level -> Text,
        actual_value -> Nullable<Float>,
        forecast_value -> Nullable<Float>,
        previous_value -> Nullable<Float>,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    predicted_events (id) {
        id -> Integer,
        event_id -> Integer,
        predicted_probability -> Float,
        confidence_score -> Float,
        model_version -> Text,
        predicted_volatility_increase -> Nullable<Float>,
        created_at -> Timestamp,
    }
}

diesel::joinable!(predicted_events -> calendar_events (event_id));

diesel::allow_tables_to_appear_in_same_query!(calendar_events, predicted_events,);
