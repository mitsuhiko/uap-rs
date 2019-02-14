use super::{client::*, device::*, file::*, os::*, user_agent::*, Parser, SubParser};
use serde_yaml;

mod device;
mod os;
mod user_agent;

#[derive(Debug)]
pub struct UserAgentParser {
    user_agent_matchers: Vec<user_agent::Matcher>,
    os_matchers: Vec<os::Matcher>,
    device_matchers: Vec<device::Matcher>,
}

impl Parser for UserAgentParser {
    fn parse(&self, user_agent: &str) -> Option<Client> {
        unimplemented!()
    }

    fn parse_device(&self, user_agent: &str) -> Option<Device> {
        self.device_matchers
            .iter()
            .filter_map(|matcher| matcher.try_parse(&user_agent))
            .collect::<Vec<Device>>()
            .pop()
    }

    fn parse_os(&self, user_agent: &str) -> Option<OS> {
        unimplemented!()
    }

    fn parse_user_agent(&self, user_agent: &str) -> Option<UserAgent> {
        let mut matches = dbg!(self
            .user_agent_matchers
            .iter()
            .filter_map(|matcher| matcher.try_parse(&user_agent))
            .collect::<Vec<UserAgent>>());

        let first = dbg!(matches.last().map(UserAgent::to_owned));

        first
    }
}

impl UserAgentParser {
    pub fn from_yaml(path: &str) -> UserAgentParser {
        let mut file = std::fs::File::open(path).expect("File not found!");
        UserAgentParser::from_file(file)
    }

    pub fn from_file(file: std::fs::File) -> UserAgentParser {
        let regex_file: RegexFile = serde_yaml::from_reader(file).expect("Serde Error");
        UserAgentParser::from(regex_file)
    }
}

impl From<RegexFile> for UserAgentParser {
    fn from(regex_file: RegexFile) -> UserAgentParser {
        UserAgentParser {
            user_agent_matchers: regex_file
                .user_agent_parsers
                .into_iter()
                .map(user_agent::Matcher::from)
                .collect(),

            os_matchers: regex_file
                .os_parsers
                .into_iter()
                .map(os::Matcher::from)
                .collect(),

            device_matchers: regex_file
                .device_parsers
                .into_iter()
                .map(device::Matcher::from)
                .collect(),
        }
    }
}
