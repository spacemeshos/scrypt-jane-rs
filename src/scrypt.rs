pub struct ScryptParams {
    nfactor: u8,
    rfactor: u8,
    pfactor: u8,
}

impl ScryptParams {
    /// Create new ScryptParams.
    /// Note that the paramters are not N, r and p but _factors_.
    /// The underlaying scrypt-jane library calculates them as:
    /// - N = 1 << (nfactor + 1)
    /// - r = 1 << rfactor
    /// - p = 1 << pfactor
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
