import type { ObjectOwner } from "@mysten/sui/client";

export function getActualOwner(objectOwner?: ObjectOwner | null): string | null {
  if (!objectOwner) {
    return null;
  }

  if (typeof objectOwner === "string") {
    // Immutable objects are returned as plain strings
    return "Immutable";
  }

  if ("AddressOwner" in objectOwner) {
    return objectOwner.AddressOwner;
  }

  if ("ObjectOwner" in objectOwner) {
    return objectOwner.ObjectOwner;
  }

  if ("Shared" in objectOwner) {
    return `Shared-${objectOwner.Shared.initial_shared_version}`;
  }

  if ("ConsensusV2" in objectOwner) {
    return `ConsensusV2-${objectOwner.ConsensusV2.start_version}`;
  }

  return null;
}
