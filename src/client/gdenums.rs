use std::convert::TryFrom;
use std::fmt;

/* NOTE: Unknown is primarly used as a placeholder for "I don't know" */
#[derive(Debug)]
pub enum IconType {
    Cube = 0,
    Ship = 1,
    Ball = 2,
    Ufo = 3,
    Wave = 4,
    Robot = 5,
    Spider = 6,
    Swing = 7,
    Jetpack = 8
}


impl TryFrom<u8> for IconType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == IconType::Cube as u8 => Ok(IconType::Cube),
            x if x == IconType::Ship as u8 => Ok(IconType::Ship),
            x if x == IconType::Ball as u8 => Ok(IconType::Ball),
            x if x == IconType::Ufo as u8 => Ok(IconType::Ufo),
            x if x == IconType::Wave as u8 => Ok(IconType::Wave),
            x if x == IconType::Robot as u8 => Ok(IconType::Robot),
            x if x == IconType::Spider as u8 => Ok(IconType::Spider),
            x if x == IconType::Swing as u8 => Ok(IconType::Swing),
            x if x == IconType::Jetpack as u8 => Ok(IconType::Jetpack),
            _ => Err(()),
        }
    }
}

impl fmt::Display for IconType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/** I call Moderators Authority simply as an ongoing programming meme, 
 * The name fits and has continued to be here */
#[derive(Debug)]
pub enum Authority {
    User = 0,
    Mod = 1,
    Eldermod = 2
}

impl TryFrom<u8> for Authority {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error>{
        match v {
            x if x == Authority::User as u8 => Ok(Authority::User),
            x if x == Authority::Mod as u8 => Ok(Authority::Mod),
            x if x == Authority::Eldermod as u8  => Ok(Authority::Eldermod),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Authority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug)]
pub enum CommentHistoryState {
    All = 0, 
    OnlyFriends = 1, 
    Nobody = 2
}

impl TryFrom <u8> for CommentHistoryState {
    type Error = ();
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == CommentHistoryState::All as u8 => Ok(CommentHistoryState::All),
            x if x == CommentHistoryState::OnlyFriends as u8 => Ok(CommentHistoryState::OnlyFriends),
            x if x == CommentHistoryState::Nobody as u8 => Ok(CommentHistoryState::Nobody),
            _ => Err(())
        }
    }
}

impl fmt::Display for CommentHistoryState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum LevelLength {
    Tiny = 0,
    Small = 1,
    Medium = 2,
    Large = 3,
    XL = 4,
    Platformer = 5
}

impl TryFrom <u8> for LevelLength {
    type Error = ();
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == LevelLength::Tiny as u8 => Ok(LevelLength::Tiny),
            x if x == LevelLength::Small as u8 => Ok(LevelLength::Small),
            x if x == LevelLength::Medium as u8 => Ok(LevelLength::Medium),
            x if x == LevelLength::Large as u8 => Ok(LevelLength::Large),
            x if x == LevelLength::XL as u8 => Ok(LevelLength::XL),
            x if x == LevelLength::Platformer as u8 => Ok(LevelLength::Platformer),
            _ => Err(())
        }
    }
}

impl fmt::Display for LevelLength {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}



