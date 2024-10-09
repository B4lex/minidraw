pub fn blend_colors(colors: Vec<u32>) -> u32 {
    let mut a_channels: Vec<u32> = vec![];
    let mut r_channels: Vec<u32> = vec![];
    let mut g_channels: Vec<u32> = vec![];
    let mut b_channels: Vec<u32> = vec![];

    for color in colors.iter() {
        a_channels.push(color >> 24 & 0xFFu32);
        r_channels.push((color & 0xFF0000) >> 16);
        g_channels.push((color & 0xFF00) >> 8);
        b_channels.push(color & 0xFF);
    }

    let blend_factor = colors.len() as u32;
    let a = a_channels.iter().sum::<u32>() / blend_factor;
    let r = r_channels.iter().sum::<u32>() / blend_factor;
    let g = g_channels.iter().sum::<u32>() / blend_factor;
    let b = b_channels.iter().sum::<u32>() / blend_factor;

    (a << 24 | r << 16 | g << 8) | b
}
