use serenity::all::Member;

#[derive(Clone,Debug)]
pub struct Membro {
    membro: Member,
    ativo: bool
}

impl Membro {
    pub fn new(membro: Member) -> Self {
        Self { membro, ativo : false }
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
}



