// models/event_movement_quality.rs - Analyse qualité mouvements événementiels
// Conforme .clinerules : structures uniquement, pas de logique métier

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Analyse qualité des mouvements générés par un type d'événement sur une paire
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMovementQuality {
    pub id: Option<i32>,
    pub symbol: String,
    pub event_type: String,
    /// Proportion d'événements générant un mouvement directionnel > ATR × 0.75
    pub directional_move_rate: f64,
    /// Proportion d'événements avec reversal complet dans les 15 minutes
    pub whipsaw_rate: f64,
    /// Mouvement moyen en pips (à titre informatif)
    pub avg_pips_moved: f64,
    /// Proportion d'événements avec succès (mouvement directionnel sans reversal rapide)
    pub success_rate: f64,
    /// Score combiné (0-10) : directional_move_rate + (100 - whipsaw_rate) / 2
    pub quality_score: f64,
    /// Nombre d'occurrences analysées
    pub sample_size: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

impl EventMovementQuality {
    /// Crée une nouvelle instance avec les paramètres essentiels
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        symbol: String,
        event_type: String,
        directional_move_rate: f64,
        whipsaw_rate: f64,
        avg_pips_moved: f64,
        success_rate: f64,
        quality_score: f64,
        sample_size: i32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            symbol,
            event_type,
            directional_move_rate,
            whipsaw_rate,
            avg_pips_moved,
            success_rate,
            quality_score,
            sample_size,
            created_at: now,
            updated_at: now,
        }
    }

    /// Retourne une recommandation basée sur le quality_score
    #[allow(dead_code)]
    pub fn recommendation(&self) -> MovementRecommendation {
        match self.quality_score {
            score if score >= 7.0 => MovementRecommendation::HighQuality,
            score if score >= 5.0 => MovementRecommendation::MediumQuality,
            score if score >= 3.0 => MovementRecommendation::LowQuality,
            _ => MovementRecommendation::AvoidEvent,
        }
    }

    /// Vérifie si l'événement est suffisamment tradable
    #[allow(dead_code)]
    pub fn is_tradable(&self) -> bool {
        self.success_rate > 0.6 && self.quality_score >= 5.0
    }
}

/// Recommandations basées sur la qualité du mouvement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MovementRecommendation {
    HighQuality,
    MediumQuality,
    LowQuality,
    AvoidEvent,
}

impl std::fmt::Display for MovementRecommendation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HighQuality => write!(f, "TRADE"),
            Self::MediumQuality => write!(f, "CAUTION"),
            Self::LowQuality => write!(f, "CAUTION"),
            Self::AvoidEvent => write!(f, "AVOID"),
        }
    }
}

#[cfg(test)]
#[path = "event_movement_quality_tests.rs"]
mod tests;
