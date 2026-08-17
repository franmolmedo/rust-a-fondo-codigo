struct DeviceBuffer {
    data: *mut u8, // los punteros crudos anulan la derivación automática
    len: usize,
}

impl DeviceBuffer {
    fn write(&mut self, offset: usize, byte: u8) {
        assert!(offset < self.len);
        // SAFETY: poseemos en exclusiva [data, data+len) y &mut self
        // garantiza acceso único durante la escritura.
        unsafe { self.data.add(offset).write(byte) }
    }
}

// SAFETY: DeviceBuffer posee en exclusiva su región de memoria; ningún
// alias externo sobrevive a la construcción, toda mutación exige &mut self
// y la liberación ocurre una sola vez en Drop. Moverlo a otro thread
// transfiere la autoridad completa, sin estado afín a un thread concreto.
unsafe impl Send for DeviceBuffer {}
