/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} data
* @returns {(Slot)[]}
*/
export function get_slots(data: Uint8Array): (Slot)[];
/**
* @param {Uint8Array} target
* @param {number} target_slot_index
* @param {Uint8Array} source
* @param {number} source_slot_index
* @returns {Uint8Array}
*/
export function replace_slot(target: Uint8Array, target_slot_index: number, source: Uint8Array, source_slot_index: number): Uint8Array;
/**
*/
export class Slot {
  free(): void;
/**
*/
  active: boolean;
/**
*/
  character_level: number;
/**
*/
  character_name: string;
/**
*/
  index: number;
/**
*/
  seconds_played: number;
}

