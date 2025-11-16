# 📋 LISTE COMPLÈTE DES PARAMÈTRES DU ROBOT BIDI

## 🎯 VERSION
- **Version** : 1.918
- **Copyright** : 2025 nonconnu
- **Type** : Expert Advisor (EA) MQL5

---

## 🔧 PARAMÈTRES D'ENTRÉE (INPUT PARAMETERS)

### 1️⃣ MODE D'EXÉCUTION
| Paramètre | Type | Valeur par défaut | Description |
|-----------|------|-------------------|-------------|
| `ModeManual` | bool | false | Mode d'exécution manuelle (false=Automatique, true=Manuel) |

### 2️⃣ HORAIRES & ÉVÉNEMENTS
| Paramètre | Type | Valeur par défaut | Description |
|-----------|------|-------------------|-------------|
| `EventTime` | string | "16:59:50" | Heure de l'événement (format HH:MM:SS) |
| `EventDays` | string | "Ven" | Jours de trading (Lun,Mar,Mer,Jeu,Ven,Sam,Dim,Semaine,Tous) |
| `EventMonths` | string | "Tous" | Mois de trading (Jan,Fev,Mar,Avr,Mai,Juin,Juil,Aou,...,Tous,SansJuilAou) |
| `EventWeeks` | string | "Toutes" | Semaines de trading (1,2,3,4,5,6,Toutes) |

### 3️⃣ GESTION DU RISQUE & STOP LOSS
| Paramètre | Type | Valeur par défaut | Description | Plage |
|-----------|------|-------------------|-------------|-------|
| `RiskPercent` | double | 1.0 | Pourcentage de risque par trade (%) | Risque doublé pour straddle |
| `StopLossLevelPoints` | int | 0 | Distance SL en points (0 = utiliser référence ou ATR) | >= 0 |
| `StopLossLevelPercent` | double | 30.0 | Niveau de Stop Loss en % de la volatilité/ATR | > 0 |
| `BreakEvenAdjustPercent` | double | 100.0 | Pourcentage d'ajustement du Break Even (100%=BE, 90%=.9*SL) | > 0 |

### 4️⃣ ATR (AVERAGE TRUE RANGE)
| Paramètre | Type | Valeur par défaut | Description | Plage |
|-----------|------|-------------------|-------------|-------|
| `ATRTimeframe` | string | "M5" | Échelle de temps pour l'ATR (M1,M5,M15,M30) | - |
| `ATRPeriod` | int | 14 | Période de l'ATR | 5 à 30 |
| `ATRMultiplier` | double | 2.0 | Multiplicateur de l'ATR pour le Trailing Stop | 0.5 à 5.0 |

### 5️⃣ GESTION DES POSITIONS
| Paramètre | Type | Valeur par défaut | Description | Plage |
|-----------|------|-------------------|-------------|-------|
| `TradeExpiration` | int | 300 | Durée de vie maximale des positions (en minutes) | 1 à 1440 (5h par défaut) |

### 6️⃣ MODE DÉBOGAGE
| Paramètre | Type | Valeur par défaut | Description |
|-----------|------|-------------------|-------------|
| `IsDebugEnabled` | bool | false | Activer le mode de débogage (false=Non, true=Oui) |
| ~~`IsTraceEnabled`~~ | bool | - | Mode de trace (plus verbeux) - **TODO** |

### 7️⃣ PARAMÈTRES DES BOUTONS (UI)
| Paramètre | Type | Valeur par défaut | Description |
|-----------|------|-------------------|-------------|
| `buttonCloseAll` | string | "CloseAllButton" | Nom du bouton pour fermer tous les trades |
| `buttonX` | int | 20 | Position X du bouton (pixels) |
| `buttonY` | int | 20 | Position Y du bouton (pixels) |
| `buttonWidth` | int | 90 | Largeur du bouton (pixels) |
| `buttonHeight` | int | 30 | Hauteur du bouton (pixels) |
| `buttonStraddle` | string | "StartStopButton" | Nom du bouton de démarrage/arrêt du straddle |
| `button2X` | int | 20 | Position X du 2e bouton (pixels) |
| `button2Y` | int | 50 | Position Y du 2e bouton (pixels) |

