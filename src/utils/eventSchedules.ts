import { usSchedules } from './eventSchedules/usSchedules'
import { euroSchedules } from './eventSchedules/euroSchedules'
import { ukSchedules } from './eventSchedules/ukSchedules'
import { asiaSchedules } from './eventSchedules/asiaSchedules'
import { canadaSchedules } from './eventSchedules/canadaSchedules'
import { swissSchedules } from './eventSchedules/swissSchedules'
import { otherSchedules } from './eventSchedules/otherSchedules'

export const eventSchedules: Record<string, string> = {
  ...usSchedules,
  ...euroSchedules,
  ...ukSchedules,
  ...asiaSchedules,
  ...canadaSchedules,
  ...swissSchedules,
  ...otherSchedules,
}

/**
 * Récupère l'horaire habituel d'un événement économique
 * @param eventName Nom de l'événement en anglais
 * @returns Horaire ou "Variable" si non défini
 */
export function obtenirHoraireEvenement(eventName: string): string {
  return eventSchedules[eventName] || 'Variable'
}
