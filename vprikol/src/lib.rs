use reqwest::Response;

const HOST: &'static str = "https://api.vprikol.dev/";

pub struct Prikol {
    token: String,
    client: reqwest::Client,
}

impl Prikol {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: reqwest::Client::new(),
        }
    }

    pub async fn members(&self, server_id: i32, fraction_id: i32) -> anyhow::Result<Vec<PrikolMember>> {
        let res = self.get(String::from(format!("members?server={}&fraction_id={}", server_id, fraction_id)).as_str())
            .await?
            .json::<serde_json::Value>()
            .await?;

        let mut members = Vec::new();
        
        let players = res["players"].as_object();
        
        for (player_name, player) in players.unwrap() {
            let rank = player["rank"].as_u64().unwrap() as u8;
            let is_online = player["isOnline"].as_bool().unwrap();
            let is_leader = player["isLeader"].as_bool().unwrap();
            let rank_label = player["rankLabel"].as_str().unwrap().to_string();
            let id = player["id"].as_u64().unwrap_or(0);

            members.push(PrikolMember {
                id,
                name: player_name.to_string(),
                rank,
                is_online,
                is_leader,
                rank_label,
            });
        }
        
        Ok(members)
    }
    
    async fn get(&self, url: &str) -> anyhow::Result<Response> {
        let req = self.client.get(HOST.to_owned() + url).header("Authorization", "Bearer ".to_string() + self.token.as_str()).build()?;
        let res = reqwest::Client::new().execute(req).await?;
        Ok(res)
    }
}

#[derive(Debug)]
pub struct PrikolMember {
    pub name: String,
    pub rank: u8,
    pub is_online: bool,
    pub is_leader: bool,
    pub rank_label: String,
    pub id: u64,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
