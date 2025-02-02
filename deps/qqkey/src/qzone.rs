use crate::{Error, Result as QQResult, QQ_REFERER};
use rand::Rng;
use regex::Regex;
use reqwest::{header::REFERER, Client};
use reqwest_cookie_store::CookieStoreRwLock;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

type GroupList = HashMap<i64, Group>;

pub struct Group {
    pub name: String,
    pub friends: Vec<Friend>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecialCareResponse {
    pub code: i64,
    pub subcode: i64,
    pub message: String,
    pub default: i64,
    pub data: SpecialCareData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecialCareData {
    pub items_special: Vec<ItemsSpecial>,
    pub items_recommend: Vec<ItemsRecommend>,
    pub group_recommend: Vec<Value>,
    pub group_special: Vec<Value>,
    pub push_flag: i64,
    pub used_count: Vec<UsedCount>,
    pub fans_count: Vec<FansCount>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemsSpecial {
    pub uin: i64,
    pub name: String,
    pub group_flag: String,
    pub score: i64,
    pub img: String,
    pub tipon: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemsRecommend {
    pub uin: i64,
    pub name: String,
    pub group_flag: String,
    pub img: String,
    pub tipon: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsedCount {
    pub used_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FansCount {
    pub fans_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FriendsResponse {
    pub code: i64,
    pub subcode: i64,
    pub message: String,
    pub default: i64,
    pub data: FriendsData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FriendsData {
    pub items: Vec<Friend>,
    #[serde(rename = "gpnames")]
    pub group_names: Vec<GroupName>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Friend {
    pub uin: i64,
    pub groupid: i64,
    pub name: String,
    pub remark: String,
    pub img: String,
    pub yellow: i64,
    pub online: i64,
    pub v6: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupName {
    #[serde(rename = "gpid")]
    pub group_id: i64,
    #[serde(rename = "gpname")]
    pub group_name: String,
}

pub struct QZone {
    client: Client,
    cookie_store: Arc<CookieStoreRwLock>,
    uin: i64,
    skey: String,
    p_skey: String,
    g_tk: String,
}

impl QZone {
    pub(crate) fn new(
        client: Client,
        cookie_store: Arc<CookieStoreRwLock>,
        uin: i64,
        skey: String,
        p_skey: String,
        g_tk: String,
    ) -> Self {
        Self {
            client,
            cookie_store,
            uin,
            skey,
            p_skey,
            g_tk,
        }
    }

    pub fn get_friends_url(&self) -> String {
        format!("https://user.qzone.qq.com/proxy/domain/r.qzone.qq.com/cgi-bin/tfriend/friend_show_qqfriends.cgi?uin={}&follow_flag=1&groupface_flag=0&fupdate=1&g_tk={}",self.uin,self.g_tk)
    }

    pub fn get_special_cares_url(&self) -> String {
        format!("https://user.qzone.qq.com/proxy/domain/r.qzone.qq.com/cgi-bin/tfriend/specialcare_get.cgi?uin={}&do=3&fupdate=1&rd={}&g_tk={}",self.uin,rand::thread_rng().gen_range(0.0..1.0),self.g_tk)
    }

    async fn get_friends_internal(&self, url: String) -> QQResult<String> {
        let response = self
            .client
            .get(url)
            .header(REFERER, QQ_REFERER)
            .send()
            .await?
            .text()
            .await?
            .replace('\n', "");
        Ok(Regex::new(r"_Callback\((?<json>.*)\)")?
            .captures(&response)
            .ok_or(Error::RegexNoMatch("get friends".into()))?["json"]
            .to_string())
    }

    pub async fn get_friends(&self) -> QQResult<GroupList> {
        let response = serde_json::from_str::<FriendsResponse>(
            &self.get_friends_internal(self.get_friends_url()).await?,
        )?;
        let mut ret = GroupList::new();

        if response.code != 0 {
            return Err(Error::QQError);
        }

        for group in response.data.group_names {
            ret.insert(
                group.group_id,
                Group {
                    name: group.group_name,
                    friends: Vec::new(),
                },
            );
        }

        for friend in response.data.items {
            ret.get_mut(&friend.groupid)
                .ok_or(Error::FieldNotFound("groups".into()))?
                .friends
                .push(friend);
        }

        Ok(ret)
    }

    pub async fn get_special_cares(&self) -> QQResult<Vec<ItemsSpecial>> {
        let ret = serde_json::from_str::<SpecialCareResponse>(
            &self
                .get_friends_internal(self.get_special_cares_url())
                .await?,
        )?;

        if ret.code != 0 {
            return Err(Error::QQError);
        }

        Ok(ret.data.items_special)
    }
}
