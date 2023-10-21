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
pub struct Data {
    pub amount_raised: Option<Amount>,
    pub avatar: Option<Avatar>,
    pub currency_code: String,
    pub description: Option<String>,
    pub donate_url: String,
    pub goal: Option<Goal>,
    pub has_schedule: bool,
    pub id: String,
    pub inserted_at: String,
    pub legacy_id: i32,
    pub livestream: Option<Livestream>,
    pub name: String,
    pub original_goal: Option<Goal>,
    pub published_at: String,
    pub retired_at: Option<String>,
    pub slug: String,
    pub status: String,
    pub supportable: String,
    pub supporting_amount_raised: Option<Amount>,
    pub team: Team,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Campaign {
    pub data: Data,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let json_data = r#"
        {
            "data": {
                "amount_raised": {
                    "currency": "USD",
                    "value": "182.32"
                },
                "avatar": {
                    "alt": "Short image description used as alternative text.",
                    "height": 200,
                    "src": "https://tiltify.com/images/example.jpg",
                    "width": 200
                },
                "currency_code": "USD",
                "description": "Let's fundraise together",
                "donate_url": "https://donate.tiltify.com/+team-slug/our-awesome-team-campaign",
                "goal": {
                    "currency": "USD",
                    "value": "182.32"
                },
                "has_schedule": false,
                "id": "e0da6810-7eeb-4585-9a96-261e73b300c8",
                "inserted_at": "2023-09-19T17:16:53.698545Z",
                "legacy_id": 907663706,
                "livestream": {
                    "channel": "tiltify",
                    "type": "twitch"
                },
                "name": "Our Awesome Team Campaign",
                "original_goal": {
                    "currency": "USD",
                    "value": "182.32"
                },
                "published_at": "2023-09-19T17:16:53.698579Z",
                "retired_at": null,
                "slug": "our-awesome-team-campaign",
                "status": "published",
                "supportable": "none",
                "supporting_amount_raised": {
                    "currency": "USD",
                    "value": "182.32"
                },
                "team": {
                    "avatar": {
                        "alt": "Short image description used as alternative text.",
                        "height": 200,
                        "src": "https://tiltify.com/images/example.jpg",
                        "width": 200
                    },
                    "description": "Awesome group of friends fundraising for charity together.",
                    "id": "2682116f-8f79-4d43-980c-b285ec07c615",
                    "legacy_id": 77805171,
                    "name": "Awesome Team",
                    "slug": "awesome-team",
                    "social": {
                        "discord": "https://discord.gg/tiltify",
                        "facebook": "tiltify",
                        "instagram": "tiltify",
                        "snapchat": "tiltify",
                        "tiktok": "tilitfy",
                        "twitch": "tilitfy",
                        "twitter": "tiltify",
                        "website": "https://tiltify.com",
                        "youtube": "UCWcPgWbuWuJX5rHWm6Kb4Vw"
                    },
                    "total_amount_raised": {
                        "currency": "USD",
                        "value": "182.32"
                    },
                    "url": "https://tiltify.com/+awesome-team"
                },
                "team_id": "de239ed5-a4ff-4766-bbcf-d6d1f678cd89",
                "total_amount_raised": {
                    "currency": "USD",
                    "value": "182.32"
                },
                "updated_at": "2023-09-19T17:16:53.698590Z",
                "url": "https://tiltify.com/+team-slug/our-awesome-team-campaign"
            }
        }
        "#;

        let campaign: Campaign = serde_json::from_str(json_data).unwrap();
        println!("{:?}", campaign);

        let serialized = serde_json::to_string(&campaign).unwrap();
        println!("{}", serialized);

        assert_eq!(4, 4);
    }
}
