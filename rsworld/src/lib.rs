use rsworld_sys::{
    CheapTrick,
    GetFFTSizeForCheapTrick,
    CheapTrickOption,
    GetNumberOfAperiodicities,
    CodeAperiodicity,
    DecodeAperiodicity,
    CodeSpectralEnvelope,
    DecodeSpectralEnvelope,
    D4C,
    D4COption,
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

pub fn get_number_of_aperiodicities(fs: i32) -> i32 {
    unsafe {
	GetNumberOfAperiodicities(fs)
    }
}

pub fn code_aperiodicity(aperiodicity: &Vec<Vec<f64>>, f0_length: i32, fs: i32, fft_size: i32) -> Vec<Vec<f64>> {
    let aperiodicity_ptr           = aperiodicity.iter().map(|inner| inner.as_ptr()).collect::<Vec<_>>();
    let aperiodicity_ptr           = aperiodicity_ptr.as_ptr();
    let n_aperiodicity;
    unsafe {
	n_aperiodicity             = GetNumberOfAperiodicities(fs);
    }
    let mut coded_aperiodicity     = vec![vec![0.0; n_aperiodicity as usize]; f0_length as usize];
    let mut coded_aperiodicity_ptr = coded_aperiodicity.iter_mut().map(|inner| inner.as_mut_ptr()).collect::<Vec<_>>();
    let coded_aperiodicity_ptr     = coded_aperiodicity_ptr.as_mut_ptr();
    unsafe {
	CodeAperiodicity(aperiodicity_ptr, f0_length, fs, fft_size, coded_aperiodicity_ptr);
    }
    coded_aperiodicity
}

pub fn decode_aperiodicity(coded_aperiodicity: &Vec<Vec<f64>>, f0_length: i32, fs: i32, fft_size: i32) -> Vec<Vec<f64>> {
    let coded_aperiodicity_ptr = coded_aperiodicity.iter().map(|inner| inner.as_ptr()).collect::<Vec<_>>();
    let coded_aperiodicity_ptr = coded_aperiodicity_ptr.as_ptr();
    let mut aperiodicity       = vec![vec![0.0; (fft_size/2+1) as usize]; f0_length as usize];
    let mut aperiodicity_ptr   = aperiodicity.iter_mut().map(|inner| inner.as_mut_ptr()).collect::<Vec<_>>();
    let aperiodicity_ptr       = aperiodicity_ptr.as_mut_ptr();
    unsafe {
	DecodeAperiodicity(coded_aperiodicity_ptr, f0_length, fs, fft_size, aperiodicity_ptr);
    }
    aperiodicity
}

pub fn code_spectral_envelope(spectrogram: &Vec<Vec<f64>>, f0_length: i32, fs: i32, fft_size: i32, number_of_dimensions: i32) -> Vec<Vec<f64>> {
    let spectrogram_ptr = spectrogram.iter().map(|inner| inner.as_ptr()).collect::<Vec<_>>();
    let spectrogram_ptr = spectrogram_ptr.as_ptr();
    let mut coded_spectral_envelope     = vec![vec![0.0; number_of_dimensions as usize]; f0_length as usize];
    let mut coded_spectral_envelope_ptr = coded_spectral_envelope.iter_mut().map(|inner| inner.as_mut_ptr()).collect::<Vec<_>>();
    let coded_spectral_envelope_ptr = coded_spectral_envelope_ptr.as_mut_ptr();
    unsafe {
	CodeSpectralEnvelope(spectrogram_ptr, f0_length, fs, fft_size, number_of_dimensions, coded_spectral_envelope_ptr);

    }
    coded_spectral_envelope
}

pub fn decode_spectral_envelope(coded_spectrogram: &Vec<Vec<f64>>, f0_length: i32, fs: i32, fft_size: i32) -> Vec<Vec<f64>> {
    let number_of_dimensions = coded_spectrogram[0].len() as i32;
    let mut spectrogram      = vec![vec![0.0; (fft_size/2+1) as usize]; f0_length as usize];
    let mut spectrogram_ptr = spectrogram.iter_mut().map(|inner| inner.as_mut_ptr()).collect::<Vec<_>>();
    let spectrogram_ptr = spectrogram_ptr.as_mut_ptr();
    let coded_spectrogram_ptr = coded_spectrogram.iter().map(|inner| inner.as_ptr()).collect::<Vec<_>>();
    let coded_spectrogram_ptr = coded_spectrogram_ptr.as_ptr();
    unsafe {
	DecodeSpectralEnvelope(coded_spectrogram_ptr, f0_length, fs, fft_size, number_of_dimensions, spectrogram_ptr);
    }
    spectrogram
}

pub fn d4c(x: &Vec<f64>, fs: i32, temporal_positions: &Vec<f64>, f0: &Vec<f64>, fft_size: i32, option: &D4COption) -> Vec<Vec<f64>> {
    let x_length  = x.len()  as i32;
    let f0_length = f0.len() as i32;
    let mut aperiodicity     = vec![vec![0.0; (fft_size/2+1) as usize]; f0_length as usize];
    let mut aperiodicity_ptr = aperiodicity.iter_mut().map(|inner| inner.as_mut_ptr()).collect::<Vec<_>>();
    let aperiodicity_ptr     = aperiodicity_ptr.as_mut_ptr();
    unsafe {
	D4C(x.as_ptr(), x_length, fs, temporal_positions.as_ptr(), f0.as_ptr(), f0_length, fft_size, option as *const _, aperiodicity_ptr);
    }
    aperiodicity
}

pub fn dio(x: &Vec<f64>, fs: i32, option: &DioOption) -> (Vec<f64>, Vec<f64>) {
    let x_length = x.len() as i32;
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
		get_number_of_aperiodicities,
		code_aperiodicity,
		decode_aperiodicity,
		code_spectral_envelope,
		decode_spectral_envelope,
		d4c,
		D4COption,
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
    }

    #[test]
    fn test_get_number_of_aperiodicities() {
	let fs = 44100;
	assert_eq!(get_number_of_aperiodicities(fs), 5);
    }

    #[test]
    fn test_code_aperiodicity() {
	let x                  = vec![0.0; 256];
	let fs                 = 44100 as i32;
        let temporal_positions = vec![0.0, 0.005];
        let f0                 = vec![0.0, 0.0];
	let fft_size           = 2048 as i32;
	let option             = D4COption::new();

	let aperiodicity = d4c(&x, fs, &temporal_positions, &f0, fft_size, &option);
	let coded_aperiodicity = code_aperiodicity(&aperiodicity, f0.len() as i32, fs, fft_size);
	assert_eq!(coded_aperiodicity.len(),    f0.len());
	assert_eq!(coded_aperiodicity[0].len(), get_number_of_aperiodicities(fs) as usize);
    }

    #[test]
    fn test_decode_aperiodicity() {
	let x                  = vec![0.0; 256];
	let fs                 = 44100 as i32;
        let temporal_positions = vec![0.0, 0.005];
        let f0                 = vec![0.0, 0.0];
	let fft_size           = 2048 as i32;
	let option             = D4COption::new();

	let aperiodicity = d4c(&x, fs, &temporal_positions, &f0, fft_size, &option);
	let coded_aperiodicity  = code_aperiodicity(&aperiodicity, f0.len() as i32, fs, fft_size);
	let decode_aperiodicity = decode_aperiodicity(&coded_aperiodicity, f0.len() as i32, fs, fft_size);
	assert_eq!(aperiodicity.len(), f0.len());
	assert_eq!(aperiodicity[0].len(), (fft_size/2+1) as usize);
	assert_eq!(aperiodicity[0][0], 0.999999999999);
    }

    #[test]
    fn test_code_spectral_envelope() {
	let x                  = vec![0.0; 256];
	let fs 		       = 44100;
        let temporal_positions = vec![0.0, 0.005];
        let f0                 = vec![0.0, 0.0];
	let mut option         = CheapTrickOption::new(fs);
	let spectrogram        = cheaptrick(&x, fs, &temporal_positions, &f0, &mut option);
	let number_of_dimensions = 256;
	let coded_spectrogram  = code_spectral_envelope(&spectrogram, f0.len() as i32, fs, option.fft_size, number_of_dimensions);
	assert_eq!(coded_spectrogram.len(), f0.len());
	assert_eq!(coded_spectrogram[0].len(), number_of_dimensions as usize);
    }

    #[test]
    fn test_decode_spectral_envelope() {
	let x                  = vec![0.0; 256];
	let fs 		       = 44100;
        let temporal_positions = vec![0.0, 0.005];
        let f0                 = vec![0.0, 0.0];
	let mut option         = CheapTrickOption::new(fs);
	let spectrogram        = cheaptrick(&x, fs, &temporal_positions, &f0, &mut option);
	let number_of_dimensions = 256;
	let coded_spectrogram  = code_spectral_envelope(&spectrogram, f0.len() as i32, fs, option.fft_size, number_of_dimensions);
	let spectrogram        = decode_spectral_envelope(&coded_spectrogram, f0.len() as i32, fs, option.fft_size);
	assert_eq!(spectrogram.len(),    f0.len());
	assert_eq!(spectrogram[0].len(), (option.fft_size/2+1) as usize);
    }

    #[test]
    fn test_d4c() {
	let x                  = vec![0.0; 256];
	let fs                 = 44100 as i32;
        let temporal_positions = vec![0.0, 0.005];
        let f0                 = vec![0.0, 0.0];
	let fft_size           = 2048 as i32;
	let option             = D4COption::new();

	let aperiodicity = d4c(&x, fs, &temporal_positions, &f0, fft_size, &option);
	assert_eq!(aperiodicity.len(), f0.len());
	assert_eq!(aperiodicity[0].len(), (fft_size/2+1) as usize);
	assert_eq!(aperiodicity[0][0], 0.999999999999);
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
