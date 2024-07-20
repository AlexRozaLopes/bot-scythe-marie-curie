use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serenity::all::Member;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MemberModel {
    member: Member,
    is_alive: bool,
    last_time_active: Option<DateTime<Utc>>,
    is_silenced: bool,
}

impl MemberModel {
    pub fn new(member: Member) -> Self {
        Self {
            member,
            is_alive: false,
            last_time_active: Option::from(Utc::now()),
            is_silenced: false,
        }
    }
    pub fn set_member(&mut self, member: Member) {
        self.member = member;
    }
    pub fn set_living(&mut self, is_alive: bool) {
        self.is_alive = is_alive;
    }
    pub fn set_last_time_active(&mut self, last_time_active: Option<DateTime<Utc>>) {
        self.last_time_active = last_time_active;
    }
    pub fn set_silenced(&mut self, is_silenced: bool) {
        self.is_silenced = is_silenced;
    }
    pub fn member(&self) -> &Member {
        &self.member
    }
    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
    pub fn last_time_active(&self) -> Option<&DateTime<Utc>> {
        self.last_time_active.as_ref()
    }
    pub fn is_silenced(&self) -> bool {
        self.is_silenced
    }
}
