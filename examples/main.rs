use image::GenericImageView;
use image::ImageBuffer;
use image::Luma;
use image::LumaA;
use image::Pixel;
use std::path::Path;

fn main() {
    let rgb_img = image::open(Path::new("examples/rocky_terrain_diff_4k.jpg")).unwrap();

    let (width, height) = rgb_img.dimensions();
    println!("Width is {}", width);
    println!("Height is {}", height);
    println!("ColorType is {:?}", rgb_img.color());

    let mut rgba_img = ImageBuffer::new(width, height);
    let mut rg_img = ImageBuffer::new(width, height);
    let mut r_img = ImageBuffer::new(width, height);

    println!("Converting RGB -> RGBA/RG/R"); // could be optimized
    for x in 0u32..width {
        for y in 0u32..height {
            let pixel = rgb_img.get_pixel(x, y);
            let pixel_rgba = pixel.to_rgba();
            let pixel_rg = LumaA::from([pixel_rgba[0], pixel_rgba[1]]);
            let pixel_r = Luma::from([pixel_rgba[0]]);
            rgba_img.put_pixel(x, y, pixel_rgba);
            rg_img.put_pixel(x, y, pixel_rg);
            r_img.put_pixel(x, y, pixel_r);
        }
    }

    let rgba_img = rgba_img.to_vec();
    let slice = [rgba_img];

    let rgba_img = astcenc_rs::Image{ extents: astcenc_rs::Extents::new(width, height), data: slice.as_slice() };

    
    let mut ctx = astcenc_rs::Context::default();
    let swz = astcenc_rs::Swizzle::rgba();

    let data = ctx.compress(&rgba_img, swz).unwrap();

    let img2 = ctx.decompress::<u8>(&data, rgba_img.extents, swz).unwrap();

    assert_eq!(rgba_img.extents, img2.extents);
    assert_eq!(rgba_img.data.len(), img2.data.len());
    /*assert!(rgba_img
        .data
        .iter()
        .zip(img2.data.iter())
        .all(|(a, b)| a.len() == b.len()));

    let block_count = intel_tex_2::divide_up_by_multiple(width * height, 16);
    println!("Block count: {}", block_count);
    let dds_defaults = NewDxgiParams {
        height,
        width,
        depth: Some(1),
        format: DxgiFormat::BC7_UNorm,
        mipmap_levels: Some(1),
        array_layers: Some(1),
        caps2: Some(Caps2::empty()),
        is_cubemap: false,
        resource_dimension: D3D10ResourceDimension::Texture2D,
        alpha_mode: AlphaMode::Opaque,
    };
    // BC4
    {
        let mut dds = Dds::new_dxgi(NewDxgiParams {
            format: DxgiFormat::BC4_UNorm,
            ..dds_defaults
        })
        .unwrap();

        let surface = intel_tex_2::RSurface {
            width,
            height,
            stride: width,
            data: &r_img,
        };

        println!("Compressing to BC4...");
        bc4::compress_blocks_into(&surface, dds.get_mut_data(0 /* layer */).unwrap());
        println!("  Done!");
        println!("Saving lambertian_bc4.dds file");
        let mut dds_file = File::create("examples/lambertian_bc4.dds").unwrap();
        dds.write(&mut dds_file).expect("Failed to write dds file");
    }
    // BC5
    {
        let mut dds = Dds::new_dxgi(NewDxgiParams {
            format: DxgiFormat::BC5_UNorm,
            ..dds_defaults
        })
        .unwrap();
        let surface = intel_tex_2::RgSurface {
            width,
            height,
            stride: width * 2,
            data: &rg_img,
        };

        println!("Compressing to BC5...");
        bc5::compress_blocks_into(&surface, dds.get_mut_data(0 /* layer */).unwrap());
        println!("  Done!");
        println!("Saving lambertian_bc5.dds file");
        let mut dds_file = File::create("examples/lambertian_bc5.dds").unwrap();
        dds.write(&mut dds_file).expect("Failed to write dds file");
    }
    // BC7
    {
        let mut dds = Dds::new_dxgi(NewDxgiParams {
            format: DxgiFormat::BC7_UNorm,
            ..dds_defaults
        })
        .unwrap();
        let surface = intel_tex_2::RgbaSurface {
            width,
            height,
            stride: width * 4,
            data: &rgba_img,
        };

        println!("Compressing to BC7...");
        bc7::compress_blocks_into(
            &bc7::opaque_ultra_fast_settings(),
            &surface,
            dds.get_mut_data(0 /* layer */).unwrap(),
        );
        println!("  Done!");
        println!("Saving lambertian_bc7.dds file");
        let mut dds_file = File::create("examples/lambertian_bc7.dds").unwrap();
        dds.write(&mut dds_file).expect("Failed to write dds file");
    }*/
}
