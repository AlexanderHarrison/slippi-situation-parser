#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Stage {
    FountainOfDreams     = 002,
    PokemonStadium       = 003,
    PrincessPeachsCastle = 004,
    KongoJungle          = 005,
    Brinstar             = 006,
    Corneria             = 007,
    YoshisStory          = 008,
    Onett                = 009,
    MuteCity             = 010,
    RainbowCruise        = 011,
    JungleJapes          = 012,
    GreatBay             = 013,
    HyruleTemple         = 014,
    BrinstarDepths       = 015,
    YoshisIsland         = 016,
    GreenGreens          = 017,
    Fourside             = 018,
    MushroomKingdomI     = 019,
    MushroomKingdomII    = 020,
    Venom                = 022,
    PokeFloats           = 023,
    BigBlue              = 024,
    IcicleMountain       = 025,
    Icetop               = 026,
    FlatZone             = 027,
    DreamLandN64         = 028,
    YoshisIslandN64      = 029,
    KongoJungleN64       = 030,
    Battlefield          = 031,
    FinalDestination     = 032,
}

impl Stage {
    pub fn from_u16(st: u16) -> Option<Self> {
        Some(match st {
            002 => Stage::FountainOfDreams    ,
            003 => Stage::PokemonStadium      ,
            004 => Stage::PrincessPeachsCastle,
            005 => Stage::KongoJungle         ,
            006 => Stage::Brinstar            ,
            007 => Stage::Corneria            ,
            008 => Stage::YoshisStory         ,
            009 => Stage::Onett               ,
            010 => Stage::MuteCity            ,
            011 => Stage::RainbowCruise       ,
            012 => Stage::JungleJapes         ,
            013 => Stage::GreatBay            ,
            014 => Stage::HyruleTemple        ,
            015 => Stage::BrinstarDepths      ,
            016 => Stage::YoshisIsland        ,
            017 => Stage::GreenGreens         ,
            018 => Stage::Fourside            ,
            019 => Stage::MushroomKingdomI    ,
            020 => Stage::MushroomKingdomII   ,
            022 => Stage::Venom               ,
            023 => Stage::PokeFloats          ,
            024 => Stage::BigBlue             ,
            025 => Stage::IcicleMountain      ,
            026 => Stage::Icetop              ,
            027 => Stage::FlatZone            ,
            028 => Stage::DreamLandN64        ,
            029 => Stage::YoshisIslandN64     ,
            030 => Stage::KongoJungleN64      ,
            031 => Stage::Battlefield         ,
            032 => Stage::FinalDestination    ,
            _ => return None,
        })
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Character {
    Mario          = 00,
    Fox            = 01,
    CaptainFalcon  = 02,
    DonkeyKong     = 03,
    Kirby          = 04,
    Bowser         = 05,
    Link           = 06,
    Sheik          = 07,
    Ness           = 08,
    Peach          = 09,
    Popo           = 10,
    Nana           = 11,
    Pikachu        = 12,
    Samus          = 13,
    Yoshi          = 14,
    Jigglypuff     = 15,
    Mewtwo         = 16,
    Luigi          = 17,
    Marth          = 18,
    Zelda          = 19,
    YoungLink      = 20,
    DrMario        = 21,
    Falco          = 22,
    Pichu          = 23,
    MrGameAndWatch = 24,
    Ganondorf      = 25,
    Roy            = 26,
}

impl Character {
    pub fn from_u8_internal(n: u8) -> Option<Self> {
        Some(match n {
            00 => Character::Mario         ,
            01 => Character::Fox           ,
            02 => Character::CaptainFalcon ,
            03 => Character::DonkeyKong    ,
            04 => Character::Kirby         ,
            05 => Character::Bowser        ,
            06 => Character::Link          ,
            07 => Character::Sheik         ,
            08 => Character::Ness          ,
            09 => Character::Peach         ,
            10 => Character::Popo          ,
            11 => Character::Nana          ,
            12 => Character::Pikachu       ,
            13 => Character::Samus         ,
            14 => Character::Yoshi         ,
            15 => Character::Jigglypuff    ,
            16 => Character::Mewtwo        ,
            17 => Character::Luigi         ,
            18 => Character::Marth         ,
            19 => Character::Zelda         ,
            20 => Character::YoungLink     ,
            21 => Character::DrMario       ,
            22 => Character::Falco         ,
            23 => Character::Pichu         ,
            24 => Character::MrGameAndWatch,
            25 => Character::Ganondorf     ,
            26 => Character::Roy           ,
            _ => return None
        })
    }

    pub fn from_u8_external(n: u8) -> Option<Self> {
        Some(match n {
            00 => Character::CaptainFalcon ,
            01 => Character::DonkeyKong    ,
            02 => Character::Fox           ,
            03 => Character::MrGameAndWatch,
            04 => Character::Kirby         ,
            05 => Character::Bowser        ,
            06 => Character::Link          ,
            07 => Character::Luigi         ,
            08 => Character::Mario         ,
            09 => Character::Marth         ,
            10 => Character::Mewtwo        ,
            11 => Character::Ness          ,
            12 => Character::Peach         ,
            13 => Character::Pikachu       ,
            14 => Character::Popo          ,
            15 => Character::Jigglypuff    ,
            16 => Character::Samus         ,
            17 => Character::Yoshi         ,
            18 => Character::Zelda         ,
            19 => Character::Sheik         ,
            20 => Character::Falco         ,
            21 => Character::YoungLink     ,
            22 => Character::DrMario       ,
            23 => Character::Roy           ,
            24 => Character::Pichu         ,
            25 => Character::Ganondorf     ,
            _ => return None
        })
    }
}

#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
pub enum CharacterColour {
    Mario         (MarioColour),
    Fox           (FoxColour),
    CaptainFalcon (CaptainFalconColour),
    DonkeyKong    (DonkeyKongColour),
    Kirby         (KirbyColour),
    Bowser        (BowserColour),
    Link          (LinkColour),
    Sheik         (ZeldaColour),
    Ness          (NessColour),
    Peach         (PeachColour),
    Popo          (IceClimbersColour),
    Nana          (IceClimbersColour),
    Pikachu       (PikachuColour),
    Samus         (SamusColour),
    Yoshi         (YoshiColour),
    Jigglypuff    (JigglypuffColour),
    Mewtwo        (MewtwoColour),
    Luigi         (LuigiColour),
    Marth         (MarthColour),
    Zelda         (ZeldaColour),
    YoungLink     (YoungLinkColour),
    DrMario       (DrMarioColour),
    Falco         (FalcoColour),
    Pichu         (PichuColour),
    MrGameAndWatch(MrGameAndWatchColour),
    Ganondorf     (GanondorfColour),
    Roy           (RoyColour),
}

impl CharacterColour {
    pub fn from_character_and_colour(character: Character, colour_idx: u8) -> Option<Self> {
        Some(match character {
            Character::Mario          => CharacterColour::Mario          (MarioColour         ::from_u8(colour_idx)?),
            Character::Fox            => CharacterColour::Fox            (FoxColour           ::from_u8(colour_idx)?),
            Character::CaptainFalcon  => CharacterColour::CaptainFalcon  (CaptainFalconColour ::from_u8(colour_idx)?),
            Character::DonkeyKong     => CharacterColour::DonkeyKong     (DonkeyKongColour    ::from_u8(colour_idx)?),
            Character::Kirby          => CharacterColour::Kirby          (KirbyColour         ::from_u8(colour_idx)?),
            Character::Bowser         => CharacterColour::Bowser         (BowserColour        ::from_u8(colour_idx)?),
            Character::Link           => CharacterColour::Link           (LinkColour          ::from_u8(colour_idx)?),
            Character::Sheik          => CharacterColour::Sheik          (ZeldaColour         ::from_u8(colour_idx)?),
            Character::Ness           => CharacterColour::Ness           (NessColour          ::from_u8(colour_idx)?),
            Character::Peach          => CharacterColour::Peach          (PeachColour         ::from_u8(colour_idx)?),
            Character::Popo           => CharacterColour::Popo           (IceClimbersColour   ::from_u8(colour_idx)?),
            Character::Nana           => CharacterColour::Nana           (IceClimbersColour   ::from_u8(colour_idx)?),
            Character::Pikachu        => CharacterColour::Pikachu        (PikachuColour       ::from_u8(colour_idx)?),
            Character::Samus          => CharacterColour::Samus          (SamusColour         ::from_u8(colour_idx)?),
            Character::Yoshi          => CharacterColour::Yoshi          (YoshiColour         ::from_u8(colour_idx)?),
            Character::Jigglypuff     => CharacterColour::Jigglypuff     (JigglypuffColour    ::from_u8(colour_idx)?),
            Character::Mewtwo         => CharacterColour::Mewtwo         (MewtwoColour        ::from_u8(colour_idx)?),
            Character::Luigi          => CharacterColour::Luigi          (LuigiColour         ::from_u8(colour_idx)?),
            Character::Marth          => CharacterColour::Marth          (MarthColour         ::from_u8(colour_idx)?),
            Character::Zelda          => CharacterColour::Zelda          (ZeldaColour         ::from_u8(colour_idx)?),
            Character::YoungLink      => CharacterColour::YoungLink      (YoungLinkColour     ::from_u8(colour_idx)?),
            Character::DrMario        => CharacterColour::DrMario        (DrMarioColour       ::from_u8(colour_idx)?),
            Character::Falco          => CharacterColour::Falco          (FalcoColour         ::from_u8(colour_idx)?),
            Character::Pichu          => CharacterColour::Pichu          (PichuColour         ::from_u8(colour_idx)?),
            Character::MrGameAndWatch => CharacterColour::MrGameAndWatch (MrGameAndWatchColour::from_u8(colour_idx)?),
            Character::Ganondorf      => CharacterColour::Ganondorf      (GanondorfColour     ::from_u8(colour_idx)?),
            Character::Roy            => CharacterColour::Roy            (RoyColour           ::from_u8(colour_idx)?),
        })
    }

    pub fn character(self) -> Character {
        match self {
            CharacterColour::Mario          (..) => Character::Mario         ,
            CharacterColour::Fox            (..) => Character::Fox           ,
            CharacterColour::CaptainFalcon  (..) => Character::CaptainFalcon ,
            CharacterColour::DonkeyKong     (..) => Character::DonkeyKong    ,
            CharacterColour::Kirby          (..) => Character::Kirby         ,
            CharacterColour::Bowser         (..) => Character::Bowser        ,
            CharacterColour::Link           (..) => Character::Link          ,
            CharacterColour::Sheik          (..) => Character::Sheik         ,
            CharacterColour::Ness           (..) => Character::Ness          ,
            CharacterColour::Peach          (..) => Character::Peach         ,
            CharacterColour::Popo           (..) => Character::Popo          ,
            CharacterColour::Nana           (..) => Character::Nana          ,
            CharacterColour::Pikachu        (..) => Character::Pikachu       ,
            CharacterColour::Samus          (..) => Character::Samus         ,
            CharacterColour::Yoshi          (..) => Character::Yoshi         ,
            CharacterColour::Jigglypuff     (..) => Character::Jigglypuff    ,
            CharacterColour::Mewtwo         (..) => Character::Mewtwo        ,
            CharacterColour::Luigi          (..) => Character::Luigi         ,
            CharacterColour::Marth          (..) => Character::Marth         ,
            CharacterColour::Zelda          (..) => Character::Zelda         ,
            CharacterColour::YoungLink      (..) => Character::YoungLink     ,
            CharacterColour::DrMario        (..) => Character::DrMario       ,
            CharacterColour::Falco          (..) => Character::Falco         ,
            CharacterColour::Pichu          (..) => Character::Pichu         ,
            CharacterColour::MrGameAndWatch (..) => Character::MrGameAndWatch,
            CharacterColour::Ganondorf      (..) => Character::Ganondorf     ,
            CharacterColour::Roy            (..) => Character::Roy           ,
        }
    }
}

impl std::fmt::Display for CharacterColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = self.character();
        match self {
            CharacterColour::Mario          (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Fox            (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::CaptainFalcon  (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::DonkeyKong     (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Kirby          (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Bowser         (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Link           (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Sheik          (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Ness           (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Peach          (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Popo           (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Nana           (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Pikachu        (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Samus          (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Yoshi          (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Jigglypuff     (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Mewtwo         (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Luigi          (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Marth          (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Zelda          (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::YoungLink      (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::DrMario        (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Falco          (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Pichu          (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::MrGameAndWatch (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Ganondorf      (colour) => write!(f, "{} ({})", ch, colour),
            CharacterColour::Roy            (colour) => write!(f, "{} ({})", ch, colour),
        }
    }
}

use character_colours::*;
pub mod character_colours {
    macro_rules! colour {
        (pub enum $char:ident { $($colour:ident = $n:expr),* $(,)? }) => {
            #[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
            pub enum $char {
                $($colour = $n,)*
            }

            impl $char {
                pub fn from_u8(n: u8) -> Option<Self> {
                    match n {
                        $($n => Some($char::$colour),)*
                        _ => None,
                    }
                }
            }

            impl std::fmt::Display for $char {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        $($char::$colour => write!(f, "{}", stringify!($colour)),)*
                    }
                }
            }
        }
    }

    colour!(pub enum CaptainFalconColour  { Neutral = 0, Black  = 1, Red      = 2, White     = 3, Green  = 4, Blue  = 5 });
    colour!(pub enum DonkeyKongColour     { Neutral = 0, Black  = 1, Red      = 2, Blue      = 3, Green  = 4            });
    colour!(pub enum FoxColour            { Neutral = 0, Red    = 1, Blue     = 2, Green     = 3                        });
    colour!(pub enum MrGameAndWatchColour { Neutral = 0, Red    = 1, Blue     = 2, Green     = 3                        });
    colour!(pub enum KirbyColour          { Neutral = 0, Yellow = 1, Blue     = 2, Red       = 3, Green  = 4, White = 5 });
    colour!(pub enum BowserColour         { Neutral = 0, Red    = 1, Blue     = 2, Black     = 3                        });
    colour!(pub enum LinkColour           { Neutral = 0, Red    = 1, Blue     = 2, Black     = 3, White  = 4            });
    colour!(pub enum LuigiColour          { Neutral = 0, White  = 1, Blue     = 2, Red       = 3                        });
    colour!(pub enum MarioColour          { Neutral = 0, Yellow = 1, Black    = 2, Blue      = 3, Green  = 4            });
    colour!(pub enum MarthColour          { Neutral = 0, Red    = 1, Green    = 2, Black     = 3, White  = 4            });
    colour!(pub enum MewtwoColour         { Neutral = 0, Red    = 1, Blue     = 2, Green     = 3                        });
    colour!(pub enum NessColour           { Neutral = 0, Yellow = 1, Blue     = 2, Green     = 3                        });
    colour!(pub enum PeachColour          { Neutral = 0, Daisy  = 1, White    = 2, Blue      = 3, Green  = 4            });
    colour!(pub enum PikachuColour        { Neutral = 0, Red    = 1, PartyHat = 2, CowboyHat = 3                        });
    colour!(pub enum IceClimbersColour    { Neutral = 0, Green  = 1, Orange   = 2, Red       = 3                        });
    colour!(pub enum JigglypuffColour     { Neutral = 0, Red    = 1, Blue     = 2, Headband  = 3, Crown  = 4            });
    colour!(pub enum SamusColour          { Neutral = 0, Pink   = 1, Black    = 2, Green     = 3, Purple = 4            });
    colour!(pub enum YoshiColour          { Neutral = 0, Red    = 1, Blue     = 2, Yellow    = 3, Pink   = 4, Cyan  = 5 });
    colour!(pub enum ZeldaColour          { Neutral = 0, Red    = 1, Blue     = 2, Green     = 3, White  = 4            });
    colour!(pub enum FalcoColour          { Neutral = 0, Red    = 1, Blue     = 2, Green     = 3                        });
    colour!(pub enum YoungLinkColour      { Neutral = 0, Red    = 1, Blue     = 2, White     = 3, Black  = 4            });
    colour!(pub enum DrMarioColour        { Neutral = 0, Red    = 1, Blue     = 2, Green     = 3, Black  = 4            });
    colour!(pub enum RoyColour            { Neutral = 0, Red    = 1, Blue     = 2, Green     = 3, Yellow = 4            });
    colour!(pub enum PichuColour          { Neutral = 0, Red    = 1, Blue     = 2, Green     = 3                        });
    colour!(pub enum GanondorfColour      { Neutral = 0, Red    = 1, Blue     = 2, Green     = 3, Purple = 4            });
}
