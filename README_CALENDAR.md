# 📅 Calendrier Économique - Guide d'utilisation

## 🎯 Objectif

Le système de calendrier économique permet de **corréler les événements économiques** (NFP, décisions BCE/Fed, etc.) **avec les pics de volatilité Forex**. Cela aide à identifier si les mouvements de prix importants coïncident avec des annonces économiques majeures.

## 📥 Comment télécharger les données CSV

### Depuis Investing.com

1. **Accédez au calendrier économique**
   - URL : https://www.investing.com/economic-calendar/
   - Version française : https://fr.investing.com/economic-calendar/

2. **Sélectionnez la période**
   - Utilisez le sélecteur de dates en haut de la page
   - Recommandé : minimum 3-6 mois de données historiques
   - Pour analyse BTCUSD : privilégier événements USD (Fed, NFP, CPI, etc.)

3. **Filtrez par importance**
   - Cliquez sur "Filters" (Filtres)
   - Sélectionnez uniquement **High Impact** (Impact élevé)
   - Les événements HIGH sont les plus susceptibles de créer de la volatilité

4. **Filtrez par devise**
   - Pour BTCUSD/EURUSD : sélectionnez **USD** et **EUR**
   - Pour GBPUSD : ajoutez **GBP**
   - Pour paires JPY : ajoutez **JPY**

5. **Exportez en CSV**
   - Cliquez sur l'icône de téléchargement (⬇️ Download)
   - Sélectionnez **CSV Format**
   - Enregistrez le fichier (ex: `economic_events_2024.csv`)

### Autres sources possibles

- **ForexFactory** : https://www.forexfactory.com/calendar
- **TradingEconomics** : https://tradingeconomics.com/calendar
- **Conversion manuelle** : Si vous avez des données dans un autre format, assurez-vous qu'elles respectent le format ci-dessous

## 📋 Format CSV attendu

Le fichier CSV doit contenir **exactement ces colonnes** dans cet ordre :

```csv
Date,Time,Currency,Event,Impact,Actual,Forecast,Previous
2024-01-05,14:30,USD,Non-Farm Payrolls,HIGH,216000,170000,199000
2024-01-11,18:00,EUR,ECB Interest Rate Decision,HIGH,4.50,4.50,4.25
2024-02-01,19:00,USD,FOMC Statement,HIGH,,,5.50
2024-02-02,13:30,USD,Unemployment Rate,MEDIUM,3.7,3.8,3.7
```

### Description des colonnes

| Colonne    | Type   | Description                                      | Exemple            |
|------------|--------|--------------------------------------------------|--------------------|
| `Date`     | String | Date de l'événement (YYYY-MM-DD)                 | `2024-01-05`       |
| `Time`     | String | Heure de l'événement (HH:MM format 24h)          | `14:30`            |
| `Currency` | String | Devise concernée (USD, EUR, GBP, JPY...)         | `USD`              |
| `Event`    | String | Nom de l'événement                               | `Non-Farm Payrolls`|
| `Impact`   | String | Niveau d'impact : **HIGH**, MEDIUM, ou LOW       | `HIGH`             |
| `Actual`   | Nombre | Valeur réelle publiée (peut être vide)           | `216000`           |
| `Forecast` | Nombre | Valeur prévue par les analystes (peut être vide) | `170000`           |
| `Previous` | Nombre | Valeur précédente (peut être vide)               | `199000`           |

### ⚠️ Points importants

- **Encodage** : Le fichier doit être en UTF-8
- **Séparateur** : Les colonnes doivent être séparées par des virgules (`,`)
- **Pas d'espace** : Pas d'espaces autour des virgules
- **Valeurs vides** : Les colonnes Actual/Forecast/Previous peuvent être vides (mais les virgules doivent être présentes)
- **Format date** : YYYY-MM-DD uniquement
- **Format heure** : HH:MM en format 24h

### ✅ Exemple de fichier valide

