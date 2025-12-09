use super::helpers::calculate_atr;
use crate::services::VolatilityDurationAnalyzer;
use chrono::{Duration, Timelike};

pub struct RetroAnalysisService;

impl RetroAnalysisService {
    pub async fn compute_peak_delay(
        pair: &str,
        _event_type: &str,
        events: &[crate::models::CalendarEvent],
        loader: &crate::services::DatabaseLoader,
    ) -> Result<(Vec<i16>, Vec<f64>), String> {
        let mut peak_delays = Vec::new();
        let mut peak_atrs = Vec::new();

        for event in events {
            let window_start = event.event_time.and_utc() - Duration::hours(2);
            let window_end = event.event_time.and_utc() + Duration::hours(2);
            let candles = loader
                .load_candles_by_pair(pair, "M1", window_start, window_end)
                .unwrap_or_default();

            if !candles.is_empty() {
                let atr_values: Vec<f64> =
                    candles.iter().map(|c| calculate_atr(c.high, c.low, c.close)).collect();
                if let Ok(pd) = VolatilityDurationAnalyzer::calculate_peak_delay(
                    &atr_values,
                    event.event_time.minute() as u8,
                ) {
                    peak_delays.push(pd);
                }
                peak_atrs.push(atr_values.iter().copied().fold(f64::NEG_INFINITY, f64::max));
            }
        }

        if peak_delays.is_empty() {
            return Err(format!("Cannot compute peak: {}", pair));
        }

        Ok((peak_delays, peak_atrs))
    }

    pub async fn compute_decay_profile(
        pair: &str,
        events: &[crate::models::CalendarEvent],
        loader: &crate::services::DatabaseLoader,
    ) -> Result<(Vec<f64>, Vec<f64>), String> {
        let mut decay_rates = Vec::new();
        let mut peak_atrs = Vec::new();

        for event in events {
            let window_start = event.event_time.and_utc() - Duration::hours(1);
            let window_end = event.event_time.and_utc() + Duration::hours(3);
            let candles = loader
                .load_candles_by_pair(pair, "M1", window_start, window_end)
                .unwrap_or_default();

            if !candles.is_empty() {
                let atr_values: Vec<f64> =
                    candles.iter().map(|c| calculate_atr(c.high, c.low, c.close)).collect();
                if let Ok((rate, _)) = VolatilityDurationAnalyzer::calculate_decay_profile(&atr_values)
                {
                    decay_rates.push(rate);
                }
                peak_atrs.push(atr_values.iter().copied().fold(f64::NEG_INFINITY, f64::max));
            }
        }

        if decay_rates.is_empty() {
            return Err(format!("Cannot compute decay: {}", pair));
        }

        Ok((decay_rates, peak_atrs))
    }

