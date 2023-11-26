pub fn parse(user_agent: &str) -> Vec<str> {
    x.agent.split(' ').map(|x| {
        let out = x.replace('(', "");
        let out = out.replace(')', "");
        let out = out.replace(';', "");
        let parts: Vec<&str> = out.split('/').collect();
        parts[0].to_owned()
    })
}