```csv
Date,Time,Currency,Event,Impact,Actual,Forecast,Previous
2024-01-05,14:30,USD,Non-Farm Payrolls,HIGH,216000,170000,199000
2024-01-11,18:00,EUR,ECB Interest Rate Decision,HIGH,4.50,4.50,4.25
2024-01-25,19:00,USD,FOMC Statement,HIGH,,,5.50
2024-02-01,13:30,USD,Initial Jobless Claims,MEDIUM,214000,220000,202000
2024-02-05,08:30,GBP,BoE Interest Rate Decision,HIGH,5.25,5.25,5.25
```

## 🔧 Utilisation dans l'application

### 1. Importer le fichier CSV

1. **Ouvrez l'application** (depuis le terminal Fedora natif)
   ```bash
   cd "/home/rono/Analyse historiques/Analyses-historiques"
   npm run tauri dev
   ```

2. **Accédez à l'onglet "📅 Calendrier ML"**
   - Cliquez sur le bouton dans la barre de navigation

3. **Importez le CSV**
   - Copiez le chemin complet de votre fichier CSV
   - Collez-le dans le champ "Chemin complet du fichier CSV"
   - Exemple : `/home/rono/Downloads/economic_events_2024.csv`
   - Cliquez sur **"📥 Importer"**

4. **Vérifiez l'import**
   - Un message de succès apparaît : "✅ X événements importés avec succès !"
   - Si erreur : vérifiez le format du CSV (voir section Troubleshooting)

### 2. Analyser les corrélations

1. **Retournez à l'onglet "📊 Analyse Volatilité"**

2. **Sélectionnez un symbole** (ex: BTCUSD)

3. **Lancez l'analyse**
   - Le système charge les données de volatilité
   - Cherche les événements économiques dans la base de données
   - Calcule les corrélations entre événements et pics de volatilité

4. **Consultez les résultats**
   - Section **"📅 Événements Économiques Corrélés"** apparaît si des corrélations sont trouvées
   - Chaque carte affiche :
     - 📅 Date et heure de l'événement
     - Badge coloré : **HIGH** (rouge), **MEDIUM** (orange), **LOW** (vert)
     - Heure de volatilité correspondante
     - Augmentation de volatilité (en %)
     - Score de corrélation
     - Données économiques (Réel, Prévu, Précédent)

### 3. Interprétation des résultats

#### Score de corrélation
- **> 80** : Corrélation très forte → L'événement a probablement causé le pic
- **60-80** : Corrélation forte → L'événement est un facteur important
- **40-60** : Corrélation modérée → L'événement peut avoir contribué
- **< 40** : Corrélation faible → Coïncidence possible

#### Augmentation de volatilité
- **> 50%** : Pic de volatilité extrême
- **30-50%** : Volatilité élevée
- **15-30%** : Volatilité modérée
- **< 15%** : Volatilité légère

