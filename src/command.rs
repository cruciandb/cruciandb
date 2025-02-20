use std::str::FromStr;

/// Command
#[derive(Debug)]
pub enum Command {
    Quit,
    Help,
    Get(String),
    Put(String, String),
    Delete(String),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c, arg) = if let Some((a, b)) = s.split_once(' ') {
            (a, Some(b))
        } else {
            (s, None)
        };
        Ok(match (c, arg) {
            ("quit", None) => Self::Quit,
            ("help", None) => Self::Help,
            ("get", Some(key)) => Self::Get(key.to_owned()),
            ("put", Some(key_val)) => {
                if let Some((k, v)) = key_val.split_once(' ') {
                    Self::Put(k.to_owned(), v.to_owned())
                } else {
                    return Err(());
                }
            }
            ("delete", Some(key)) => Self::Delete(key.to_owned()),
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn parse_command_without_args_test() {
        let res = Command::from_str("quit");
        assert!(res.is_ok());
        assert!(matches!(res.unwrap(), Command::Quit));
        let res = Command::from_str("help");
        assert!(res.is_ok());
        assert!(matches!(res.unwrap(), Command::Help));         
    }

    #[test]
    pub fn parse_command_with_args_test() {
        let res = Command::from_str("get 1");
        assert!(res.is_ok());
        match res.unwrap() {           
            Command::Get(key) => assert_eq!(key, "1"),
            _ => panic!("res is not a 'Get' command"), 
        }
        
        let res = Command::from_str("delete 1");
        assert!(res.is_ok());
        match res.unwrap() {           
            Command::Delete(key) => assert_eq!(key, "1"),
            _ => panic!("res is not a 'Delete' command"), 
        }

        let res = Command::from_str("put 1 2");
        assert!(res.is_ok());
        match res.unwrap() {           
            Command::Put(key, val) => {
                assert_eq!(key, "1");
                assert_eq!(val, "2");
            },
            _ => panic!("res is not a 'Put' command"), 
        }
    }


    #[test]
    pub fn parse_unknown_command_test() {
        let res = Command::from_str("unknown_command");
        assert!(res.is_err());        
    }
}
