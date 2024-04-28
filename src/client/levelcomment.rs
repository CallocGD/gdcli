use super::gdenums::Authority;
use std::collections::HashMap;
use itertools::Itertools;


/* Some Data Was Gerated using a new DSL that I programmed to help me optimize 
 * the data since there's so many tiny moving parts that make this script work 
 * 
 * This was done by ripping the html of the documentation https://wyliemaster.github.io/gddocs/#/resources/server/comment 
 * and then converting the tables to yaml so they could be easily edited and then the DSL converts 
 * that given data over to rust 
 * */


#[derive(Builder)]
pub struct LevelCommentUser {
    pub username:String,
    pub icon:u64,
    pub playercolor:u64,
    pub playercolor2:u64,
    pub icontype:u64,
    pub glow:u64,
    pub accountid:u64,
}

impl LevelCommentUser {
    pub (crate) fn new(resp_str:String) -> LevelCommentUser {
        let _s:HashMap<&str, &str> = resp_str.as_str().split("~").tuples().collect();
        LevelCommentUserBuilder::create_empty()
            .username(_s.get("1").unwrap_or(&"UNKNOWN").to_string())
            .icon(_s.get("9").unwrap_or(&"0").parse::<u64>().unwrap_or(0))
            .playercolor(_s.get("10").unwrap_or(&"0").parse::<u64>().unwrap_or(0))
            .playercolor2(_s.get("11").unwrap_or(&"0").parse::<u64>().unwrap_or(0))
            .icontype(_s.get("14").unwrap_or(&"0").parse::<u64>().unwrap_or(0))
            .glow(_s.get("15").unwrap_or(&"0").parse::<u64>().unwrap_or(0))
            .accountid(_s.get("16").unwrap_or(&"0").parse::<u64>().unwrap_or(0))
            .build().unwrap()
    }
}

#[derive(Builder)]
pub struct LevelCommmentData {
    pub levelid:u64,
    pub comment:String,
    pub authorplayerid:u64,
    pub likes:i64,
    pub dislikes:i64,
    pub messageid:u64,
    pub spam:bool,
    pub authoraccountid:u64,
    pub age:String,
    pub percent:u8,
    pub modbadge:u8,
    pub moderatorchatcolor:String,
}

impl LevelCommmentData {
    pub (crate) fn new(resp_str:String) -> LevelCommmentData {
        let _s:HashMap<&str, &str> = resp_str.as_str().split("~").tuples().collect();
        LevelCommmentDataBuilder::create_empty()
            .levelid(_s.get("1").unwrap_or(&"0").parse::<u64>().unwrap_or(0))
            .comment(_s.get("2").unwrap_or(&"UNKNOWN").to_string())
            .authorplayerid(_s.get("3").unwrap_or(&"0").parse::<u64>().unwrap_or(0))
            .likes(_s.get("4").unwrap_or(&"0").parse::<i64>().unwrap_or(0))
            .dislikes(_s.get("5").unwrap_or(&"0").parse::<i64>().unwrap_or(0))
            .messageid(_s.get("6").unwrap_or(&"0").parse::<u64>().unwrap_or(0))
            .spam(_s.get("7").unwrap_or(&"0").starts_with("1"))
            .authoraccountid(_s.get("8").unwrap_or(&"0").parse::<u64>().unwrap_or(0))
            .age(_s.get("9").unwrap_or(&"UNKNOWN").to_string())
            .percent(_s.get("10").unwrap_or(&"0").parse::<u8>().unwrap_or(0))
            .modbadge(_s.get("11").unwrap_or(&"0").parse::<u8>().unwrap_or(0))
            .moderatorchatcolor(_s.get("12").unwrap_or(&"UNKNOWN").to_string())
            .build().unwrap()
    }
}

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
pub struct LevelComment{
    pub data:LevelCommmentData,
    pub user:LevelCommentUser,
}

impl LevelComment{
    pub (crate) fn new(resp_str:String) -> Self {
        let pair: Vec<&str> = resp_str.split(":").collect();
        Self { 
            data: LevelCommmentData::new(pair[0].to_string()), 
            user: LevelCommentUser::new(pair[1].to_string())
        }
    }

    /// Obtains the mod status of a geometry dash user 
    pub (crate) fn get_authority_level(&self) -> Authority {
        Authority::try_from(self.data.modbadge).unwrap()
    }

    // Sets the levelid for the geometry dash level comment from the requested levelID
    // pub (crate) fn set_levelid(&mut self, _levelid:u64) {
    //     self.data.levelid = _levelid;
    // }

    /// Decodes A Geometry dash comment from base64
    pub (crate) fn comment(&self) -> String{
        String::from_utf8(URL_SAFE.decode(&self.data.comment).unwrap()).unwrap().to_string()
    }    


}



