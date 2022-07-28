#![feature(bench_black_box)]

use loop_interchange_2::{blur, Grayscale, K_MAX_IMAGE_DIMENSION};

fn main() {
    let mut image: Grayscale = Default::default();
    if !image.load("pexels-pixabay-434334.pbm", K_MAX_IMAGE_DIMENSION) {
        // Original message: An IO problem. C++ programmers lol amirite?
        panic!("Load of pexels-pixabay-434334.pbm failed");
    }

    let mut result = vec![0u8; image.size];
    let mut temp = vec![0u8; image.size];

    for _ in 0..10 {
        blur(
            result.as_mut_slice(),
            image.data.as_slice(),
            image.width,
            image.height,
            temp.as_mut_slice(),
        );

        std::hint::black_box(&mut image);
    }
}

#[cfg(test)]
mod tests {

    use loop_interchange_2::{blur, Grayscale, K_MAX_IMAGE_DIMENSION};

    #[test]
    fn validate() {
        let input = "pexels-pixabay-434334.pbm";
        let output_golden = "output-golden.pgm";

        let mut image: Grayscale = Default::default();
        if image.load(&input, K_MAX_IMAGE_DIMENSION) {
            let mut image2: Grayscale = Default::default();
            if image2.load(&output_golden, K_MAX_IMAGE_DIMENSION) {
                let mut result = vec![0u8; image.size];
                let mut temp = vec![0u8; image.size];

                blur(
                    result.as_mut_slice(),
                    image.data.as_slice(),
                    image.width,
                    image.height,
                    temp.as_mut_slice(),
                );
                //std::mem::swap(&image.data.as_mut_slice(), &result.as_mut_slice());
                image.data = result;

                assert_eq!(image.width, image2.width);
                assert_eq!(image.height, image2.height);
                assert_eq!(image.data, image2.data);

                /*
                int downcount = 10;
                for (size_t i = 0; i < image.size; i++) {
                    if (p1[i] != p2[i]) {
                    std::cerr << "Result[" << i << "] = " << static_cast<int>(p1[i])
                                << ". Expected[" << i << "] = " << static_cast<int>(p2[i])
                                << std::endl;
                    if (--downcount <= 0)
                        break;
                    }
                }
                */
            }
        }
    }
}
