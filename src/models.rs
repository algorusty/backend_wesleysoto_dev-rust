use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavItem {
    pub icon: String,
    pub text: String,
}

// You might want to add more models here in the future
