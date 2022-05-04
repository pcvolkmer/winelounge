use crate::world::{BoxAreaContent, BoxAreaPosition, Command, Direction};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Up" => Ok(Direction::Up),
            "Down" => Ok(Direction::Down),
            "Left" => Ok(Direction::Left),
            "Right" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl Display for BoxAreaPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxAreaPosition::RightTop => write!(f, "RightTop"),
            BoxAreaPosition::RightBottom => write!(f, "RightBottom"),
            BoxAreaPosition::LeftBottom => write!(f, "LeftBottom"),
            BoxAreaPosition::LeftTop => write!(f, "LeftTop"),
        }
    }
}

impl FromStr for BoxAreaPosition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RightTop" => Ok(BoxAreaPosition::RightTop),
            "RightBottom" => Ok(BoxAreaPosition::RightBottom),
            "LeftBottom" => Ok(BoxAreaPosition::LeftBottom),
            "LeftTop" => Ok(BoxAreaPosition::LeftTop),
            _ => Err(()),
        }
    }
}

impl Display for BoxAreaContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxAreaContent::Nothing => write!(f, "Nothing"),
            BoxAreaContent::HiddenBox => write!(f, "HiddenBox"),
            BoxAreaContent::EmptyGlass => write!(f, "EmptyGlass"),
            BoxAreaContent::FilledBottle => write!(f, "FilledBottle"),
            BoxAreaContent::EmptyBottle => write!(f, "EmptyBottle"),
        }
    }
}

impl FromStr for BoxAreaContent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Nothing" => Ok(BoxAreaContent::Nothing),
            "HiddenBox" => Ok(BoxAreaContent::HiddenBox),
            "EmptyGlass" => Ok(BoxAreaContent::EmptyGlass),
            "FilledBottle" => Ok(BoxAreaContent::FilledBottle),
            "EmptyBottle" => Ok(BoxAreaContent::EmptyBottle),
            _ => Err(()),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::SpawnPlayer(player_id, x, y) => write!(f, "Spawn {}", player_id),
            Command::RemovePlayer(player_id) => write!(f, "Face {}", player_id),
            Command::FacePlayer(player_id, direction) => write!(f, "Face {} {}", player_id, direction),
            Command::MovePlayer(player_id, direction) => write!(f, "Move {} {}", player_id, direction),
            Command::StopPlayer(player_id) => write!(f, "Stop {}", player_id),
            Command::UpdateBoxArea(pos, content) => write!(f, "UpdateBoxArea {} {}", pos, content),
        }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');

        match parts.next() {
            Some("Spawn") => match parts.next() {
                Some(player_id) => match parts.next() {
                    Some(x) => match parts.next() {
                        Some(y) => Ok(Command::SpawnPlayer(player_id.to_string(), x.parse().unwrap(), y.parse().unwrap())),
                        _ => Err(())
                    },
                    _ => Err(())
                }
                _ => Err(())
            },
            Some("Remove") => match parts.next() {
                Some(player_id) => Ok(Command::RemovePlayer(player_id.to_string())),
                _ => Err(())
            },
            Some("Face") => match parts.next() {
                Some(player_id) => match parts.next() {
                    Some(direction) => Ok(Command::FacePlayer(player_id.to_string(), direction.parse::<Direction>().unwrap())),
                    _ => Err(()),
                },
                _ => Err(())
            },
            Some("Move") => match parts.next() {
                Some(player_id) => match parts.next() {
                    Some(direction) => Ok(Command::MovePlayer(player_id.to_string(), direction.parse::<Direction>().unwrap())),
                    _ => Err(()),
                },
                _ => Err(())
            },
            Some("Stop") => match parts.next() {
                Some(player_id) => Ok(Command::StopPlayer(player_id.to_string())),
                _ => Err(())
            },
            Some("UpdateBoxArea") => match parts.next() {
                Some(position) => {
                    let position = position.parse::<BoxAreaPosition>().unwrap();
                    match parts.next() {
                        Some(content) => {
                            let content = content.parse::<BoxAreaContent>().unwrap();
                            Ok(Command::UpdateBoxArea(position, content))
                        }
                        _ => Err(()),
                    }
                }
                _ => Err(()),
            },
            Some(_) | None => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::world::Direction::{Left, Up};
    use crate::world::{BoxAreaContent, BoxAreaPosition, Command};

    #[test]
    fn should_deserialize_command_line() {
        assert_eq!(
            Command::SpawnPlayer("1234".to_string(), 100, 200),
            "Spawn 1234 100 200".parse::<Command>().unwrap()
        );
        assert_eq!(
            Command::RemovePlayer("1234".to_string()),
            "Remove 1234".parse::<Command>().unwrap()
        );
        assert_eq!(
            Command::FacePlayer("1234".to_string(), Left),
            "Face 1234 Left".parse::<Command>().unwrap()
        );
        assert_eq!(
            Command::MovePlayer("1234".to_string(), Up),
            "Move 1234 Up".parse::<Command>().unwrap()
        );
        assert_eq!(Command::StopPlayer("1234".to_string()), "Stop 1234".parse::<Command>().unwrap());
        assert_eq!(
            Command::UpdateBoxArea(BoxAreaPosition::RightBottom, BoxAreaContent::HiddenBox),
            "UpdateBoxArea RightBottom HiddenBox"
                .parse::<Command>()
                .unwrap()
        );
    }
}
