use flate2::read::ZlibEncoder;
use flate2::Compression;
use lodepng::ColorType::PALETTE;
use lodepng::{CompressSettings, State, RGBA};
use std::io::Read;
use std::os::raw::c_uchar;
use std::path::{Path, PathBuf};
use std::str;

pub struct Options {
    pub add_ext: bool,
}

pub fn compress_file(file_name: String, options: Options) {
    let path = Path::new(&file_name);
    if !path.is_file() {
        eprintln!("The Given Path is not a File");
    }

    let in_file_path = PathBuf::from(&path);

    let file = match lodepng::decode32_file(&in_file_path) {
        Ok(file) => file,
        Err(e) => panic!("Could Not Open File\nError: {e:?}"),
    };

    let add_ext = match options.add_ext {
        true => ".min",
        _ => "",
    };

    let out_file_name = format!(
        "{}{}.{}",
        path.file_stem().unwrap().to_str().unwrap(),
        add_ext,
        path.extension().unwrap().to_str().unwrap()
    );

    let out_file_path = path.with_file_name(out_file_name);

    let height = file.height;
    let width = file.width;

    let (palette, pixels) = quantize(&file.buffer, width as usize, height as usize);
    let mut state = make_state();

    match state.encode_file(&out_file_name_path_buf, &pixels, width, height) {
        Err(e) => {
            let err_msg = str::from_utf8(e.c_description());
            let err_msg = err_msg.ok().unwrap();
            println!("{}", err_msg);
        },
        _ => {},
    }
}

fn quantize(buffer: &[RGBA], width: usize, height: usize) -> (Vec<RGBA>, Vec<u8>) {
    let mut liq = imagequant::new();
    liq.set_speed(1);
    liq.set_quality(70, 99);
    let ref mut img = liq
        .new_image(&buffer, width as usize, height as usize, 0.45455)
        .unwrap();
    let mut res = match liq.quantize(img) {
        Ok(res) => res,
        Err(_) => panic!("Failed to quantize image"),
    };
    res.remapped(img).unwrap()
}

#[allow(deprecated)]
fn make_state() -> State {
    let mut state = lodepng::ffi::State::new();
    state.info_png_mut().color.colortype = PALETTE;
    state.info_png_mut().color.set_bitdepth(8);

    state.info_raw_mut().colortype = PALETTE;
    state.info_raw_mut().set_bitdepth(8);

    unsafe {
        state.set_custom_zlib(Some(deflate_ffi), std::ptr::null());
    }

    state.encoder.add_id = 0;
    state.encoder.text_compression = 1;

    state
}

fn add_palette_to_state(state: &mut State, palette: Vec<RGBA>) {
    palette.iter().for_each(|&palette| {
        state
            .info_png_mut()
            .color
            .palette_add(palette.clone())
            .unwrap();

        state.info_raw_mut().palette_add(palette.clone()).unwrap();
    })
}

// to override the default compressor for lodepng, an unsafe external c function has to be passed to used
#[allow(unused_must_use)]
unsafe extern "C" fn deflate_ffi(
    out: &mut *mut c_uchar,
    out_size: &mut usize,
    input: *const c_uchar,
    input_size: usize,
    settings: *const CompressSettings,
) -> u32 {
    let input = vec_from_raw(input, input_size);
    let settings = std::ptr::read(settings);

    let (mut buffer, size) = deflate(&input, settings);

    std::mem::replace(out, buffer.as_mut_ptr());
    std::ptr::replace(out_size, size);
    return 0;
}

unsafe fn vec_from_raw(data: *const c_uchar, len: usize) -> Vec<u8> {
    std::slice::from_raw_parts(data, len).to_owned()
}

// call flate2's zlib encoder return the buffer and length
fn deflate(input: &[u8], _settings: CompressSettings) -> (Vec<u8>, usize) {
    let mut z = ZlibEncoder::new(input, Compression::best());
    let mut buffer = vec![];
    match z.read_to_end(&mut buffer) {
        Ok(len) => (buffer, len),
        Err(_) => panic!("Failed to compress buffer"),
    }
}