    /// Analyser l'impact d'un événement sur la volatilité d'une paire
    /// Charge TOUS les candles M1 sur la période complète du calendrier
    /// Trouve TOUTES les occurrences de l'événement
    /// Calcule ATR + Body% + Noise Ratio avant/pendant/après chaque occurrence
    /// Retourne les courbes moyennes comparatives
    pub async fn compute_event_impact(
        pair: &str,
        event_type: &str,
        events: &[crate::models::CalendarEvent],
        loader: &crate::services::DatabaseLoader,
    ) -> Result<super::types::EventImpactResult, String> {
        if events.is_empty() {
            return Err("No events found".into());
        }

        // Déterminer la plage de dates complète du calendrier
        let date_min = events.first().map(|e| e.event_time.and_utc()).unwrap();
        let date_max = events.last().map(|e| e.event_time.and_utc()).unwrap();

        // Charger TOUS les candles M1 sur la période complète
        let all_candles = loader
            .load_candles_by_pair(pair, "M1", date_min, date_max)
            .unwrap_or_default();

        if all_candles.is_empty() {
            return Err(format!("No candle data for: {}", pair));
        }

        // Accumulateurs pour les moyennes
        let mut atr_before_sum = vec![0.0; 30];  // T-30 à T0
        let mut atr_after_sum = vec![0.0; 90];   // T0 à T+90
        let mut body_before_sum = vec![0.0; 30];
        let mut body_after_sum = vec![0.0; 90];
        let mut counts_before = vec![0usize; 30];
        let mut counts_after = vec![0usize; 90];
        let mut noise_before_sum = 0.0;
        let mut noise_during_sum = 0.0;
        let mut noise_after_sum = 0.0;
        let mut event_count = 0;

        // Pour chaque occurrence de l'événement
        for event in events {
            event_count += 1;
            let event_time = event.event_time.and_utc();
            let window_start = event_time - Duration::minutes(30);
            let window_end = event_time + Duration::minutes(90);

            // Filtrer les candles pour cette occurrence
            let occurrence_candles: Vec<_> = all_candles
                .iter()
                .filter(|c| c.datetime >= window_start && c.datetime <= window_end)
                .collect();

            if occurrence_candles.len() < 120 {
                continue; // Skip si pas assez de données
            }

            // Calculer ATR et Body% pour chaque candle
            let mut atrs = Vec::new();
            let mut bodies = Vec::new();
            for candle in &occurrence_candles {
                let atr = calculate_atr(candle.high, candle.low, candle.close);
                let range = candle.high - candle.low;
                let body = (candle.close - candle.open).abs();
                let body_pct = if range > 0.0 { (body / range) * 100.0 } else { 0.0 };

                atrs.push(atr);
                bodies.push(body_pct);
            }

            // Séparer avant (T-30 à T0) et après (T0 à T+90)
            let event_index = 30; // L'événement est à l'indice 30 (30 min après le début)

            // AVANT (T-30 à T0)
            for i in 0..event_index.min(atrs.len()) {
                atr_before_sum[i] += atrs[i];
                body_before_sum[i] += bodies[i];
                counts_before[i] += 1;
            }

            // APRÈS (T0 à T+90)
            for i in event_index..atrs.len().min(event_index + 90) {
                let idx = i - event_index;
                atr_after_sum[idx] += atrs[i];
                body_after_sum[idx] += bodies[i];
                counts_after[idx] += 1;
            }

            // Calculer Noise Ratio pour avant/pendant/après
            // Noise Ratio = Range / Body (avant event)
            for i in 0..event_index.min(atrs.len()) {
                let range = if bodies[i] > 0.0 { 100.0 / bodies[i] } else { 1.0 };
                noise_before_sum += range;
            }
            // Noise pendant event (1 candle autour du T0)
            if event_index < atrs.len() {
                let range = if bodies[event_index] > 0.0 { 100.0 / bodies[event_index] } else { 1.0 };
                noise_during_sum += range;
            }
            // Noise après event
            for i in (event_index + 1)..atrs.len().min(event_index + 90) {
                let range = if bodies[i] > 0.0 { 100.0 / bodies[i] } else { 1.0 };
                noise_after_sum += range;
            }
        }

        // Calculer les moyennes
        let mut atr_timeline_before = vec![0.0; 30];
        let mut atr_timeline_after = vec![0.0; 90];
        let mut body_timeline_before = vec![0.0; 30];
        let mut body_timeline_after = vec![0.0; 90];

        for i in 0..30 {
            if counts_before[i] > 0 {
                atr_timeline_before[i] = atr_before_sum[i] / counts_before[i] as f64;
                body_timeline_before[i] = body_before_sum[i] / counts_before[i] as f64;
            }
        }

        for i in 0..90 {
            if counts_after[i] > 0 {
                atr_timeline_after[i] = atr_after_sum[i] / counts_after[i] as f64;
                body_timeline_after[i] = body_after_sum[i] / counts_after[i] as f64;
            }
        }

        let noise_before = if event_count > 0 { noise_before_sum / (event_count as f64 * 30.0) } else { 0.0 };
        let noise_during = if event_count > 0 { noise_during_sum / event_count as f64 } else { 0.0 };
        let noise_after = if event_count > 0 { noise_after_sum / (event_count as f64 * 89.0) } else { 0.0 };

        // Calculer l'augmentation de volatilité
        let atr_mean_before = atr_timeline_before.iter().sum::<f64>() / 30.0;
        let atr_mean_after = atr_timeline_after.iter().sum::<f64>() / 90.0;
        let volatility_increase = if atr_mean_before > 0.0 {
            ((atr_mean_after - atr_mean_before) / atr_mean_before) * 100.0
        } else {
            0.0
        };

        // Calculer l'heure moyenne de l'événement
        let avg_timestamp = if event_count > 0 {
            events.iter().take(event_count).map(|e| e.event_time.and_utc().timestamp()).sum::<i64>() / event_count as i64
        } else {
            0
        };
        let event_datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(avg_timestamp, 0)
            .map(|dt| dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
            .unwrap_or_else(|| "Unknown".into());

        // === CALCUL DES PARAMÈTRES BIDI POUR STRADDLE ===
        let bidi_params = Self::calculate_bidi_parameters_from_impact(
            &atr_timeline_before,
            &atr_timeline_after,
            noise_during,
            volatility_increase,
            event_count,
        );

        Ok(super::types::EventImpactResult {
            atr_timeline_before,
            atr_timeline_after,
            body_timeline_before,
            body_timeline_after,
            noise_ratio_before: noise_before,
            noise_ratio_during: noise_during,
            noise_ratio_after: noise_after,
            volatility_increase_percent: volatility_increase,
            event_count,
            event_type: event_type.into(),
            pair: pair.into(),
            event_datetime,
            timezone_offset: "UTC+0".into(),
            meilleur_moment: bidi_params.0,
            stop_loss: bidi_params.1,
            trailing_stop: bidi_params.2,
            timeout: bidi_params.3,
        })
    }

    /// Calcul des 4 paramètres Bidi à partir des données d'impact
    /// Retourne: (meilleur_moment en min, stop_loss en points, trailing_stop coefficient, timeout en min)
    fn calculate_bidi_parameters_from_impact(
        atr_before: &[f64],
        atr_after: &[f64],
        noise_during: f64,
        volatility_increase: f64,
        event_count: usize,
    ) -> (f64, f64, f64, i32) {
        // 1. MEILLEUR MOMENT: Pic de volatilité avant événement (dernière minute avant T0)
        // Chercher le pic dans les 5 dernières minutes avant T0 (indices 25-29)
        let mut best_moment = 0.0;
        if !atr_before.is_empty() {
            let peak_idx = atr_before
                .iter()
                .enumerate()
                .rev()
                .take(5)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| i)
                .unwrap_or(29);
            best_moment = (29.0 - peak_idx as f64).max(0.0);
        }

        // 2. STOP LOSS: Basé sur l'ATR moyen (déjà en pips dans les timelines)
        // ATR timeline est stockée en pips (ex: 41, 45, etc.)
        let atr_mean = (atr_before.iter().sum::<f64>() + atr_after.iter().sum::<f64>()) / (atr_before.len() + atr_after.len()) as f64;
        
        // Adapter le ratio de SL selon le noise ratio (1.5 = clean, 2.5 = choppy, 3.5 = very choppy)
        let sl_ratio = if noise_during > 3.0 {
            3.0 // Noise très élevé = SL large
        } else if noise_during > 2.5 {
            2.5
        } else if noise_during > 2.0 {
            2.0
        } else if noise_during > 1.5 {
            1.75
        } else {
            1.5 // Noise bas = SL plus serré
        };
        
        // SL = ATR × ratio (résultat en pips directement)
        let stop_loss = (atr_mean * sl_ratio).ceil();

        // 3. TRAILING STOP: Basé sur ATR × coefficient adapté au noise
        // Formule: ATR × 0.75 × (1 - noise_factor / 2.0)
        let noise_factor = (noise_during / 3.0).min(1.0); // Normaliser à max 1.0
        let ts_coefficient = 0.75 * (1.0 - noise_factor / 2.0); // Coefficient 0.375-0.75
        let trailing_stop = atr_mean * ts_coefficient; // En pips, comme le SL

        // 4. TIMEOUT: Basé sur la décroissance de volatilité après événement
        // Chercher où la volatilité revient à 70% du pic post-événement
        let peak_after = atr_after.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let threshold = peak_after * 0.7;
        let mut timeout = 60; // Défaut: 60 min
        
        if peak_after > 0.0 {
            for (i, &atr) in atr_after.iter().enumerate() {
                if atr <= threshold && i > 0 {
                    timeout = (i as i32).min(60);
                    break;
                }
            }
            // Si nunca desciende, timeout basé sur la volatility increase
            if timeout == 60 && volatility_increase > 50.0 {
                timeout = 35; // Volatilité très élevée = exit rapide
            } else if timeout == 60 && volatility_increase < 10.0 {
                timeout = 50; // Faible volatilité = exit plus tard
            }
        }

        (best_moment, stop_loss, trailing_stop, timeout)
    }
}
