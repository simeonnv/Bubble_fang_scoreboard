use chrono::Local;
use uuid::Uuid;

use crate::{Session, SharedSessions};



pub async fn create_session(sessions: &SharedSessions) -> String {
    
    let uuid = Uuid::new_v4().to_string();

    let mut sessions = sessions.lock().await;
    (*sessions).push(Session {
        started: Local::now(),
        uuid: uuid.clone()
    });

    return uuid;
     
}