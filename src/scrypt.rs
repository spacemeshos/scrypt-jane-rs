/// Scrypt algorithm parameters.
///
/// Refer to <https://www.rfc-editor.org/rfc/rfc7914#section-2>
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ScryptParams {
    /// N = 1 << (nfactor + 1)
    nfactor: u8,
    /// r = 1 << rfactor
    rfactor: u8,
    /// p = 1 << pfactor
    pfactor: u8,
}

impl ScryptParams {
    /// Create new ScryptParams.
    /// Note that the paramters are not N, r and p but _factors_.
    pub fn new(nfactor: u8, rfactor: u8, pfactor: u8) -> Self {
        Self {
            nfactor,
            rfactor,
            pfactor,
        }
    }
}

pub fn scrypt(password: &[u8], salt: &[u8], params: ScryptParams, output: &mut [u8]) {
    unsafe {
        scrypt_jane_sys::scrypt(
            password.as_ptr(),
            password.len(),
            salt.as_ptr(),
            salt.len(),
            params.nfactor,
            params.rfactor,
            params.pfactor,
            output.as_mut_ptr(),
            output.len(),
        );
    }
}

pub struct Scrypter {
    inst: *const scrypt_jane_sys::scrypt_instance,
}

// SAFETY: It's safe to send Scrypter to another thread because it's read-only.
unsafe impl std::marker::Sync for Scrypter {}

impl Scrypter {
    pub fn new(params: ScryptParams) -> Self {
        unsafe {
            Self {
                inst: scrypt_jane_sys::new_instance(params.nfactor, params.rfactor, params.pfactor),
            }
        }
    }

    pub fn scrypt(&self, password: &[u8], salt: &[u8], output: &mut [u8]) {
        unsafe {
            scrypt_jane_sys::scrypt_preallocated(
                self.inst,
                password.as_ptr(),
                password.len(),
                salt.as_ptr(),
                salt.len(),
                output.as_mut_ptr(),
                output.len(),
            );
        }
    }
}

impl Drop for Scrypter {
    fn drop(&mut self) {
        unsafe {
            scrypt_jane_sys::free_instance(self.inst);
        }
    }
}
