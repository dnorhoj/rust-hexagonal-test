pub struct Note {
    pub id: String,
    pub title: Option<String>,
    pub body: String,
}

pub struct CreateNote {
    pub title: Option<String>,
    pub body: String,
}

impl CreateNote {
    pub fn new(title: Option<String>, body: String) -> Self {
        Self { title, body}
    }
}
