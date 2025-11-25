/**
 * Composable pour la traduction des noms d'événements économiques
 */

export function useEventTranslation() {

    const translateEventName = (eventName: string): string => {
        const translations: Record<string, string> = {
            // US Events
            'Non-Farm Employment Change': 'Variation de l\'emploi non agricole',
            'Unemployment Rate': 'Taux de chômage',
            'Average Hourly Earnings': 'Salaire horaire moyen',
            'Consumer Price Index': 'Indice des prix à la consommation',
            'Core CPI': 'IPC de base',
            'Producer Price Index': 'Indice des prix à la production',
            'Retail Sales': 'Ventes au détail',
            'Core Retail Sales': 'Ventes au détail de base',
            'GDP Growth Rate': 'Taux de croissance du PIB',
            'GDP': 'PIB',
            'Fed Interest Rate Decision': 'Décision sur les taux de la Fed',
            'FOMC Statement': 'Déclaration du FOMC',
            'FOMC Press Conference': 'Conférence de presse du FOMC',
            'Fed Chair Powell Speaks': 'Discours du président de la Fed Powell',
            'ISM Manufacturing PMI': 'PMI manufacturier ISM',
            'ISM Services PMI': 'PMI des services ISM',
            'Flash Manufacturing PMI': 'PMI manufacturier flash',
            'Flash Services PMI': 'PMI des services flash',
            'Durable Goods Orders': 'Commandes de biens durables',
            'Initial Jobless Claims': 'Nouvelles demandes d\'allocations chômage',
            'Continuing Jobless Claims': 'Demandes continues d\'allocations chômage',
            'Building Permits': 'Permis de construire',
            'Housing Starts': 'Mises en chantier',
            'Existing Home Sales': 'Ventes de logements existants',
            'New Home Sales': 'Ventes de logements neufs',
            'Consumer Confidence': 'Confiance des consommateurs',
            'Michigan Consumer Sentiment': 'Sentiment des consommateurs Michigan',
            'Trade Balance': 'Balance commerciale',
            'Industrial Production': 'Production industrielle',
            'Capacity Utilization': 'Taux d\'utilisation des capacités',
            'Factory Orders': 'Commandes industrielles',
            'Business Inventories': 'Stocks des entreprises',
            'Personal Spending': 'Dépenses personnelles',
            'Personal Income': 'Revenus personnels',
            'PCE Price Index': 'Indice des prix PCE',
            'Core PCE Price Index': 'Indice des prix PCE de base',
            'ADP Non-Farm Employment Change': 'Variation de l\'emploi ADP',
            'Challenger Job Cuts': 'Suppressions d\'emplois Challenger',
            'JOLTS Job Openings': 'Offres d\'emploi JOLTS',
            'Nonfarm Payrolls': 'Emplois non agricoles',
            'NFP': 'Emplois non agricoles',

            // ECB/EU Events
            'ECB Interest Rate Decision': 'Décision sur les taux de la BCE',
            'ECB Press Conference': 'Conférence de presse de la BCE',
            'ECB President Lagarde Speaks': 'Discours de la présidente de la BCE Lagarde',
            'German ZEW Economic Sentiment': 'Sentiment économique ZEW allemand',
            'German IFO Business Climate': 'Climat des affaires IFO allemand',
            'Eurozone CPI': 'IPC de la zone euro',
            'Eurozone GDP': 'PIB de la zone euro',

            // UK Events
            'BoE Interest Rate Decision': 'Décision sur les taux de la BoE',
            'BoE MPC Meeting Minutes': 'Procès-verbal du MPC de la BoE',
            'BoE Gov Bailey Speaks': 'Discours du gouverneur de la BoE Bailey',
            'UK CPI': 'IPC britannique',
            'UK GDP': 'PIB britannique',
            'UK Retail Sales': 'Ventes au détail britanniques',

            // Japan Events
            'BoJ Interest Rate Decision': 'Décision sur les taux de la BoJ',
            'BoJ Press Conference': 'Conférence de presse de la BoJ',
            'BoJ Gov Ueda Speaks': 'Discours du gouverneur de la BoJ Ueda',
            'Japan CPI': 'IPC japonais',
            'Japan GDP': 'PIB japonais',
            'Tankan Survey': 'Enquête Tankan',

            // Canada Events
            'BoC Interest Rate Decision': 'Décision sur les taux de la BdC',
            'BoC Gov Macklem Speaks': 'Discours du gouverneur de la BdC Macklem',
            'Canada CPI': 'IPC canadien',
            'Canada GDP': 'PIB canadien',
            'Canada Employment Change': 'Variation de l\'emploi canadien',

            // Australia Events
            'RBA Interest Rate Decision': 'Décision sur les taux de la RBA',
            'RBA Gov Lowe Speaks': 'Discours du gouverneur de la RBA Lowe',
            'RBNZ Gov Orr Speaks': 'Discours du gouverneur de la RBNZ Orr',
            'Australia CPI': 'IPC australien',
            'Australia GDP': 'PIB australien',

            // China Events
            'China CPI': 'IPC chinois',
            'China GDP': 'PIB chinois',
            'China Manufacturing PMI': 'PMI manufacturier chinois',
            'China Services PMI': 'PMI des services chinois',

            // Political Events
            'President Trump Speaks': 'Discours du président Trump',
            'President Biden Speaks': 'Discours du président Biden',
            'Presidential Election': 'Élection présidentielle',
            'Congressional Testimony': 'Témoignage au Congrès',

            // Other
            'OPEC Meeting': 'Réunion de l\'OPEC',
            'G7 Meeting': 'Réunion du G7',
            'G20 Meeting': 'Réunion du G20',
        }

        return translations[eventName] || eventName
    }

    return {
        translateEventName
    }
}
