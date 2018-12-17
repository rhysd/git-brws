use crate::envvar;

pub fn empty_env() -> envvar::Envvar {
    envvar::Envvar {
        git_command: "git".to_string(),
        ghe_ssh_port: None,
        ghe_url_host: None,
        gitlab_ssh_port: None,
    }
}
