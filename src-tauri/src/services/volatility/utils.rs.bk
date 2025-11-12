// services/volatility/utils.rs - Fonctions utilitaires
// Conforme .clinerules : helpers simples, < 50L

/// Calcule la moyenne d'un vecteur
pub(crate) fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

/// Retourne la valeur maximale d'un vecteur
pub(crate) fn max(values: &[f64]) -> f64 {
    values
        .iter()
        .fold(0.0, |acc, &x| if x > acc { x } else { acc })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        assert_eq!(mean(&[1.0, 2.0, 3.0]), 2.0);
        assert_eq!(mean(&[]), 0.0);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(&[1.0, 5.0, 3.0]), 5.0);
        assert_eq!(max(&[]), 0.0);
    }
}
