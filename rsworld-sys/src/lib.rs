use std::os::raw::{c_int, c_double};

// CheapTrick
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct CheapTrickOption {
    pub q1:       c_double,
    pub f0_floor: c_double,
    pub fft_size: c_int,
}

impl CheapTrickOption {
    pub fn new(fs: c_int) -> Self {
        unsafe {
            let mut option = std::mem::MaybeUninit::uninit().assume_init();
            InitializeCheapTrickOption(fs, &mut option as *mut _);
            option
        }
    }
}

#[link(name = "cheaptrick")]
extern {
    pub fn CheapTrick(x:                  *const c_double,
                      x_length:           c_int,
                      fs:                 c_int,
                      temporal_positions: *const c_double,
                      f0:                 *const c_double,
                      f0_length:          c_int,
                      option:             *const CheapTrickOption,
                      spectrogram:        *mut *mut c_double);
    pub fn InitializeCheapTrickOption(fs:     c_int,
                                      option: *mut CheapTrickOption);
    pub fn GetFFTSizeForCheapTrick(fs:     c_int,
                                   option: *mut CheapTrickOption);
}

// Dio
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct DioOption {
    pub f0_floor:           c_double,
    pub f0_ceil:            c_double,
    pub channels_in_octave: c_double,
    pub frame_period:       c_double,
    pub speed:              c_int,
    pub allowed_range:      c_double,
}

impl DioOption {
    pub fn new() -> Self {
        unsafe {
            let mut option:DioOption = std::mem::MaybeUninit::uninit().assume_init();
            InitializeDioOption(&mut option as *mut _);
            option
        }
    }
}

#[link(name = "dio")]
extern {
    pub fn Dio(x:                  *const c_double,
               x_length:           c_int,
               fs:                 c_int,
               option:             *const DioOption,
               temporal_positions: *mut c_double,
               f0:                 *mut c_double);
    pub fn InitializeDioOption(option: *mut DioOption);
    pub fn GetSamplesForDIO(fs:           c_int,
                            x_length:     c_int,
                            frame_period: c_double) -> c_int;
}

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    fn get_type<T>(_: T) -> &'static str {
        std::any::type_name::<T>()
    }

    // CheapTrick test
    use crate::{CheapTrick, GetFFTSizeForCheapTrick, CheapTrickOption};

    #[test]
    fn test_initialize_cheaptrick_option() {
        let fs     = 44100;
        let option = CheapTrickOption::new(fs);
        assert_eq!(option, CheapTrickOption { q1: -0.15, f0_floor: 71.0, fft_size: 2048 });
    }

    #[test]
    fn test_get_fft_size_for_cheaptrick() {
        let fs = 44100;
        let mut option = CheapTrickOption::new(fs);
        unsafe {
            GetFFTSizeForCheapTrick(fs, &mut option as *mut _);
            assert_eq!(option.fft_size, 2048);
        }
    }

    use std::os::raw::{c_double};

    #[test]
    fn test_cheaptrick() {
        let x: Vec<f64>        = vec![0.0; 256];
        let x_length           = x.len() as i32;
        let fs                 = 44100;
        let temporal_positions = vec![0.0, 0.005];
        let f0                 = vec![0.0, 0.0];
        let f0_length          = f0.len() as i32;
        let mut option         = CheapTrickOption::new(fs);
        unsafe {
            GetFFTSizeForCheapTrick(fs, &mut option as *mut _);
        }
	let xl = (option.fft_size/2+1) as usize;
	let yl = f0_length as usize;
	let mut spectrogram     = vec![vec![0.0; xl]; yl];
	let mut spectrogram_ptr = spectrogram.iter_mut().map(|inner| inner.as_mut_ptr()).collect::<Vec<_>>();
	let spectrogram_ptr = spectrogram_ptr.as_mut_ptr();
        unsafe {
            CheapTrick(x.as_ptr(), x_length, fs, temporal_positions.as_ptr(), f0.as_ptr(), f0_length, &option as *const _, spectrogram_ptr);
        }
	assert_eq!(spectrogram.len(), yl);
	assert_eq!(spectrogram[0].len(), xl);
	assert_eq!(spectrogram[0][0], 0.0000000000000000973637408614245);
    }

    // Dio test
    use crate::{Dio, GetSamplesForDIO, DioOption};

    #[test]
    fn test_initialize_dio_option() {
        let option = DioOption::new();
        assert_eq!(option, DioOption { f0_floor: 71.0, f0_ceil: 800.0, channels_in_octave: 2.0, frame_period: 5.0, speed: 1, allowed_range: 0.1 });
    }

    #[test]
    fn test_get_samples_for_dio() {
        let fs           = 44100;
        let x_length     = 256;
        let frame_period = 5.0;
        unsafe {
            let samples = GetSamplesForDIO(fs, x_length, frame_period);
            assert_eq!(samples, 2);
        }
    }

    #[test]
    fn test_dio() {
        let x: Vec<f64> = vec![0.0; 256];
        let x_length    = x.len() as i32;
        let fs          = 44100;
        let option      = DioOption::new();
        let f0_length: usize;
        unsafe {
            f0_length = GetSamplesForDIO(fs, x_length, option.frame_period) as usize;
        }
        let mut temporal_positions: Vec<f64> = vec![0.0; f0_length];
        let mut f0: Vec<f64>                 = vec![0.0; f0_length];
        unsafe {
            Dio(x.as_ptr(), x_length, fs, &option as *const _, temporal_positions.as_mut_ptr(), f0.as_mut_ptr());
        }
        assert_eq!(temporal_positions, vec![0.0, 0.005]);
        assert_eq!(f0,                 vec![0.0, 0.0]);
    }
}
