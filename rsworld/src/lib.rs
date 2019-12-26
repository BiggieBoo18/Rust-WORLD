use rsworld_sys::{
    CheapTrick,
    GetFFTSizeForCheapTrick,
    CheapTrickOption,
    Dio,
    GetSamplesForDIO,
    DioOption
};

pub fn cheaptrick(x: &Vec<f64>, fs: i32, temporal_positions: &Vec<f64>, f0: &Vec<f64>, option: &mut CheapTrickOption) -> Vec<Vec<f64>> {
    let x_length:  i32 = x.len()  as i32;
    let f0_length: i32 = f0.len() as i32;
    unsafe {
	GetFFTSizeForCheapTrick(fs, option as *mut _);
    }
    let mut spectrogram     = vec![vec![0.0; (option.fft_size/2+1) as usize]; f0_length as usize];
    let mut spectrogram_ptr = spectrogram.iter_mut().map(|inner| inner.as_mut_ptr()).collect::<Vec<_>>();
    let spectrogram_ptr = spectrogram_ptr.as_mut_ptr();
    unsafe {
	CheapTrick(x.as_ptr(), x_length, fs, temporal_positions.as_ptr(), f0.as_ptr(), f0_length, option as *const _, spectrogram_ptr);
    }
    spectrogram
}

pub fn dio(x: &Vec<f64>, fs: i32, option: &DioOption) -> (Vec<f64>, Vec<f64>) {
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
    use crate::{cheaptrick,
		CheapTrickOption,
		dio,
		DioOption};
    #[test]
    fn test_cheaptrick() {
	let x                  = vec![0.0; 256];
	let fs 		       = 44100;
        let temporal_positions = vec![0.0, 0.005];
        let f0                 = vec![0.0, 0.0];
	let mut option         = CheapTrickOption::new(fs);
	let spectrogram = cheaptrick(&x, fs, &temporal_positions, &f0, &mut option);
	assert_eq!(spectrogram.len(), f0.len());
	assert_eq!(spectrogram[0].len(), (option.fft_size/2+1) as usize);
	assert_eq!(spectrogram[0][0], 0.0000000000000000973637408614245);
    }

    #[test]
    fn test_dio() {
        let x  = vec![0.0; 256];
        let fs = 44100;
        let option = DioOption::new();
        let (temporal_positions, f0) = dio(&x, fs, &option);
        assert_eq!(temporal_positions, vec![0.0, 0.005]);
        assert_eq!(f0,                 vec![0.0, 0.0]);
    }
}
