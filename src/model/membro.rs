use chrono::{DateTime, Utc};
use serenity::all::Member;

#[derive(Clone, Debug)]
pub struct Membro {
    membro: Member,
    ativo: bool,
    ativo_em: Option<DateTime<Utc>>,
}

impl Membro {
    pub fn new(membro: Member) -> Self {
        Self { membro, ativo: false, ativo_em: None }
    }
    pub fn set_membro(&mut self, membro: Member) {
        self.membro = membro;
    }
    pub fn set_ativo(&mut self, ativo: bool) {
        self.ativo = ativo;
    }
    pub fn ativo(&self) -> bool {
        self.ativo
    }
    pub fn membro(&self) -> &Member {
        &self.membro
    }
    pub fn set_ativo_em(&mut self, ativo_em: Option<DateTime<Utc>>) {
        self.ativo_em = ativo_em;
    }
    pub fn ativo_em(&self) -> Option<DateTime<Utc>> {
        self.ativo_em
    }
}



