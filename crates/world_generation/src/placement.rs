use serde::{Deserialize, Serialize};

use crate::{ComponentSpec, Layout};

/// One authored placement: mesh prototype + layout + optional components.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Placement {
    pub id: String,
    pub mesh: String,
    pub layout: Layout,
    #[serde(default)]
    pub components: Vec<ComponentSpec>,
}
