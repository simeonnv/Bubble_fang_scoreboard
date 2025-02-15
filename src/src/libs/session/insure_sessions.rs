use chrono::{Duration, Local};
use crate::SharedSessions;



pub async fn insure_sessions(sessions: &SharedSessions) {
    
    let mut sessions = sessions.lock().await;

    let now = Local::now();

    sessions.retain(|session| {
        let duration = now - session.started;
        duration < Duration::minutes(30)
    });
     
}