use super::parser::{dictify, key_for_u64, key_for_u32, key_for_u8, key_for_u16};

use super::gdenums::{IconType, CommentHistoryState, Authority};


/** See: https://wyliemaster.github.io/gddocs/#/resources/server/user */
/** NOTE: Some integers are 64 bit because I belive we will hit that may users in our life-time */






#[derive(Builder)]
pub struct User{
    pub name:String,
    pub player_id:u64,
    pub stars:u64,
    pub demons:u64,
    pub ranking:u64,
    pub creatorpoints:u32,
    pub icon_id:u32,
    pub color:u32,
    pub color2:u32,
    pub secret_coins:u32,
    pub icon_type:u8,
    pub special:u32,
    pub account_id:u64,
    pub usercoins:u32,
    pub message_state:u8,
    pub friends_state:u8,
    pub youtube:String,
    pub acc_icon:u16,
    pub acc_ship:u16,
    pub acc_ball:u16,
    pub acc_bird:u16,
    pub acc_dart:u16,
    pub acc_robot:u16,
    pub acc_streak:u16,
    pub acc_glow:u16,
    pub is_registered:u32,
    pub global_rank:u64,
    pub friendstate:u8,
    pub age: String,
    pub acc_spider: u16,
    pub twitter:String,
    pub twitch:String,
    pub diamonds:u64,
    pub acc_explosion:u16,
    pub authority:u8,
    pub comment_history_state:u8
}




impl User {
    pub (crate) fn new(data:String) -> Self {
        /* Rather Poopy that I couldn't make the hashMap be somewhere else but what can I say? it's my first rust program */
        let s = dictify(&data, ":");
 
        UserBuilder::create_empty()
            .name(s.get("1").unwrap().to_string())
            .player_id(key_for_u64(&s, "2"))
            .stars(key_for_u64(&s, "3"))
            .demons(key_for_u64(&s, "4"))
            .ranking(key_for_u64(&s, "6"))
            .creatorpoints(key_for_u32(&s, "8"))
            .icon_id(key_for_u32(&s, "9"))
            .color(key_for_u32(&s, "10"))
            .color2(key_for_u32( &s, "11"))
            .secret_coins(key_for_u32(&s, "13"))
            .icon_type(key_for_u8(&s, "14"))
            .special(key_for_u32(&s, "15"))
            .account_id(key_for_u64(&s, "16"))
            .usercoins(key_for_u32(&s, "17"))
            .message_state(key_for_u8(&s, "18"))
            .friendstate(key_for_u8(&s, "19"))
            .youtube(s.get("20").unwrap_or(&"NONE").to_string())
            .acc_icon(key_for_u16(&s, "21"))
            .acc_ship(key_for_u16(&s, "22"))
            .acc_ball(key_for_u16(&s, "23"))
            .acc_bird(key_for_u16(&s, "24"))
            .acc_dart(key_for_u16(&s, "25"))
            .acc_robot(key_for_u16(&s, "26"))
            .acc_streak(key_for_u16(&s, "27"))
            .acc_glow(key_for_u16(&s, "28"))
            .is_registered(key_for_u32(&s, "29"))
            .global_rank(key_for_u64(&s, "30"))
            .friends_state(key_for_u8(&s, "31"))
            .age(s.get("42").unwrap_or(&"UNKNOWN").to_string())
            .acc_spider(key_for_u16(&s, "43"))
            .twitter(s.get("44").unwrap_or(&"UNKNOWN").to_string())
            .twitch(s.get("45").unwrap_or(&"UNKNOWN").to_string())
            .diamonds(key_for_u64( &s, "46"))
            .acc_explosion(key_for_u16(&s, "48"))
            .authority(key_for_u8(&s, "49"))
            .comment_history_state(key_for_u8(&s, "50"))
            .build().unwrap()
    }

    /// Determines What the level of authority the user has been given over Geometry dash
    pub (crate) fn authority_status(&self) -> Authority {
        Authority::try_from(self.authority).unwrap()
    }

    /// Determines what settings/rules the user has on looking at their comment history
    pub (crate) fn comment_history_status(&self) -> CommentHistoryState {
        CommentHistoryState::try_from(self.comment_history_state).unwrap()
    }

    /// Obtains the Enum For the User's IconType
    pub (crate) fn icon(&self) -> IconType {
        IconType::try_from(self.icon_type).unwrap()
    }


    pub (crate) fn report_info(&self) {
        println!("=================== USER INFO =========================================");
        println!("[Username]: {}", self.name);
        println!("[AccountID]: {}", self.account_id);
        println!("[PlayerID]: {}", self.player_id);
        println!("[Stars]: {}", self.stars);
        println!("[Demons]: {}", self.demons);
        println!("[CurrentIcon]: {}", self.icon());
        if self.authority > 0 {
            println!("[Moderation Level]: {}", self.authority_status());
        }
        println!("[Comment History Status]: {}", self.comment_history_status());
    }

}

