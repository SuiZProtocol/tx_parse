import type{ ObjectOwner } from "@mysten/sui/client";

export function getActualOwner(objectOwner: ObjectOwner): string | null {
  if (typeof objectOwner === 'string') {
    // process immutable
    return 'Immutable';
  }
  
  if ('AddressOwner' in objectOwner) {
    // process address owner
    return objectOwner.AddressOwner;
  }
  
  if ('ObjectOwner' in objectOwner) {
    // process object owner
    return objectOwner.ObjectOwner;
  }

  if ('Shared' in objectOwner) {
    // process shared object
    return `Shared-${objectOwner.Shared.initial_shared_version}`;
  }

  if ('ConsensusV2' in objectOwner) {
    // process consensus v2
    return `ConsensusV2-${objectOwner.ConsensusV2.start_version}`;
  }

  return null;
}
