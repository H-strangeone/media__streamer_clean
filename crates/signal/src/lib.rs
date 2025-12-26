    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Signal{
        pub samples: Vec<f64>,
        pub sample_rate:u32,
    }
    impl Signal{
        pub fn new(samples:Vec<f64>, sample_rate:u32) -> Self{
            Self{
                samples,
                sample_rate,
            }
        }
        pub fn duration_secs(&self)->f64{
            self.samples.len() as f64/ self.sample_rate as f64
        }
    }
    use std::f64::consts::PI;

    pub fn sine_wave(freq: f64, duration: f64, sample_rate: u32) -> Signal {
        let total_samples = (duration * sample_rate as f64) as usize;

        let samples = (0..total_samples)
            .map(|n| {
                let t = n as f64 / sample_rate as f64;
                (2.0 * PI * freq * t).sin()
            })
            .collect();

        Signal::new(samples, sample_rate)
    }