---

## 📊 INDICATEURS TECHNIQUES

### Bollinger Bands
| Paramètre | Valeur | Description |
|-----------|--------|-------------|
| **Période** | 20 | Fenêtre de calcul |
| **Écart-type** | 2.0 | Nombre d'écarts-types |
| **Timeframe** | PERIOD_CURRENT | Utilise le timeframe du graphique actuel |
| **Utilisation** | Détection de squeeze (volatilité faible) | Prépare le marché à une explosion de volatilité |

### ATR (Average True Range)
| Paramètre | Valeur | Description |
|-----------|--------|-------------|
| **Période** | 14 (configurable: 5-30) | Fenêtre de calcul |
| **Timeframe** | M5 (configurable: M1,M5,M15,M30) | Échelle de calcul |
| **Multiplicateur** | 2.0 (configurable: 0.5-5.0) | Pour Trailing Stop Loss |

---

## 🤖 STRATÉGIE STRADDLE - DÉTAILS D'EXÉCUTION

### Séquence d'Entrée
1. **Attente** : Position fermée + Signal d'événement prêt
2. **Trigger** : Ouverture SIMULTANÉE de deux positions opposées
   - **LEG BUY** : Achat à cours demandé (ASK)
   - **LEG SELL** : Vente à cours offert (BID)
3. **Distance SL** : Basée sur `StopLossLevelPercent` de l'ATR ou des points fixes

### Gestion des Sorties (Trading)
| Scénario | Action | Résultat |
|----------|--------|----------|
| **TP atteint en 1er** | ❌ Situation interdite | Erreur log + Fermeture straddle |
| **SL atteint (LEG)** | ✅ Ferme la jambe perdante | Straddle → One-Leg trading |
| **SL atteint (One-Leg)** | ✅ Ferme la position | Reset straddle complet |
| **TP atteint (One-Leg)** | ✅ Ferme la position gagnante | Reset straddle complet |
| **Timeout** | ✅ Ferme la position | Après `TradeExpiration` minutes |
| **Break Even** | ✅ Ajuste SL du survivor | À `BreakEvenAdjustPercent` |

### Trailing Stop Loss (TSL)
- **Activation** : Une fois une jambe fermée (One-Leg)
- **Multiplicateur** : `ATRMultiplier` × ATR actuel
- **Mise à jour** : À chaque tick (OnTick)
- **Protection** : Suit les gains mais ne revient jamais en arrière

---

## 📈 LOGIQUE DE GESTION DES POSITIONS

### État du Robot
```
[Fermé] 
  ↓ (Signal + Ready)
[Straddle: 2 jambes]
  ↓ (SL atteint sur 1 jambe)
[One-Leg: 1 jambe survivante]
  ↓ (SL ou TP atteint)
[Fermé]
```

### Détection de Clôture
- **Manual** : Bouton "Clôturer trades" si positions ouvertes
- **OnTimer** : Vérifie expiration toutes les secondes
- **OnTradeTransaction** : Détecte automatiquement SL/TP/Raisons fermeture

---

## 🛡️ RISQUE MANAGEMENT

### Calcul du Risque
- **Par jambe** : `RiskPercent` de la balance
- **Total straddle** : `RiskPercent × 2` (2 jambes)
- **Exemple** : RiskPercent=1% → Risque total=2% du compte

### Limite Maximale
- **Max durée** : 300 minutes (5 heures) après entrée
- **Auto-close** : Force fermeture si dépassé

### Protection Compte
- Vérifie `ACCOUNT_MARGIN_FREE` avant entrée
- Logs diagnostic si échec

---

## 🔍 DÉTECTION DE VOLATILITÉ

### Squeeze Detection
| Condition | Seuil | Signification |
|-----------|-------|---------------|
| **Largeur BB actuelle** | < 1.1 × min_largeur_30bars | Bande Bollinger serrée (squeeze) |
| **Action** | Log WARNING | Volatilité imminente |
| **Status** | Commenté en OnTick | Non utilisé actuellement |

