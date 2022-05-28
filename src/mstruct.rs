use crate::{
    dstruct::{Agf, Agi},
    pound_fs::MountPoint,
};

pub struct AgCtx<'a> {
    pub mp: &'a MountPoint<'a>,
    pub ag_no: u32,
    pub agfCtx: &'a AgfCtx<'a>,
}

impl<'a> AgCtx<'a> {
    pub fn new(mp: &'a MountPoint<'a>, ag_no: u32, agfCtx: &'a AgfCtx<'a>) -> Self {
        AgCtx { mp, ag_no, agfCtx }
    }
}

// Ag Free Block
pub struct AgfCtx<'a> {
    pub mp: &'a MountPoint<'a>,
    pub agf: &'a mut Agf,
}

impl<'a> AgfCtx<'a> {
    pub fn new(mp: &'a MountPoint<'a>, agf: &'a mut Agf) -> Self {
        AgfCtx { mp, agf }
    }
}

// Ag Inode Block
pub struct AgiCtx<'a> {
    pub mp: &'a MountPoint<'a>,
    pub agi: &'a mut Agi,
}

impl<'a> AgiCtx<'a> {
    pub fn new(mp: &'a MountPoint<'a>, agi: &'a mut Agi) -> Self {
        AgiCtx { mp, agi }
    }
}

// Ag Free list
pub struct AgflCtx<'a> {
    pub mp: &'a MountPoint<'a>,
    pub agfl: &'a mut Agf,
}

impl<'a> AgflCtx<'a> {
    pub fn new(mp: &'a MountPoint<'a>, agfl: &'a mut Agf) -> Self {
        AgflCtx { mp, agfl }
    }
}
