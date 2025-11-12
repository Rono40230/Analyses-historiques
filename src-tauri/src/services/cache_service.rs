// services/cache_service.rs - Service de caching pour résultats
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Entrée en cache avec timestamp d'expiration
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    data: T,
    expires_at: u64,  // UNIX timestamp
}

/// Service de cache global avec TTL (Time To Live)
pub struct CacheService<T: Clone> {
    cache: Arc<Mutex<HashMap<String, CacheEntry<T>>>>,
    ttl_seconds: u64,
}

impl<T: Clone> CacheService<T> {
    /// Créer un nouveau service de cache
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            ttl_seconds,
        }
    }
    
    /// Obtenir une valeur du cache (retourne None si expiré ou inexistant)
    pub fn get(&self, key: &str) -> Option<T> {
        let now = current_unix_timestamp();
        let cache = self.cache.lock().map_err(|_| ()).ok()?;
        
        if let Some(entry) = cache.get(key) {
            if now < entry.expires_at {
                return Some(entry.data.clone());
            }
        }
        None
    }
    
    /// Mettre une valeur en cache
    pub fn set(&self, key: String, value: T) {
        let expires_at = current_unix_timestamp() + self.ttl_seconds;
        let entry = CacheEntry {
            data: value,
            expires_at,
        };
        
        if let Ok(mut cache) = self.cache.lock() {
            cache.insert(key, entry);
        }
    }
    
    /// Nettoyer les entrées expirées
    pub fn cleanup_expired(&self) {
        let now = current_unix_timestamp();
        if let Ok(mut cache) = self.cache.lock() {
            cache.retain(|_, entry| entry.expires_at > now);
        }
    }
    
    /// Vider le cache complètement
    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }
}

fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_cache_get_set() {
        let cache = CacheService::new(10);
        
        cache.set("key1".to_string(), 42);
        assert_eq!(cache.get("key1"), Some(42));
        assert_eq!(cache.get("key2"), None);
    }
    
    #[test]
    fn test_cache_expiration() {
        let cache = CacheService::new(1);  // 1 second TTL
        
        cache.set("key1".to_string(), 42);
        assert_eq!(cache.get("key1"), Some(42));
        
        thread::sleep(Duration::from_millis(1100));
        assert_eq!(cache.get("key1"), None);
    }
}
