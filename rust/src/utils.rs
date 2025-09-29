use crate::types::ObjectOwner;

pub(crate) fn get_actual_owner(owner: Option<&ObjectOwner>) -> Option<String> {
    let owner = owner?;

    match owner {
        ObjectOwner::Immutable(value) => Some(value.clone()),
        ObjectOwner::AddressOwner { address_owner } => Some(address_owner.clone()),
        ObjectOwner::ObjectOwner { object_owner } => Some(object_owner.clone()),
        ObjectOwner::Shared { shared } => Some(format!("Shared-{}", shared.initial_shared_version)),
        ObjectOwner::ConsensusV2 { consensus_v2 } => {
            Some(format!("ConsensusV2-{}", consensus_v2.start_version))
        }
        ObjectOwner::Other(value) => Some(value.to_string()),
    }
}