#### Couleurs d'impact
- 🔴 **RED (HIGH)** : Événements majeurs (NFP, taux d'intérêt, inflation)
- 🟠 **ORANGE (MEDIUM)** : Événements importants (chômage, ventes au détail)
- 🟢 **GREEN (LOW)** : Événements mineurs (indicateurs régionaux)

## 🐛 Troubleshooting

### Erreur : "Failed to load CSV"

**Cause** : Le fichier n'existe pas ou le chemin est incorrect

**Solution** :
1. Vérifiez que le fichier existe : `ls -la /chemin/vers/fichier.csv`
2. Utilisez le chemin absolu complet (pas de `~` ou chemins relatifs)
3. Vérifiez les permissions : `chmod 644 /chemin/vers/fichier.csv`

### Erreur : "Invalid CSV format"

**Cause** : Le CSV ne respecte pas le format attendu

**Solution** :
1. Ouvrez le CSV dans un éditeur de texte
2. Vérifiez la première ligne (header) :
   ```
   Date,Time,Currency,Event,Impact,Actual,Forecast,Previous
   ```
3. Vérifiez qu'il n'y a pas d'espaces avant/après les virgules
4. Vérifiez l'encodage (doit être UTF-8)
5. Convertissez si nécessaire : `iconv -f ISO-8859-1 -t UTF-8 input.csv > output.csv`

### Erreur : "Failed to parse date"

**Cause** : Format de date incorrect

**Solution** :
1. Les dates doivent être au format `YYYY-MM-DD` (ex: `2024-01-05`)
2. Pas de formats comme `01/05/2024` ou `5 Jan 2024`
3. Utilisez un script pour convertir si nécessaire

### Erreur : "Database error"

**Cause** : Problème avec la base de données SQLite

**Solution** :
1. Vérifiez que le fichier `volatility.db` existe dans `src-tauri/`
2. Vérifiez les permissions : `chmod 644 src-tauri/volatility.db`
3. En dernier recours, supprimez la DB et relancez l'app (elle sera recréée)

### Aucun événement corrélé affiché

**Causes possibles** :
1. **Pas d'événements dans la période analysée** : Les données de volatilité et les événements ne se chevauchent pas
2. **Pas d'événements HIGH** : Seuls les événements HIGH créent généralement de la volatilité
3. **Symbole incorrect** : Les événements USD n'affecteront pas forcément EURJPY

**Solution** :
1. Importez plus de données CSV (6-12 mois)
2. Filtrez par événements HIGH uniquement dans Investing.com
3. Vérifiez que le symbole correspond aux devises des événements (ex: BTCUSD → événements USD)

### L'app ne compile pas depuis VSCode

**Cause** : VSCode terminal ne peut pas linker contre webkit

**Solution** :
```bash
# Ouvrez un terminal FEDORA NATIF (Konsole, GNOME Terminal)
cd "/home/rono/Analyse historiques/Analyses-historiques"
npm run tauri dev
```

**NE PAS** compiler depuis le terminal intégré VSCode.

## 📊 Exemples d'utilisation

### Exemple 1 : Analyse NFP pour BTCUSD

1. Téléchargez les NFP (Non-Farm Payrolls) des 6 derniers mois depuis Investing.com
2. Filtrez par USD, HIGH impact uniquement
3. Importez le CSV
4. Analysez BTCUSD
5. Résultat attendu : Corrélations fortes (>70) les premiers vendredis du mois à 14:30 UTC

### Exemple 2 : Décisions BCE pour EURUSD

1. Téléchargez les décisions de taux BCE depuis Investing.com
2. Filtrez par EUR, HIGH impact
3. Importez le CSV
4. Analysez EURUSD
5. Résultat attendu : Pics de volatilité les jeudis à 13:45 UTC (annonces BCE)

### Exemple 3 : Corrélation multi-devises

1. Téléchargez événements USD + EUR + GBP + JPY
2. Importez le CSV complet
3. Analysez plusieurs paires (EURUSD, GBPUSD, USDJPY)
4. Comparez les patterns de corrélation entre les paires

## 🔍 Commandes disponibles

### Backend (Rust)

```rust
// Dans src-tauri/src/commands/economic_commands.rs
#[tauri::command]
pub async fn load_economic_events_from_csv(
    csv_path: String,
    state: State<'_, CalendarState>,
) -> Result<usize, String>
```

### Frontend (Vue/TypeScript)

```typescript
// Dans src/components/CalendarView.vue
import { invoke } from '@tauri-apps/api/core'

const count = await invoke<number>('load_economic_events_from_csv', {
  csvPath: '/path/to/file.csv'
})
```

## 📚 Ressources

- **Investing.com Economic Calendar** : https://www.investing.com/economic-calendar/
- **ForexFactory Calendar** : https://www.forexfactory.com/calendar
- **Documentation Tauri** : https://tauri.app/
- **Documentation Diesel** : https://diesel.rs/

## 🎓 Prochaines étapes

1. **Testez avec différentes paires Forex**
   - EURUSD (événements EUR + USD)
   - GBPUSD (événements GBP + USD)
   - USDJPY (événements USD + JPY)

2. **Affinez les filtres**
   - N'importez que les événements HIGH impact
   - Concentrez-vous sur 1-2 devises max par analyse

3. **Analysez les patterns**
   - Notez quels événements créent systématiquement de la volatilité
   - Identifiez les heures récurrentes de pics
   - Utilisez ces patterns pour votre stratégie de trading

4. **Exportez vos découvertes**
   - Prenez des captures d'écran des corrélations intéressantes
   - Notez les scores de corrélation élevés
   - Créez votre propre calendrier de trading basé sur ces données

---

**🚀 Bon trading et bonnes analyses !**
