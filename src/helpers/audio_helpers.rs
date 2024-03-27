const K0: [i32;4] = [0, 240, 460, 392];
const K1: [i32;4] = [0, 0, -208, -220];

fn limit_sample(sample: i32) -> i16 {
    if sample > 32767 {
        32767
    } else if sample < -32768 {
        -32768
    } else {
        sample as i16
    }
}

fn decode_adpcm(su: usize, gain: i32, sd: &[&[i8]], ranges: &[u8], filters: &[u8], stereo: bool, left: &mut Vec<i16>, right: &mut Vec<i16>) -> u8 {
    let mut index: u8 = 0;

    let mut lk0 = 0;
    let mut rk0 = 0;
    let mut lk1 = 0;
    let mut rk1 = 0;

    for i in 0..su {
        let cur_gain = 2u16 << (gain - ranges[i] as i32);
        for ss in 0..28 {
            if stereo && (i & 1) == 1 {
                let sample = limit_sample((sd[i][ss] as i32 * cur_gain as i32) + ((rk0 * K0[filters[i] as usize] as i32 + rk1 * K1[filters[i] as usize] as i32) / 256));
                rk1 = rk0;
                rk0 = sample.into();
                right.push(sample);
                index += 1;
            } else {
                let sample = limit_sample((sd[i][ss] as i32 * cur_gain as i32) + ((lk0 * K0[filters[i] as usize] as i32 + lk1 * K1[filters[i] as usize] as i32) / 256));
                lk1 = lk0;
                lk0 = sample.into();
                left.push(sample);
                index += 1;
            }
        }
    }

    index
}

// DecodeLevelASoundGroup function takes parameters similar to the C# code
fn decode_level_a_sound_group(stereo: bool, data: &[u8], left: &mut Vec<i16>, right: &mut Vec<i16>) -> u8 {
    // Initialize index to 16
    let mut index: u8 = 16;

    // Initialize range and filter arrays with length 4
    let mut range = [0u8; 4];
    let mut filter = [0u8; 4];

    // Initialize SD array as a 2D array with dimensions 4x28
    let mut sd = [[0i8; 28]; 4];

    // Iterate over the range and filter arrays
    for i in 0..4 {
        // Extract lower 4 bits to determine range
        range[i] = data[i] & 0x0F;
        // Extract upper 4 bits to determine filter
        filter[i] = data[i] >> 4;
    }

    // Iterate over sound samples (ss) from 0 to 27
    for ss in 0..28 {
        // Iterate over sound units (su) from 0 to 3
        for su in 0..4 {
            // Fill SD array with data from index onwards
            sd[su][ss] = data[index as usize] as i8;
            // Increment index
            index += 1;
        }
    }

    // Call DecodeADPCM function with appropriate parameters
    index = decode_adpcm(4, 8, &sd.iter().map(|x| x.as_ref()).collect::<Vec<_>>(), &range, &filter, stereo, left, right);

    // Return the updated index
    index
}

fn decode_level_bc_sound_group(stereo: bool, data: &[u8], left: &mut Vec<i16>, right: &mut Vec<i16>) -> u8 {
    let mut index: usize = 4;
    let mut range = [0u8; 8];
    let mut filter = [0u8; 8];
    let mut sd = [[0i8; 28]; 8];

    for i in 0..8 {
        range[i] = data[i + index] & 0x0F;
        filter[i] = data[i + index] >> 4;
        sd[i] = [0i8; 28];
    }

    index = 16;
    for ss in 0..28 {
        for su in (0..8).step_by(2) {
            let sb = data[index as usize];
            sd[su][ss] = (sb & 0x0F) as i8;
            if sd[su][ss] >= 8 {
                sd[su][ss] -= 16;
            }
            sd[su + 1][ss] = (sb >> 4) as i8;
            if sd[su + 1][ss] >= 8 {
                sd[su + 1][ss] -= 16;
            }
            index += 1;
        }
    }

    decode_adpcm(8, 12, &sd.iter().map(|x| x.as_ref()).collect::<Vec<_>>(), &range, &filter, stereo, left, right)
}
