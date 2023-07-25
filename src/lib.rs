use serde_json::Value;

struct PayloadParser {
    parsed_payload: Value,
}

impl PayloadParser {
    fn new(payload: String) -> Result<Self, serde_json::Error> {
        let parsed_payload: Value = serde_json::from_str(&payload.as_str())?;
        Ok(Self { parsed_payload })
    }

    fn extract_value(&self, path: &[&'static str]) -> &str {
        let mut intermediate_value = &self.parsed_payload;
        for path_part in path.to_owned() {
            intermediate_value = &intermediate_value[path_part];
        }
        intermediate_value.as_str().unwrap_or("")
    }
}

pub fn get_issue_text(payload: String) -> String {
    let payload_parser = PayloadParser::new(payload).unwrap();
    let text = format!(
        "Issue is <b>{}</b>, for more details see {}",
        payload_parser.extract_value(&["action"]),
        payload_parser.extract_value(&["issue", "html_url"]),
    );
    text
}

pub fn get_pr_text(payload: String) -> String {
    let payload_parser = PayloadParser::new(payload).unwrap();
    let action = payload_parser.extract_value(&["action"]);
    let text: String = match action {
        "review_requested" => {
            format!(
                "Review was requested for <a href='{}'>pull request</a> from <a href='{}'>{}</a> by <a href='{}'>{}</a> for <a href='{}'>{}</a>",
                payload_parser.extract_value(&["pull_request", "html_url"]),
                payload_parser.extract_value(&["requested_reviewer", "html_url"]),
                payload_parser.extract_value(&["requested_reviewer", "login"]),
                payload_parser.extract_value(&["sender", "html_url"]),
                payload_parser.extract_value(&["sender", "login"]),
                payload_parser.extract_value(&["repository", "html_url"]),
                payload_parser.extract_value(&["repository", "full_name"]),
            )
        }
        "assigned" => {
            format!(
                "<a href='{}'>Pull request</a> was assigned to <a href='{}'>{}</a> by <a href='{}'>{}</a> for <a href='{}'>{}</a>",
                payload_parser.extract_value(&["pull_request", "html_url"]),
                payload_parser.extract_value(&["assignee", "html_url"]),
                payload_parser.extract_value(&["assignee", "login"]),
                payload_parser.extract_value(&["sender", "html_url"]),
                payload_parser.extract_value(&["sender", "login"]),
                payload_parser.extract_value(&["repository", "html_url"]),
                payload_parser.extract_value(&["repository", "full_name"]),
            )
        }
        _ => {
            format!(
                "<a href='{}'>Pull request</a> for <a href='{}'>{}</a> was <b>{}</b> by <a href='{}'>{}</a>.",
                payload_parser.extract_value(&["pull_request", "html_url"]),
                payload_parser.extract_value(&["repository", "html_url"]),
                payload_parser.extract_value(&["repository", "full_name"]),
                action,
                payload_parser.extract_value(&["pull_request", "user", "html_url"]),
                payload_parser.extract_value(&["pull_request", "user", "login"]),
            )
        }
    };
    text
}

pub fn get_pr_review_text(payload: String) -> String {
    let payload_parser = PayloadParser::new(payload).unwrap();
    let text = format!(
        "<a href='{}'>Review</a> ({}) for <a href='{}'>PR</a> was <b>{}</b> by <a href='{}'>{}</a>.\n\n<i>{}</i>",
        payload_parser.extract_value(&["review", "html_url"]),
        payload_parser.extract_value(&["review", "state"]),
        payload_parser.extract_value(&["pull_request", "html_url"]),
        payload_parser.extract_value(&["action"]),
        payload_parser.extract_value(&["review", "user", "html_url"]),
        payload_parser.extract_value(&["review", "user", "login"]),
        payload_parser.extract_value(&["review", "body"]),
    );
    text
}

pub fn get_pr_review_comment_text(payload: String) -> String {
    let payload_parser = PayloadParser::new(payload).unwrap();
    let text = format!(
        "<a href='{}'>Review comment</a> for <a href='{}'>PR</a> was <b>{}</b> by <a href='{}'>{}</a>.\n\n<i>{}</i>",
        payload_parser.extract_value(&["comment", "html_url"]),
        payload_parser.extract_value(&["pull_request", "html_url"]),
        payload_parser.extract_value(&["action"]),
        payload_parser.extract_value(&["comment", "user", "html_url"]),
        payload_parser.extract_value(&["comment", "user", "login"]),
        payload_parser.extract_value(&["comment", "body"]),
    );
    text
}
