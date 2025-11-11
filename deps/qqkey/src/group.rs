use crate::{Error, Result as QQResult, QQ};
use chrono::Utc;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Detail {
    pub admin_num: i64,
    pub levelname: Option<HashMap<String, String>>,
    pub members: Vec<Member>,
    pub count: i64,
    pub max_count: i64,
}

impl Detail {
    pub fn new(
        admin_num: i64,
        count: i64,
        max_count: i64,
        levelname: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            admin_num,
            levelname,
            members: Vec::new(),
            count,
            max_count,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Role {
    Owner,
    Admin,
    Member,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetailResponse {
    pub ec: i64,
    pub errcode: i64,
    pub em: String,
    pub cache: Option<i64>,
    pub adm_num: Option<i64>,
    pub levelname: Option<HashMap<String, String>>,
    pub mems: Option<Vec<Member>>,
    pub count: Option<i64>,
    pub svr_time: Option<i64>,
    pub max_count: Option<i64>,
    pub search_count: Option<i64>,
    pub extmode: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Member {
    pub uin: i64,
    pub role: i64,
    pub g: i64,
    pub join_time: i64,
    pub last_speak_time: i64,
    pub lv: Level,
    pub card: String,
    pub tags: String,
    pub flag: i64,
    pub nick: String,
    pub qage: i64,
    pub rm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Level {
    pub point: i64,
    pub level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListResponse {
    pub ec: i64,
    pub errcode: i64,
    pub em: String,
    pub join: Option<Vec<Info>>,
    pub manage: Option<Vec<Info>>,
    pub create: Option<Vec<Info>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub gc: i64,
    pub gn: String,
    pub owner: i64,
}

pub struct Group {
    qq: Arc<QQ>,
    info: Info,
    bkn: String,
    role: Role,
}

impl Group {
    pub fn new(qq: Arc<QQ>, info: Info, bkn: String, role: Role) -> Self {
        Self {
            qq,
            info,
            bkn,
            role,
        }
    }

    pub fn id(&self) -> i64 {
        self.info.gc
    }

    pub fn name(&self) -> &str {
        &self.info.gn
    }

    pub fn is_admin(&self) -> bool {
        self.role != Role::Member
    }

    pub fn role(&self) -> Role {
        self.role
    }

    fn qun_search_url(&self) -> String {
        let timestamp = Utc::now().timestamp();

        format!(
            "https://qun.qq.com/cgi-bin/qun_mgr/search_group_members?bkn={}&ts={}",
            self.bkn, timestamp
        )
    }

    async fn detail_internal(&self, start: i64, end: i64) -> QQResult<DetailResponse> {
        let url = self.qun_search_url();
        let response = self
            .qq
            .qq_request(Method::POST, url)
            .form(&[
                ("bkn", self.bkn.clone()),
                ("gc", self.info.gc.to_string()),
                ("st", start.to_string()),
                ("end", end.to_string()),
                ("sort", "0".to_string()),
            ])
            .send()
            .await?
            .json::<DetailResponse>()
            .await?;

        if response.ec != 0 {
            return Err(Error::RequestQQError);
        }

        Ok(response)
    }

    pub async fn detail(&self) -> QQResult<Detail> {
        let page_size = 20;
        let mut start = 0;
        let response = self.detail_internal(0, 0).await?;
        let mut count = response
            .count
            .ok_or(Error::FieldNotFound("group member count".into()))?;
        let max_count = response
            .max_count
            .ok_or(Error::FieldNotFound("group member max count".into()))?;
        let admin_num = response
            .adm_num
            .ok_or(Error::FieldNotFound("group admin count".into()))?;
        let levelname = response.levelname;
        let mut ret = Detail::new(admin_num, count, max_count, levelname);

        while count > 0 {
            let response = self.detail_internal(start, start + page_size).await?;

            ret.members.append(
                response
                    .mems
                    .ok_or(Error::FieldNotFound("group members".into()))?
                    .as_mut(),
            );

            count -= page_size + 1;
            start += page_size + 1;
        }

        Ok(ret)
    }
}
