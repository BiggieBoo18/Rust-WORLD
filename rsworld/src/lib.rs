use rsworld_sys::{
    Dio,
    GetSamplesForDIO,
    DioOption
};

pub fn dio(x: Vec<f64>, fs: i32, option: &DioOption) -> (Vec<f64>, Vec<f64>) {
    let x_length: i32 = x.len() as i32;
    let f0_length: usize;
    unsafe {
        f0_length = GetSamplesForDIO(fs, x_length, option.frame_period) as usize;
    }
    let mut temporal_positions: Vec<f64> = vec![0.0; f0_length];
    let mut f0:                 Vec<f64> = vec![0.0; f0_length];
    unsafe {
        Dio(x.as_ptr(), x_length, fs, option as *const _, temporal_positions.as_mut_ptr(), f0.as_mut_ptr());
    }
    (temporal_positions, f0)
}

#[cfg(test)]
mod tests {
    use crate::{dio, DioOption};
    #[test]
    fn test_dio() {
        let x = vec![0.0; 256];
        let fs = 44100;
        let option = DioOption::new();
        let (temporal_positions, f0) = dio(x, fs, &option);
        assert_eq!(temporal_positions, vec![0.0, 0.005]);
        assert_eq!(f0,                 vec![0.0, 0.0]);
    }
}
