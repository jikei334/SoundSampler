pub fn resample_linear(data: Vec<f32>, factor: f32) -> Vec<f32> {
    if data.is_empty() {
        return data;
    }

    let len = ((data.len() as f32) / factor).floor().max(1f32) as usize;
    let mut out = Vec::with_capacity(len);
    for n in 0..len {
        let src_pos = (n as f32) * factor;
        let i = src_pos.floor() as usize;
        let frac = src_pos - (i as f32);
        let s0 = data[i];
        let s1 = if i + 1 < data.len() {
            data[i+1]
        } else {
            data[i]
        };
        out.push(s0 + (s1 - s0) * frac);
    }

    out
}

pub fn resample_data(data: Vec<f32>, original_sample_rate: u32, target_sample_rate: u32) -> Vec<f32> {
    let factor = original_sample_rate as f32 / target_sample_rate as f32;
    resample_linear(data, factor)
}

pub fn normalize_data(data: Vec<f32>, peak: f32) -> Vec<f32> {
    let m = data.iter().fold(0f32, |a, &x| a.max(x.abs()));
    let mut out = data.clone();
    if m > 0.0 {
        let g = peak / m;
        for s in out.iter_mut() {
            *s *= g;
        }
    }

    out
}
