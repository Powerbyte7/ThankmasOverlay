use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Amount {
    pub currency: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Avatar {
    pub alt: Option<String>,
    pub height: Option<i32>,
    pub src: String,
    pub width: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Goal {
    pub currency: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Livestream {
    pub channel: Option<String>,
    #[serde(rename = "type")]
    pub livestream_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Social {
    pub discord: Option<String>,
    pub facebook: Option<String>,
    pub instagram: Option<String>,
    pub snapchat: Option<String>,
    pub tiktok: Option<String>,
    pub twitch: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub youtube: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Campaign {
    pub amount_raised: Amount,
    pub avatar: Option<Avatar>,
    pub currency_code: Option<String>,
    pub description: Option<String>,
    pub donate_url: String,
    pub goal: Option<Goal>,
    pub has_schedule: Option<bool>,
    pub id: String,
    pub inserted_at: Option<String>,
    pub legacy_id: i32,
    pub livestream: Option<Livestream>,
    pub name: String,
    pub original_goal: Option<Goal>,
    pub published_at: Option<String>,
    pub retired_at: Option<String>,
    pub slug: String,
    pub status: Option<String>,
    pub supportable: Option<String>,
    pub supporting_amount_raised: Option<Amount>,
    pub team: Option<Team>,
    pub team_id: Option<String>,
    pub total_amount_raised: Option<Amount>,
    pub updated_at: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Team {
    pub avatar: Option<Avatar>,
    pub description: Option<String>,
    pub id: String,
    pub legacy_id: i32,
    pub name: String,
    pub slug: String,
    pub social: Option<Social>,
    pub total_amount_raised: Option<Amount>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Donation {
    pub id: String,
    pub amount: Amount,
    pub donor_name: String,
    pub donor_comment: Option<String>,
}

// Used for correctly parsing JSON
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TiltifyReponse<T> {
    pub data: T
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn campaign_serialization() {
        let json_data_a = include_str!("../testdata/campaign1.json");
        let json_data_b = include_str!("../testdata/campaign2.json");

        let campaign_a: TiltifyReponse<Campaign> = serde_json::from_str(json_data_a).unwrap();
        let campaign_b: TiltifyReponse<Campaign> = serde_json::from_str(json_data_b).unwrap();

        serde_json::to_string(&campaign_a).unwrap();
        serde_json::to_string(&campaign_b).unwrap();
    }

    #[test]
    fn donation_serialization() {
        let json_data = include_str!("../testdata/donation.json");

        let donation: TiltifyReponse<Donation> = serde_json::from_str(json_data).unwrap();

        serde_json::to_string(&donation).unwrap();
    }
}
