use super::parser::{dictify, key_for_u64, key_for_bool, key_for_i64, key_for_u8, key_for_u16, key_for_str};
use super::gdenums::LevelLength;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};

/* 26364 */

#[derive(Builder)]
pub struct Level {
    pub id:u64,
    pub name:String,
    pub raw_description:String,
    pub version:u64,
    pub player_id:u64,
    pub difficulty_denominator:u8,
    pub difficulty_numerator:u8,
    pub downloads:u64,
    pub official_song:u8,
    pub game_version:u16,
    pub likes:i64,
    pub length:u8,
    pub dislikes:i64,
    pub demon:bool,
    pub stars:u8,
    pub feature_score:u64,
    pub auto:bool,
    pub password:String,
    pub upload_date:String,
    pub update_date:String,
    pub copied_id:u64,
    pub two_player:bool,
    pub custom_song_id:u64,
    pub extra_string:String,
    pub coins:u8,
    pub verified_coins:bool,
    pub stars_requested:u8,
    pub low_detail_mode:bool,
    pub daily_number:u64,
    pub epic:u8,
    pub demon_difficulty:u8,
    pub is_gauntlet:bool,
    pub objects:u16,
    pub editor_time:u64,
    pub editor_time_copies:u64
}


impl Level {
    pub (crate) fn new(resp_str:String) -> Level {
        let _s = dictify(resp_str.as_str(), ":");
        LevelBuilder::create_empty()
            .id(key_for_u64(&_s, "1"))
            .name(_s.get("2").unwrap().to_string())
            .raw_description(key_for_str(&_s, "3"))
            .version(key_for_u64(&_s, "5"))
            .player_id(key_for_u64(&_s, "6"))
            .difficulty_denominator(key_for_u8(&_s, "8"))
            .difficulty_numerator(key_for_u8(&_s, "9"))
            .downloads(key_for_u64(&_s, "10"))
            .official_song(key_for_u8(&_s, "12"))
            .game_version(key_for_u16(&_s, "13"))
            .likes(key_for_i64(&_s, "14"))
            .length(key_for_u8(&_s, "15"))
            .dislikes(key_for_i64(&_s,"16"))
            .demon(key_for_bool(&_s, "17"))
            .stars(key_for_u8(&_s, "18"))
            .feature_score(key_for_u64(&_s, "19"))
            .auto(key_for_bool(&_s, "25"))
            .password(key_for_str(&_s, "27"))
            .update_date(key_for_str(&_s, "28"))
            .upload_date(key_for_str(&_s,"29"))
            .copied_id(key_for_u64(&_s, "30"))
            .two_player(key_for_bool(&_s, "31"))
            .custom_song_id(key_for_u64(&_s,  "35" ))
            .extra_string(key_for_str(&_s, "36"))
            .coins(key_for_u8(&_s, "37"))
            .verified_coins(key_for_bool(&_s, "38"))
            .stars_requested(key_for_u8(&_s, "39"))
            .low_detail_mode(key_for_bool(&_s, "40"))
            .daily_number(key_for_u64(&_s, "41"))
            .epic(key_for_u8(&_s, "42"))
            .demon_difficulty(key_for_u8(&_s, "43"))
            .is_gauntlet(key_for_bool(&_s, "44"))
            .objects(key_for_u16(&_s, "45"))
            .editor_time(key_for_u64(&_s, "46"))
            .editor_time_copies(key_for_u64(&_s, "47"))
            .build().unwrap()
    }

    pub (crate) fn length_type(&self) -> LevelLength{
        LevelLength::try_from(self.length).unwrap()
    }

    pub (crate) fn description(&self) -> String {
        String::from_utf8(URL_SAFE.decode(&self.raw_description).unwrap()).unwrap().to_string()
    }

    pub (crate) fn report_info(&self) {
        println!("=================== LEVEL INFO =========================================");
        println!("[Level Name]: {}", self.name);
        println!("[Author playerID]: {}", self.player_id);

        if self.daily_number > 0 {
            println!("[Daily Number]: {}", self.daily_number);
        }

        if self.stars > 0 {
            println!("[Stars]: {}", self.stars);
        } else {
            println!("[Stars Requested]: {}", self.stars_requested);
        }
        println!("[Length]: {}", self.length_type());
        println!("[Downloads]: {}", self.downloads);
        println!("[Demon]: {}", self.demon);
        
        if self.likes != 0 {
            println!("[Likes]: {}", self.likes);
        }
        
        if self.dislikes != 0 {
            println!("[Dislikes]: {}", self.dislikes);
        }

        if self.coins > 0 {
            println!("[Coins]: {}", self.coins);
        }

        println!("[Gauntlet]: {}", self.is_gauntlet);

        /* TODO: Official Song Enum? */
        if self.official_song == 0 {
            println!("[SongID]: {}", self.custom_song_id)
        } else {
            println!("[Official Song ID]: {}", self.official_song)
        }
        println!("[Has LDM]: {}", self.low_detail_mode);
        println!("[Object Count]: {}", self.objects);
        println!("[Description]: {}", self.description());

    }

}

