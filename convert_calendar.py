#!/usr/bin/env python3
"""
Script de conversion du calendrier économique Investing.com
Filtre uniquement les événements MEDIUM et HIGH impact
Convertit au format attendu par l'application
"""

import pandas as pd
import sys
from pathlib import Path

def convert_calendar(input_file, output_file):
    """
    Convertit le fichier Excel Investing.com en CSV filtré
    
    Args:
        input_file: Chemin du fichier Excel source
        output_file: Chemin du fichier CSV de sortie
    """
    print(f"📖 Lecture de {input_file}...")
    
    # Lire le fichier Excel (colonnes A à E uniquement)
    df = pd.read_excel(
        input_file,
        header=None,  # Pas de header
        names=['Date', 'Time', 'Currency', 'Impact', 'Event'],
        usecols=[0, 1, 2, 3, 4]  # Colonnes A à E seulement
    )
    
    print(f"✅ {len(df)} lignes lues")
    
    # Filtrer uniquement M (Medium) et H (High)
    print("🔍 Filtrage des événements MEDIUM et HIGH...")
    df_filtered = df[df['Impact'].isin(['M', 'H'])].copy()
    
    print(f"✅ {len(df_filtered)} événements MEDIUM/HIGH trouvés")
    
    # Convertir Impact en format complet
    impact_map = {
        'H': 'HIGH',
        'M': 'MEDIUM',
        'L': 'LOW',
        'N': 'LOW'
    }
    df_filtered['Impact'] = df_filtered['Impact'].map(impact_map)
    
    # Convertir la date au format YYYY-MM-DD
    df_filtered['Date'] = pd.to_datetime(df_filtered['Date']).dt.strftime('%Y-%m-%d')
    
    # Formatter l'heure au format HH:MM
    def format_time(time_val):
        if pd.isna(time_val):
            return '00:00'
        if isinstance(time_val, str):
            return time_val
        # Si c'est un datetime ou time
        try:
            return time_val.strftime('%H:%M')
        except:
            return '00:00'
    
    df_filtered['Time'] = df_filtered['Time'].apply(format_time)
    
    # Ajouter les colonnes vides pour Actual, Forecast, Previous
    # (requis par le format mais non utilisées)
    df_filtered['Actual'] = ''
    df_filtered['Forecast'] = ''
    df_filtered['Previous'] = ''
    
    # Sauvegarder en CSV
    print(f"💾 Sauvegarde dans {output_file}...")
    df_filtered.to_csv(
        output_file,
        index=False,
        encoding='utf-8',
        columns=['Date', 'Time', 'Currency', 'Event', 'Impact', 'Actual', 'Forecast', 'Previous']
    )
    
    print(f"""
✅ Conversion terminée !

📊 Statistiques :
   - Total lignes source : {len(df)}
   - Événements M/H : {len(df_filtered)}
   - Taux de filtrage : {len(df_filtered)/len(df)*100:.1f}%
   
📁 Fichier créé : {output_file}

🎯 Prochaines étapes :
   1. Lancez l'application : npm run tauri dev (depuis terminal Fedora natif)
   2. Onglet "📅 Calendrier ML"
   3. Importez : {output_file}
""")

    # Afficher un échantillon
    print("\n📋 Aperçu des 5 premiers événements :")
    print(df_filtered.head(5).to_string(index=False))
    
    return len(df_filtered)

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("""
Usage: python3 convert_calendar.py <fichier_excel.xlsx> [fichier_sortie.csv]

Exemple:
  python3 convert_calendar.py ~/Downloads/Calendar_2007-2025.xlsx
  python3 convert_calendar.py ~/Downloads/Calendar.xlsx economic_events.csv
        """)
        sys.exit(1)
    
    input_file = Path(sys.argv[1])
    
    if not input_file.exists():
        print(f"❌ Erreur : Le fichier {input_file} n'existe pas")
        sys.exit(1)
    
    # Fichier de sortie
    if len(sys.argv) >= 3:
        output_file = Path(sys.argv[2])
    else:
        output_file = input_file.with_name(f"{input_file.stem}_filtered.csv")
    
    try:
        count = convert_calendar(input_file, output_file)
        print(f"\n🎉 {count} événements prêts à être importés !")
    except Exception as e:
        print(f"❌ Erreur lors de la conversion : {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
