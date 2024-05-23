use askama::Template;

#[derive(Template)]
#[template(path = "upload.html")]
pub struct UploadFormTemplate<'a> {
    pub backend_upload_endpoint: &'a str,
    pub upload_action_name: &'a str,
}