---

## 🖱️ CONTRÔLE MANUEL (ModeManual = true)

### Boutons disponibles
1. **"Straddle" (Vert)** : Démarre un straddle manuel
2. **"Clôturer trades"** : Ferme toutes positions ouvertes

### Comportement
- Les boutons apparaissent/disparaissent selon l'état des positions
- Textes en français
- Localisation fixe en haut-gauche du graphique

---

## 📊 DONNÉES AFFICHÉES EN DEBUG

Quand `IsDebugEnabled = true`, affiche :
- ATR actuel
- Positions ouvertes (détails)
- Informations de marché
- Spécifications du symbole
- Données Bollinger Bands
- État des calculs de squeeze

---

## 🎯 RÉSUMÉ DES CONFIGURATIONS CRITIQUES

### Configuration MINIMALE (Recommandée par défaut)
```
RiskPercent = 1.0%
ATRPeriod = 14
ATRMultiplier = 2.0
ATRTimeframe = M5
StopLossLevelPercent = 30%
TradeExpiration = 300 min (5h)
BreakEvenAdjustPercent = 100%
EventTime = 16:59:50
EventDays = Ven
```

### Configuration AGRESSIVE (Plus de risque)
```
RiskPercent = 2.0%
ATRMultiplier = 1.5
StopLossLevelPercent = 25%
TradeExpiration = 240 min (4h)
BreakEvenAdjustPercent = 90%
```

### Configuration CONSERVATRICE (Moins de risque)
```
RiskPercent = 0.5%
ATRMultiplier = 3.0
StopLossLevelPercent = 40%
TradeExpiration = 360 min (6h)
BreakEvenAdjustPercent = 110%
```

---

## 🔗 FICHIERS INCLUS (DÉPENDANCES)

| Include | Utilité |
|---------|---------|
| `<Trade\Trade.mqh>` | Gestion des ordres MQL5 |
| `<Indicators\Indicators.mqh>` | Gestion des indicateurs |
| `<nonconnu\Event.mqh>` | Détection événements économiques |
| `<nonconnu\AccountData.mqh>` | Données compte |
| `<nonconnu\MagicNumber.mqh>` | Identification unique des trades |
| `<nonconnu\MarketHelper.mqh>` | Helpers marché |
| `<nonconnu\Logger.mqh>` | Système de logging |
| `<nonconnu\TradeInfo.mqh>` | Informations trades |
| `<nonconnu\Trader.mqh>` | **Classe principale du trading Straddle** |
| `<nonconnu\Scheduler.mqh>` | Planification (manuel/auto) |
| `<nonconnu\Utils.mqh>` | Utilitaires divers |

---

## ⚡ ÉVÉNEMENTS PRINCIPAUX

| Événement | Déclenchement | Action |
|-----------|--------------|--------|
| **OnInit** | Au démarrage | Initialisation indicateurs, Trader, Event/Scheduler |
| **OnDeinit** | À l'arrêt | Fermeture positions, nettoyage, logs |
| **OnTimer** | Chaque seconde | Gestion expirations, straddle/one-leg, buttons |
| **OnTick** | À chaque tick | Trailing Stop Loss, Trailing TP |
| **OnTradeTransaction** | Lors deal/fermeture | Détecte SL/TP, gère transitions Straddle→One-Leg |
| **OnChartEvent** | Click boutons | Straddle manuel ou Clôture force |

---

## 📌 NOTES IMPORTANTES

1. **Straddle double le risque** : Avec RiskPercent=1%, risque réel=2% (2 jambes)
2. **Break Even obligatoire** : Quand 1 jambe hit SL, l'autre passe en BE
3. **TP interdit avant SL** : Architecture force SL<TP toujours
4. **Timeframe M5 par défaut** : ATR calculé sur M5 pour réactivité
5. **Squeeze non utilisé** : Code commenté dans OnTick
6. **Log verbeux en DEBUG** : Peut impacter performance (désactiver en prod)

---

Generated: 2025-11-16 | Version Bidi: 1.918
