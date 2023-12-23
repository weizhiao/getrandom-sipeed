use crate::Error;
use core::mem::MaybeUninit;

extern "C" {
    fn rand() -> u32;
}

pub fn getrandom_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
	for elem in dest{
		unsafe { elem.as_mut_ptr().write(rand() as _) }
	}
    Ok(())
}
